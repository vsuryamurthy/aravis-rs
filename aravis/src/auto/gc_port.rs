// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use aravis_sys;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use DomElement;
use DomNode;
use GcFeatureNode;
use GcNode;

glib_wrapper! {
	pub struct GcPort(Object<aravis_sys::ArvGcPort, aravis_sys::ArvGcPortClass, GcPortClass>) @extends GcFeatureNode, GcNode, DomElement, DomNode;

	match fn {
		get_type => || aravis_sys::arv_gc_port_get_type(),
	}
}

impl GcPort {
	pub fn new() -> GcPort {
		assert_initialized_main_thread!();
		unsafe { GcNode::from_glib_full(aravis_sys::arv_gc_port_new()).unsafe_cast() }
	}
}

impl Default for GcPort {
	fn default() -> Self {
		Self::new()
	}
}

unsafe impl Send for GcPort {}

pub const NONE_GC_PORT: Option<&GcPort> = None;

/// Trait containing all `GcPort` methods.
///
/// # Implementors
///
/// [`GcPort`](struct.GcPort.html)
pub trait GcPortExt: 'static {
	//fn read(&self, buffer: /*Unimplemented*/Option<Fundamental: Pointer>, address: u64, length: u64) -> Result<(), glib::Error>;

	//fn write(&self, buffer: /*Unimplemented*/Option<Fundamental: Pointer>, address: u64, length: u64) -> Result<(), glib::Error>;
}

impl<O: IsA<GcPort>> GcPortExt for O {
	//fn read(&self, buffer: /*Unimplemented*/Option<Fundamental: Pointer>, address: u64, length: u64) -> Result<(), glib::Error> {
	//    unsafe { TODO: call aravis_sys:arv_gc_port_read() }
	//}

	//fn write(&self, buffer: /*Unimplemented*/Option<Fundamental: Pointer>, address: u64, length: u64) -> Result<(), glib::Error> {
	//    unsafe { TODO: call aravis_sys:arv_gc_port_write() }
	//}
}

impl fmt::Display for GcPort {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "GcPort")
	}
}
