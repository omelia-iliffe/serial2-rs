use std::os::windows::io::{AsRawHandle, FromRawHandle, IntoRawHandle};
use std::io::{Read, Write};
use mio::windows::NamedPipe;

use crate::Settings;

pub struct Inner {
	inner: mio::windows::NamedPipe,
}

impl Inner {
	pub fn from_blocking(inner: serial2::SerialPort) -> std::io::Result<Self> {
		// SAFETY: The handle is gauranteed to be valid by [`serial2::SerialPort`].
		unsafe {
			let inner = NamedPipe::from_raw_handle(inner.into_raw_handle());
			Ok(Self { inner })
		}
	}

	pub fn as_raw_handle(&self) -> std::os::windows::io::RawHandle {
		self.inner.as_raw_handle()
	}

	pub unsafe fn from_raw_handle(handle: std::os::windows::io::RawHandle) -> Self {
		let inner = NamedPipe::from_raw_handle(handle);
		Self { inner }
	}

	pub fn get_configuration(&self) -> std::io::Result<Settings> {
		self.as_serial_port().get_configuration()
	}

	pub fn set_configuration(&mut self, settings: &Settings) -> std::io::Result<()> {
		self.as_serial_port().set_configuration(settings)
	}

	pub fn read(&self, buf: &mut [u8]) -> std::io::Result<usize> {
		let mut inner = &self.inner;
		inner.read(buf)
	}

	pub fn read_vectored(&self, buf: &mut [std::io::IoSliceMut]) -> std::io::Result<usize> {
		let mut inner = &self.inner;
		let buf = buf.get_mut(0).ok_or(std::io::ErrorKind::InvalidInput)?;
		inner.read(buf)
	}

	pub fn is_read_vectored(&self) -> bool {
		false
	}

	pub fn write(&self, buf: &[u8]) -> std::io::Result<usize> {
		let mut inner = &self.inner;
		inner.write(buf)
	}

	pub fn write_vectored(&self, buf: &[std::io::IoSlice]) -> std::io::Result<usize> {
		let mut inner = &self.inner;
		let buf = buf.get(0).ok_or(std::io::ErrorKind::InvalidInput)?;
		inner.write(buf)
	}

	pub fn is_write_vectored(&self) -> bool {
		false
	}

	pub fn flush(&self) -> std::io::Result<()> {
		let mut inner = &self.inner;
		inner.flush()
	}

	pub fn discard_buffers(&self) -> std::io::Result<()> {
		self.as_serial_port().discard_buffers()
	}

	pub fn discard_input_buffer(&self) -> std::io::Result<()> {
		self.as_serial_port().discard_input_buffer()
	}

	pub fn discard_output_buffer(&self) -> std::io::Result<()> {
		self.as_serial_port().discard_output_buffer()
	}

	pub fn set_rts(&self, state: bool) -> std::io::Result<()> {
		self.as_serial_port().set_rts(state)
	}

	pub fn read_cts(&self) -> std::io::Result<bool> {
		self.as_serial_port().read_cts()
	}

	pub fn set_dtr(&self, state: bool) -> std::io::Result<()> {
		self.as_serial_port().set_dtr(state)
	}

	pub fn read_dsr(&self) -> std::io::Result<bool> {
		self.as_serial_port().read_dsr()
	}

	pub fn read_ri(&self) -> std::io::Result<bool> {
		self.as_serial_port().read_ri()
	}

	pub fn read_cd(&self) -> std::io::Result<bool> {
		self.as_serial_port().read_cd()
	}

	fn as_serial_port(&self) -> std::mem::ManuallyDrop<serial2::SerialPort> {
		// SAFETY: The handle is guaranteed to be valid and an actual serial port.
		unsafe {
			std::mem::ManuallyDrop::new(serial2::SerialPort::from_raw_handle(self.as_raw_handle()))
		}
	}
}

impl mio::event::Source for Inner {
	fn register(
		&mut self,
		registry: &mio::Registry,
		token: mio::Token,
		interests: mio::Interest,
	) -> std::io::Result<()> {
		self.inner.register(registry, token, interests)
	}

	fn reregister(
		&mut self,
		registry: &mio::Registry,
		token: mio::Token,
		interests: mio::Interest,
	) -> std::io::Result<()> {
		self.inner.reregister(registry, token, interests)
	}

	fn deregister(&mut self, registry: &mio::Registry) -> std::io::Result<()> {
		self.inner.deregister(registry)
	}
}
