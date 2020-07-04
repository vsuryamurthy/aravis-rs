// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use aravis_sys;
use glib::translate::*;
use std::fmt;
use DomElement;
use DomNode;
use GcFeatureNode;
use GcNode;

glib_wrapper! {
	pub struct GcConverter(Object<aravis_sys::ArvGcConverter, aravis_sys::ArvGcConverterClass, GcConverterClass>) @extends GcFeatureNode, GcNode, DomElement, DomNode;

	match fn {
		get_type => || aravis_sys::arv_gc_converter_get_type(),
	}
}

impl GcConverter {}

unsafe impl Send for GcConverter {}

pub const NONE_GC_CONVERTER: Option<&GcConverter> = None;

impl fmt::Display for GcConverter {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "GcConverter")
	}
}
