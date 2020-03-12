// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use aravis_sys;
use glib::translate::*;
use std::fmt;
use DomNode;

glib_wrapper! {
	pub struct DomDocumentFragment(Object<aravis_sys::ArvDomDocumentFragment, aravis_sys::ArvDomDocumentFragmentClass, DomDocumentFragmentClass>) @extends DomNode;

	match fn {
		get_type => || aravis_sys::arv_dom_document_fragment_get_type(),
	}
}

impl DomDocumentFragment {
	pub fn new() -> DomDocumentFragment {
		assert_initialized_main_thread!();
		unsafe { from_glib_full(aravis_sys::arv_dom_document_fragment_new()) }
	}
}

impl Default for DomDocumentFragment {
	fn default() -> Self {
		Self::new()
	}
}

pub const NONE_DOM_DOCUMENT_FRAGMENT: Option<&DomDocumentFragment> = None;

impl fmt::Display for DomDocumentFragment {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "DomDocumentFragment")
	}
}
