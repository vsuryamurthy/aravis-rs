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

glib_wrapper! {
	pub struct GcGroupNode(Object<aravis_sys::ArvGcGroupNode, aravis_sys::ArvGcGroupNodeClass, GcGroupNodeClass>) @extends GcFeatureNode, GcNode, DomElement, DomNode;

	match fn {
		get_type => || aravis_sys::arv_gc_group_node_get_type(),
	}
}

impl GcGroupNode {
	pub fn new() -> GcGroupNode {
		assert_initialized_main_thread!();
		unsafe { GcNode::from_glib_full(aravis_sys::arv_gc_group_node_new()).unsafe_cast() }
	}
}

impl Default for GcGroupNode {
	fn default() -> Self {
		Self::new()
	}
}

pub const NONE_GC_GROUP_NODE: Option<&GcGroupNode> = None;

impl fmt::Display for GcGroupNode {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "GcGroupNode")
	}
}
