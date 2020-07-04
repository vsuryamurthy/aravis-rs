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
use Device;

glib_wrapper! {
	pub struct Interface(Object<aravis_sys::ArvInterface, aravis_sys::ArvInterfaceClass, InterfaceClass>);

	match fn {
		get_type => || aravis_sys::arv_interface_get_type(),
	}
}

unsafe impl Send for Interface {}

pub const NONE_INTERFACE: Option<&Interface> = None;

/// Trait containing all `Interface` methods.
///
/// # Implementors
///
/// [`FakeInterface`](struct.FakeInterface.html), [`GvInterface`](struct.GvInterface.html), [`Interface`](struct.Interface.html), [`UvInterface`](struct.UvInterface.html)
pub trait InterfaceExt: 'static {
	/// queries the device address (IP address in the case of an ethernet camera). Useful
	/// for constructing manual connections to devices using `GvDevice::new`
	///
	/// Prior to this call the `InterfaceExt::update_device_list`
	/// function must be called.
	/// ## `index`
	/// device index
	///
	/// # Returns
	///
	/// the device address
	fn get_device_address(&self, index: u32) -> Option<GString>;

	/// Queries the unique device id corresponding to index. Prior to this
	/// call the `InterfaceExt::update_device_list` function must be called.
	/// ## `index`
	/// device index
	///
	/// # Returns
	///
	/// a unique device id
	fn get_device_id(&self, index: u32) -> Option<GString>;

	/// Queries the device model.
	///
	/// Prior to this call the `InterfaceExt::update_device_list`
	/// function must be called.
	/// ## `index`
	/// device index
	///
	/// # Returns
	///
	/// the device model, NULL on error
	fn get_device_model(&self, index: u32) -> Option<GString>;

	/// Queries the physical device id corresponding to index such
	/// as the MAC address for Ethernet based devices, bus id for PCI,
	/// USB or Firewire based devices.
	///
	/// Prior to this call the `InterfaceExt::update_device_list`
	/// function must be called.
	/// ## `index`
	/// device index
	///
	/// # Returns
	///
	/// a physical device id
	fn get_device_physical_id(&self, index: u32) -> Option<GString>;

	/// Queries the device protocol. Possible values are 'USB3Vision', 'GigEVision'
	/// and 'Fake'.
	///
	/// Prior to this call the `InterfaceExt::update_device_list`
	/// function must be called.
	/// ## `index`
	/// device index
	///
	/// # Returns
	///
	/// the device protocol as a string, NULL on error
	fn get_device_protocol(&self, index: u32) -> Option<GString>;

	/// Queries the device serial.
	///
	/// Prior to this call the `InterfaceExt::update_device_list`
	/// function must be called.
	/// ## `index`
	/// device index
	///
	/// # Returns
	///
	/// the device serial, NULL on error
	fn get_device_serial_nbr(&self, index: u32) -> Option<GString>;

	/// Queries the device vendor.
	///
	/// Prior to this call the `InterfaceExt::update_device_list`
	/// function must be called.
	/// ## `index`
	/// device index
	///
	/// # Returns
	///
	/// the device vendor, NULL on error
	fn get_device_vendor(&self, index: u32) -> Option<GString>;

	/// Queries the number of available devices on this interface. Prior to this
	/// call the `InterfaceExt::update_device_list` function must be called. The list content will not
	/// change until the next call of the update function.
	///
	/// # Returns
	///
	/// the number of available devices
	fn get_n_devices(&self) -> u32;

	/// Creates a new `Device` object corresponding to the given device id string.
	/// The first available device is returned if `device_id` is `None`.
	/// ## `device_id`
	/// device unique id
	///
	/// # Returns
	///
	/// a new `Device`
	fn open_device(&self, device_id: Option<&str>) -> Result<Device, glib::Error>;

	/// Updates the internal list of available devices. This may change the
	/// connection between a list index and a device ID.
	fn update_device_list(&self);
}

impl<O: IsA<Interface>> InterfaceExt for O {
	fn get_device_address(&self, index: u32) -> Option<GString> {
		unsafe {
			from_glib_none(aravis_sys::arv_interface_get_device_address(
				self.as_ref().to_glib_none().0,
				index,
			))
		}
	}

	fn get_device_id(&self, index: u32) -> Option<GString> {
		unsafe {
			from_glib_none(aravis_sys::arv_interface_get_device_id(
				self.as_ref().to_glib_none().0,
				index,
			))
		}
	}

	fn get_device_model(&self, index: u32) -> Option<GString> {
		unsafe {
			from_glib_none(aravis_sys::arv_interface_get_device_model(
				self.as_ref().to_glib_none().0,
				index,
			))
		}
	}

	fn get_device_physical_id(&self, index: u32) -> Option<GString> {
		unsafe {
			from_glib_none(aravis_sys::arv_interface_get_device_physical_id(
				self.as_ref().to_glib_none().0,
				index,
			))
		}
	}

	fn get_device_protocol(&self, index: u32) -> Option<GString> {
		unsafe {
			from_glib_none(aravis_sys::arv_interface_get_device_protocol(
				self.as_ref().to_glib_none().0,
				index,
			))
		}
	}

	fn get_device_serial_nbr(&self, index: u32) -> Option<GString> {
		unsafe {
			from_glib_none(aravis_sys::arv_interface_get_device_serial_nbr(
				self.as_ref().to_glib_none().0,
				index,
			))
		}
	}

	fn get_device_vendor(&self, index: u32) -> Option<GString> {
		unsafe {
			from_glib_none(aravis_sys::arv_interface_get_device_vendor(
				self.as_ref().to_glib_none().0,
				index,
			))
		}
	}

	fn get_n_devices(&self) -> u32 {
		unsafe { aravis_sys::arv_interface_get_n_devices(self.as_ref().to_glib_none().0) }
	}

	fn open_device(&self, device_id: Option<&str>) -> Result<Device, glib::Error> {
		unsafe {
			let mut error = ptr::null_mut();
			let ret = aravis_sys::arv_interface_open_device(
				self.as_ref().to_glib_none().0,
				device_id.to_glib_none().0,
				&mut error,
			);
			if error.is_null() {
				Ok(from_glib_full(ret))
			} else {
				Err(from_glib_full(error))
			}
		}
	}

	fn update_device_list(&self) {
		unsafe {
			aravis_sys::arv_interface_update_device_list(self.as_ref().to_glib_none().0);
		}
	}
}

impl fmt::Display for Interface {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Interface")
	}
}
