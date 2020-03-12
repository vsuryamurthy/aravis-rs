// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use aravis_sys;
use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use std::fmt;
use DomNode;

glib_wrapper! {
	pub struct DomElement(Object<aravis_sys::ArvDomElement, aravis_sys::ArvDomElementClass, DomElementClass>) @extends DomNode;

	match fn {
		get_type => || aravis_sys::arv_dom_element_get_type(),
	}
}

pub const NONE_DOM_ELEMENT: Option<&DomElement> = None;

pub trait DomElementExt: 'static {
	fn get_attribute(&self, name: &str) -> Option<GString>;

	fn get_tag_name(&self) -> Option<GString>;

	fn set_attribute(&self, name: &str, attribute_value: &str);
}

impl<O: IsA<DomElement>> DomElementExt for O {
	fn get_attribute(&self, name: &str) -> Option<GString> {
		unsafe {
			from_glib_none(aravis_sys::arv_dom_element_get_attribute(
				self.as_ref().to_glib_none().0,
				name.to_glib_none().0,
			))
		}
	}

	fn get_tag_name(&self) -> Option<GString> {
		unsafe {
			from_glib_none(aravis_sys::arv_dom_element_get_tag_name(
				self.as_ref().to_glib_none().0,
			))
		}
	}

	fn set_attribute(&self, name: &str, attribute_value: &str) {
		unsafe {
			aravis_sys::arv_dom_element_set_attribute(
				self.as_ref().to_glib_none().0,
				name.to_glib_none().0,
				attribute_value.to_glib_none().0,
			);
		}
	}
}

impl fmt::Display for DomElement {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "DomElement")
	}
}
