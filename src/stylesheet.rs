use std::error::Error;
use std::ptr;

use crate::bindings::*;
use libxml::tree::Document;

/// An XSLT stylesheet object which can `transform` a libxml2 `Document`
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

  /// Transforms a libxml `Document` per the current stylesheet
  pub fn transform(&mut self, doc: &Document) -> Result<Document, Box<Error>> {
    let ctxt = self.build_context(doc)?;

    // ctxt.xinclude = 1;
    // ctxt._private = (void *) wrapper;
    // LibXSLT_init_security_prefs(ctxt);
    // LibXSLT_init_functions(ctxt, wrapper);
    // LibXSLT_init_elements(ctxt, wrapper);
    let xslt_params = ptr::null_mut();

    let real_dom = unsafe {
      xsltApplyStylesheetUser(
        self.ptr_mut(),
        doc.doc_ptr(),
        xslt_params,
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
