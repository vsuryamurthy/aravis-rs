// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use aravis_sys;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;
use DomElement;
use DomNode;
use GcFeatureNode;
use GcNode;
use GcRegister;

glib_wrapper! {
	pub struct GcRegisterNode(Object<aravis_sys::ArvGcRegisterNode, aravis_sys::ArvGcRegisterNodeClass, GcRegisterNodeClass>) @extends GcFeatureNode, GcNode, DomElement, DomNode, @implements GcRegister;

	match fn {
		get_type => || aravis_sys::arv_gc_register_node_get_type(),
	}
}

impl GcRegisterNode {
	pub fn new() -> GcRegisterNode {
		assert_initialized_main_thread!();
		unsafe { GcNode::from_glib_full(aravis_sys::arv_gc_register_node_new()).unsafe_cast() }
	}
}

impl Default for GcRegisterNode {
	fn default() -> Self {
		Self::new()
	}
}

pub const NONE_GC_REGISTER_NODE: Option<&GcRegisterNode> = None;

impl fmt::Display for GcRegisterNode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "GcRegisterNode")
	}
}
