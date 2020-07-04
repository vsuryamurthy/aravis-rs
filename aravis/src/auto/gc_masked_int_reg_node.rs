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
use GcInteger;
use GcNode;
use GcRegister;
use GcRegisterNode;
use GcSelector;

glib_wrapper! {
	pub struct GcMaskedIntRegNode(Object<aravis_sys::ArvGcMaskedIntRegNode, aravis_sys::ArvGcMaskedIntRegNodeClass, GcMaskedIntRegNodeClass>) @extends GcRegisterNode, GcFeatureNode, GcNode, DomElement, DomNode, @implements GcRegister, GcInteger, GcSelector;

	match fn {
		get_type => || aravis_sys::arv_gc_masked_int_reg_node_get_type(),
	}
}

impl GcMaskedIntRegNode {
	///
	/// # Returns
	///
	/// a new MaskedIntReg node
	pub fn new() -> GcMaskedIntRegNode {
		assert_initialized_main_thread!();
		unsafe {
			GcNode::from_glib_full(aravis_sys::arv_gc_masked_int_reg_node_new()).unsafe_cast()
		}
	}
}

impl Default for GcMaskedIntRegNode {
	fn default() -> Self {
		Self::new()
	}
}

unsafe impl Send for GcMaskedIntRegNode {}

pub const NONE_GC_MASKED_INT_REG_NODE: Option<&GcMaskedIntRegNode> = None;

impl fmt::Display for GcMaskedIntRegNode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "GcMaskedIntRegNode")
	}
}
