// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use aravis_sys;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use std::fmt;
use std::ptr;

glib_wrapper! {
	pub struct GcFloat(Interface<aravis_sys::ArvGcFloat>);

	match fn {
		get_type => || aravis_sys::arv_gc_float_get_type(),
	}
}

unsafe impl Send for GcFloat {}

pub const NONE_GC_FLOAT: Option<&GcFloat> = None;

/// Trait containing all `GcFloat` methods.
///
/// # Implementors
///
/// [`GcConverterNode`](struct.GcConverterNode.html), [`GcFloatNode`](struct.GcFloatNode.html), [`GcFloatRegNode`](struct.GcFloatRegNode.html), [`GcFloat`](struct.GcFloat.html), [`GcSwissKnifeNode`](struct.GcSwissKnifeNode.html)
pub trait GcFloatExt: 'static {
	fn get_inc(&self) -> Result<f64, glib::Error>;

	fn get_max(&self) -> Result<f64, glib::Error>;

	fn get_min(&self) -> Result<f64, glib::Error>;

	fn get_unit(&self) -> Result<GString, glib::Error>;

	fn get_value(&self) -> Result<f64, glib::Error>;

	fn impose_max(&self, maximum: f64) -> Result<(), glib::Error>;

	fn impose_min(&self, minimum: f64) -> Result<(), glib::Error>;

	fn set_value(&self, value: f64) -> Result<(), glib::Error>;
}

impl<O: IsA<GcFloat>> GcFloatExt for O {
	fn get_inc(&self) -> Result<f64, glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let ret = aravis_sys::arv_gc_float_get_inc(self.as_ref().to_glib_none().0, &mut error);
			if error.is_null() {
				Ok(ret)
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn get_max(&self) -> Result<f64, glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let ret = aravis_sys::arv_gc_float_get_max(self.as_ref().to_glib_none().0, &mut error);
			if error.is_null() {
				Ok(ret)
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn get_min(&self) -> Result<f64, glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let ret = aravis_sys::arv_gc_float_get_min(self.as_ref().to_glib_none().0, &mut error);
			if error.is_null() {
				Ok(ret)
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn get_unit(&self) -> Result<GString, glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let ret = aravis_sys::arv_gc_float_get_unit(self.as_ref().to_glib_none().0, &mut error);
			if error.is_null() {
				Ok(from_glib_none(ret))
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn get_value(&self) -> Result<f64, glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let ret =
				aravis_sys::arv_gc_float_get_value(self.as_ref().to_glib_none().0, &mut error);
			if error.is_null() {
				Ok(ret)
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn impose_max(&self, maximum: f64) -> Result<(), glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let _ = aravis_sys::arv_gc_float_impose_max(
				self.as_ref().to_glib_none().0,
				maximum,
				&mut error,
			);
			if error.is_null() {
				Ok(())
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn impose_min(&self, minimum: f64) -> Result<(), glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let _ = aravis_sys::arv_gc_float_impose_min(
				self.as_ref().to_glib_none().0,
				minimum,
				&mut error,
			);
			if error.is_null() {
				Ok(())
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn set_value(&self, value: f64) -> Result<(), glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let _ = aravis_sys::arv_gc_float_set_value(
				self.as_ref().to_glib_none().0,
				value,
				&mut error,
			);
			if error.is_null() {
				Ok(())
			} else {
				Err(from_glib_full(error))
			}
		}
	}
}

impl fmt::Display for GcFloat {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "GcFloat")
	}
}
