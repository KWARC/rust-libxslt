use std::error::Error;
use std::ffi::CString;
use std::iter::once;
use std::ptr;

use crate::bindings::*;
use libxml::tree::Document;

/// An XSLT stylesheet object which can `transform` a libxml2 `Document`.
///
/// A `Stylesheet` is a compiled artifact returned by
/// [`parser::parse_file`] / [`parser::parse_bytes`] and reused for one
/// or more `transform` calls. Reuse within a single thread is the
/// documented and well-tested pattern (it is what `xsltproc`'s
/// `--maxdepth`-style loops do), and skips the cost of
/// `xsltParseStylesheetFile` on subsequent transforms.
///
/// # Thread safety
///
/// `Stylesheet` is intentionally **not** `Send` and **not** `Sync`.
/// libxslt as a whole is not documented as thread-safe, and the
/// underlying `xsltStylesheetPtr` carries fields (e.g. namespace-prefix
/// internalisation caches, error contexts) that libxslt may write to
/// during `xsltApplyStylesheetUser` without external synchronisation.
/// Until those write paths are audited and either eliminated upstream
/// or wrapped here, treat a `Stylesheet` as single-threaded.
///
/// Callers that need a per-thread reusable stylesheet should park one
/// in `thread_local!` storage; callers that need to share across
/// threads should wrap a `Stylesheet` in a `Mutex` and pay the
/// serialisation cost. We deliberately leave that policy to the caller
/// rather than baking it into this wrapper — the same caution that
/// resolved issue #6 (libxslt's hidden mutation of the input
/// `Document`) applies to the stylesheet object too.
pub struct Stylesheet {
  pub(crate) ptr: xsltStylesheetPtr,
}

struct TransformContext {
  pub(crate) ptr: xsltTransformContextPtr,
}
impl Drop for TransformContext {
  fn drop(&mut self) {
    unsafe {
      xsltFreeTransformContext(self.ptr);
    }
  }
}

impl Stylesheet {
  pub(crate) fn ptr(&self) -> xsltStylesheetPtr {
    self.ptr
  }
  pub(crate) fn ptr_mut(&mut self) -> xsltStylesheetPtr {
    self.ptr
  }

  /// Transforms a libxml `Document` per the current stylesheet.
  ///
  /// Takes `&mut self` because libxslt's apply path is not documented
  /// as read-only on the stylesheet: it may write back into the
  /// stylesheet's internalisation caches or error context fields.
  /// Sequential reuse from a single thread is sound (and skips the
  /// per-call parse cost); cross-thread sharing needs an external
  /// `Mutex`. The mirror of issue #6 applies here — the safe surface
  /// is the conservative one.
  ///
  /// The input `Document` is consumed: libxslt may mutate it while applying
  /// stylesheet-directed whitespace stripping, so handing out a shared
  /// reference would be unsound (see issue #6). `doc` is dropped — and the
  /// underlying `xmlDoc` freed — once the transform returns. If you need
  /// to transform the same source through several stylesheets, clone the
  /// `Document` at the call site.
  pub fn transform(&mut self, doc: Document, params: Vec<(&str, &str)>) -> Result<Document, Box<dyn Error>> {
    let ctxt = self.build_context(&doc)?;

    // ctxt.xinclude = 1;
    // ctxt._private = (void *) wrapper;
    // LibXSLT_init_security_prefs(ctxt);
    // LibXSLT_init_functions(ctxt, wrapper);
    // LibXSLT_init_elements(ctxt, wrapper);

    let params_cstrings_result: Result<Vec<CString>, _> = params.iter()
      .flat_map(|pair| once(pair.0).chain(once(pair.1)))
      .map(CString::new)
      .collect();

    let params_cstrings = params_cstrings_result?;

    let mut params_cstrings_pointers: Vec<*const libc::c_char> =  params_cstrings.iter()
        .map(|cstr| cstr.as_ptr())
        .collect();

    // Params array has to be null terminated
    params_cstrings_pointers.push(std::ptr::null());

    let params_ptr = params_cstrings_pointers.as_mut_ptr();

    let real_dom = unsafe {
      xsltApplyStylesheetUser(
        self.ptr_mut(),
        doc.doc_ptr(),
        params_ptr,
        ptr::null_mut(),
        ptr::null_mut(),
        ctxt.ptr,
      )
    };

    // if (doc->intSubset != NULL &&
    //     doc->prev == NULL && doc->next == NULL) {
    //    xmlNodePtr cur = (xmlNodePtr) doc->intSubset;
    //    cur->prev = dtd_prev;
    //    cur->next = dtd_next;
    //    if (dtd_prev) dtd_prev->next = cur;
    //    if (dtd_next) dtd_next->prev = cur;
    //    if (doc->children == dtd_next) doc->children = cur;
    //    if (doc->last == dtd_prev) doc->last = cur;
    // }
    // if ((real_dom != NULL) && (ctxt->state != XSLT_STATE_OK)) {
    //   /* fatal error */
    //      xmlFreeDoc(real_dom);
    //      real_dom = NULL;
    // }
    // LibXSLT_free_security_prefs(sec, ctxt);

    if real_dom.is_null() {
      // LibXSLT_report_error_ctx(saved_error,0);
      Err(From::from("Unknown error applying stylesheet"))
    }
    // if (real_dom->type == XML_HTML_DOCUMENT_NODE) {
    //     if (self->method != NULL) {
    //         xmlFree(self->method);
    //     }
    //     self->method = (xmlChar *) xmlMalloc(5);
    //     strcpy((char *) self->method, "html");
    // }
    /* non-fatal: probably just a message from the stylesheet */
    // LibXSLT_report_error_ctx(saved_error,1);
    else {
      Ok(Document::new_ptr(real_dom))
    }
  }

  fn build_context(&mut self, doc: &Document) -> Result<TransformContext, &'static str> {
    unsafe {
      let ptr = xsltNewTransformContext(self.ptr(), doc.doc_ptr());
      if ptr.is_null() {
        Err("xsltNewTransformContext returned NULL")
      } else {
        Ok(TransformContext { ptr })
      }
    }
  }
}
