// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use aravis_sys;
use glib::translate::*;
use std::fmt;
use Interface;

glib_wrapper! {
	pub struct GvInterface(Object<aravis_sys::ArvGvInterface, aravis_sys::ArvGvInterfaceClass, GvInterfaceClass>) @extends Interface;

	match fn {
		get_type => || aravis_sys::arv_gv_interface_get_type(),
	}
}

impl GvInterface {
	/// Gets the unique instance of the GV interface.
	///
	/// # Returns
	///
	/// a `Interface` singleton.
	pub fn get_instance() -> Option<Interface> {
		assert_initialized_main_thread!();
		unsafe { from_glib_none(aravis_sys::arv_gv_interface_get_instance()) }
	}
}

unsafe impl Send for GvInterface {}

pub const NONE_GV_INTERFACE: Option<&GvInterface> = None;

impl fmt::Display for GvInterface {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "GvInterface")
	}
}
