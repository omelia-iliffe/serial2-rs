use mio::unix::SourceFd;
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd};

use crate::Settings;

pub struct Inner {
	inner: serial2::SerialPort,
}

impl Inner {
	pub fn from_blocking(inner: serial2::SerialPort) -> std::io::Result<Self> {
		// SAFETY: The managed file descriptor is guaranteed to be valid by the [`serial2::SerialPort`] type.
		unsafe {
			let flags = check_i32(libc::fcntl(inner.as_raw_fd(), libc::F_GETFL))?;
			check_i32(libc::fcntl(inner.as_raw_fd(), libc::F_SETFL, flags | libc::O_NONBLOCK))?;
		}
		Ok(Self { inner })
	}

	pub fn as_raw_fd(&self) -> std::os::unix::io::RawFd {
		self.inner.as_raw_fd()
	}

	pub fn into_raw_fd(self) -> std::os::unix::io::RawFd {
		self.inner.into_raw_fd()
	}

	pub unsafe fn from_raw_fd(fd: std::os::unix::io::RawFd) -> Self {
		Self {
			inner: serial2::SerialPort::from_raw_fd(fd),
		}
	}

	pub fn get_configuration(&self) -> std::io::Result<Settings> {
		self.inner.get_configuration()
	}

	pub fn set_configuration(&mut self, settings: &Settings) -> std::io::Result<()> {
		self.inner.set_configuration(settings)
	}

	pub fn read(&self, buf: &mut [u8]) -> std::io::Result<usize> {
		self.inner.read(buf)
	}

	pub fn read_vectored(&self, buf: &mut [std::io::IoSliceMut]) -> std::io::Result<usize> {
		self.inner.read_vectored(buf)
	}

	pub fn is_read_vectored(&self) -> bool {
		self.inner.is_read_vectored()
	}

	pub fn write(&self, buf: &[u8]) -> std::io::Result<usize> {
		self.inner.write(buf)
	}

	pub fn write_vectored(&self, buf: &[std::io::IoSlice]) -> std::io::Result<usize> {
		self.inner.write_vectored(buf)
	}

	pub fn is_write_vectored(&self) -> bool {
		self.inner.is_write_vectored()
	}

	pub fn flush(&self) -> std::io::Result<()> {
		self.inner.flush()
	}

	pub fn discard_buffers(&self) -> std::io::Result<()> {
		self.inner.discard_buffers()
	}

	pub fn discard_input_buffer(&self) -> std::io::Result<()> {
		self.inner.discard_input_buffer()
	}

	pub fn discard_output_buffer(&self) -> std::io::Result<()> {
		self.inner.discard_output_buffer()
	}

	pub fn set_rts(&self, state: bool) -> std::io::Result<()> {
		self.inner.set_rts(state)
	}

	pub fn read_cts(&self) -> std::io::Result<bool> {
		self.inner.read_cts()
	}

	pub fn set_dtr(&self, state: bool) -> std::io::Result<()> {
		self.inner.set_dtr(state)
	}

	pub fn read_dsr(&self) -> std::io::Result<bool> {
		self.inner.read_dsr()
	}

	pub fn read_ri(&self) -> std::io::Result<bool> {
		self.inner.read_ri()
	}

	pub fn read_cd(&self) -> std::io::Result<bool> {
		self.inner.read_cd()
	}
}

fn check_i32(ret: i32) -> std::io::Result<i32> {
	if ret == -1 {
		Err(std::io::Error::last_os_error())
	} else {
		Ok(ret)
	}
}

impl mio::event::Source for Inner {
	fn register(
		&mut self,
		registry: &mio::Registry,
		token: mio::Token,
		interests: mio::Interest,
	) -> std::io::Result<()> {
		SourceFd(&self.inner.as_raw_fd()).register(registry, token, interests)
	}

	fn reregister(
		&mut self,
		registry: &mio::Registry,
		token: mio::Token,
		interests: mio::Interest,
	) -> std::io::Result<()> {
		SourceFd(&self.inner.as_raw_fd()).reregister(registry, token, interests)
	}

	fn deregister(&mut self, registry: &mio::Registry) -> std::io::Result<()> {
		SourceFd(&self.inner.as_raw_fd()).deregister(registry)
	}
}
