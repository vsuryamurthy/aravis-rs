// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::Device;
#[cfg(any(feature = "v0_8_17", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8_17")))]
use crate::UvUsbMode;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib::wrapper! {
	#[doc(alias = "ArvUvDevice")]
	pub struct UvDevice(Object<ffi::ArvUvDevice, ffi::ArvUvDeviceClass>) @extends Device;

	match fn {
		type_ => || ffi::arv_uv_device_get_type(),
	}
}

impl UvDevice {
	/// ## `vendor`
	/// USB3 vendor string
	/// ## `product`
	/// USB3 product string
	/// ## `serial_number`
	/// device serial number
	///
	/// # Returns
	///
	/// a newly created [`Device`][crate::Device] using USB3 based protocol
	#[doc(alias = "arv_uv_device_new")]
	pub fn new(vendor: &str, product: &str, serial_number: &str) -> Result<UvDevice, glib::Error> {
		assert_initialized_main_thread!();
		unsafe {
			let mut error = ptr::null_mut();
			let ret = ffi::arv_uv_device_new(
				vendor.to_glib_none().0,
				product.to_glib_none().0,
				serial_number.to_glib_none().0,
				&mut error,
			);
			if error.is_null() {
				Ok(Device::from_glib_full(ret).unsafe_cast())
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	/// ## `guid`
	/// device GUID
	///
	/// # Returns
	///
	/// a newly created [`Device`][crate::Device] using USB3 based protocol
	#[cfg(any(feature = "v0_8_17", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8_17")))]
	#[doc(alias = "arv_uv_device_new_from_guid")]
	#[doc(alias = "new_from_guid")]
	pub fn from_guid(guid: &str) -> Result<UvDevice, glib::Error> {
		assert_initialized_main_thread!();
		unsafe {
			let mut error = ptr::null_mut();
			let ret = ffi::arv_uv_device_new_from_guid(guid.to_glib_none().0, &mut error);
			if error.is_null() {
				Ok(Device::from_glib_full(ret).unsafe_cast())
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	/// Sets the option to utilize the USB synchronous or asynchronous device I/O API. The default mode is
	/// [`UvUsbMode::Sync`][crate::UvUsbMode::Sync], which means USB bulk transfer will be synchronously executed. This mode is qualified to work,
	/// but it has the performance issue with some high framerate device. Using [`UvUsbMode::Async`][crate::UvUsbMode::Async] possibly improves the
	/// bandwidth.
	/// ## `usb_mode`
	/// a [`UvUsbMode`][crate::UvUsbMode] option
	#[cfg(any(feature = "v0_8_17", feature = "dox"))]
	#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_8_17")))]
	#[doc(alias = "arv_uv_device_set_usb_mode")]
	pub fn set_usb_mode(&self, usb_mode: UvUsbMode) {
		unsafe {
			ffi::arv_uv_device_set_usb_mode(self.to_glib_none().0, usb_mode.into_glib());
		}
	}
}

unsafe impl Send for UvDevice {}

impl fmt::Display for UvDevice {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("UvDevice")
	}
}
