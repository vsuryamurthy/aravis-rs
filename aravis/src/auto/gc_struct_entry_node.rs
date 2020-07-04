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

glib_wrapper! {
	pub struct GcStructEntryNode(Object<aravis_sys::ArvGcStructEntryNode, aravis_sys::ArvGcStructEntryNodeClass, GcStructEntryNodeClass>) @extends GcFeatureNode, GcNode, DomElement, DomNode, @implements GcInteger, GcRegister;

	match fn {
		get_type => || aravis_sys::arv_gc_struct_entry_node_get_type(),
	}
}

impl GcStructEntryNode {
	///
	/// # Returns
	///
	/// a newly created `GcStructEntryNode`.
	pub fn new() -> GcStructEntryNode {
		assert_initialized_main_thread!();
		unsafe { GcNode::from_glib_full(aravis_sys::arv_gc_struct_entry_node_new()).unsafe_cast() }
	}
}

impl Default for GcStructEntryNode {
	fn default() -> Self {
		Self::new()
	}
}

unsafe impl Send for GcStructEntryNode {}

pub const NONE_GC_STRUCT_ENTRY_NODE: Option<&GcStructEntryNode> = None;

impl fmt::Display for GcStructEntryNode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "GcStructEntryNode")
	}
}
