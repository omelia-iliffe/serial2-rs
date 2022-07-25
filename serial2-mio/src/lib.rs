#![warn(missing_docs)]

//! Non-blocking serial port for [`mio`], based on [`serial2`].

use std::path::{Path, PathBuf};

pub use serial2;
pub use serial2::{
	COMMON_BAUD_RATES,
	CharSize,
	FlowControl,
	IntoSettings,
	KeepSettings,
	Parity,
	Settings,
	StopBits,
};

mod sys;

/// Non-blocking serial port, compatible with [`mio`].
pub struct SerialPort {
	inner: sys::Inner,
}

impl SerialPort {
	/// Open and configure a serial port by path or name.
	///
	/// On Unix systems, the `name` parameter must be a path to a TTY device.
	/// On Windows, it must be the name of a COM device, such as COM1, COM2, etc.
	///
	/// The second argument is used to configure the serial port.
	/// For simple cases, you pass a `u32` for the baud rate.
	/// See [`IntoSettings`] for more information.
	///
	/// # Example
	/// ```no_run
	/// # use serial2_mio::SerialPort;
	/// # fn main() -> std::io::Result<()> {
	/// SerialPort::open("/dev/ttyUSB0", 115200)?;
	/// #   Ok(())
	/// # }
	/// ```
	pub fn open(path: impl AsRef<Path>, settings: impl IntoSettings) -> std::io::Result<Self> {
		let inner = serial2::SerialPort::open(path, settings)?;
		Self::try_from(inner)
	}

	/// Get a list of available serial ports.
	///
	/// Not currently supported on all platforms.
	/// On unsupported platforms, this function always returns an error.
	pub fn available_ports() -> std::io::Result<Vec<PathBuf>> {
		serial2::SerialPort::available_ports()
	}

	/// Get the current configuration of the serial port.
	///
	/// This function can fail if the underlying syscall fails,
	/// or if the serial port configuration can't be reported using [`Settings`].
	pub fn get_configuration(&self) -> std::io::Result<Settings> {
		self.inner.get_configuration()
	}

	/// Configure (or reconfigure) the serial port.
	pub fn set_configuration(&mut self, settings: &Settings) -> std::io::Result<()> {
		self.inner.set_configuration(settings)
	}

	/// Read bytes from the serial port.
	///
	/// This is identical to [`std::io::Read::read()`], except that this function takes a const reference `&self`.
	/// This allows you to use the serial port concurrently from multiple threads.
	///
	/// Note that there are no guarantees on which thread receives what data when multiple threads are reading from the serial port.
	/// You should normally limit yourself to a single reading thread and a single writing thread.
	pub fn read(&self, buf: &mut [u8]) -> std::io::Result<usize> {
		self.inner.read(buf)
	}

	/// Read bytes from the serial port into a slice of buffers.
	///
	/// This is identical to [`std::io::Read::read_vectored()`], except that this function takes a const reference `&self`.
	/// This allows you to use the serial port concurrently from multiple threads.
	///
	/// Note that there are no guarantees on which thread receives what data when multiple threads are reading from the serial port.
	/// You should normally limit yourself to a single reading thread and a single writing thread.
	pub fn read_vectored(&self, buf: &mut [std::io::IoSliceMut]) -> std::io::Result<usize> {
		self.inner.read_vectored(buf)
	}

	/// Check if the implementation supports vectored reads.
	///
	/// If this returns false, then [`Self::read_vectored()`] will only use the first buffer of the given slice.
	/// All platforms except for Windows support vectored reads.
	pub fn is_read_vectored(&self) -> bool {
		self.inner.is_read_vectored()
	}

	/// Write bytes to the serial port.
	///
	/// This is identical to [`std::io::Write::write()`], except that this function takes a const reference `&self`.
	/// This allows you to use the serial port concurrently from multiple threads.
	///
	/// Note that data written to the same serial port from multiple threads may end up interleaved at the receiving side.
	/// You should normally limit yourself to a single reading thread and a single writing thread.
	pub fn write(&self, buf: &[u8]) -> std::io::Result<usize> {
		self.inner.write(buf)
	}

	/// Write bytes to the serial port from a slice of buffers.
	///
	/// This is identical to [`std::io::Write::write_vectored()`], except that this function takes a const reference `&self`.
	/// This allows you to use the serial port concurrently from multiple threads.
	///
	/// Note that data written to the same serial port from multiple threads may end up interleaved at the receiving side.
	/// You should normally limit yourself to a single reading thread and a single writing thread.
	pub fn write_vectored(&self, buf: &[std::io::IoSlice]) -> std::io::Result<usize> {
		self.inner.write_vectored(buf)
	}

	/// Check if the implementation supports vectored writes.
	///
	/// If this returns false, then [`Self::write_vectored()`] will only use the first buffer of the given slice.
	/// All platforms except for Windows support vectored writes.
	pub fn is_write_vectored(&self) -> bool {
		self.inner.is_write_vectored()
	}

	/// Flush all data queued to be written.
	///
	/// This will block until the OS buffer has been fully transmitted.
	///
	/// This is identical to [`std::io::Write::flush()`], except that this function takes a const reference `&self`.
	pub fn flush(&self) -> std::io::Result<()> {
		self.inner.flush()
	}

	/// Discard the kernel input and output buffers for the serial port.
	///
	/// When you write to a serial port, the data may be put in a buffer by the OS to be transmitted by the actual device later.
	/// Similarly, data received on the device can be put in a buffer by the OS untill you read it.
	/// This function clears both buffers: any untransmitted data and received but unread data is discarded by the OS.
	pub fn discard_buffers(&self) -> std::io::Result<()> {
		self.inner.discard_buffers()
	}

	/// Discard the kernel input buffers for the serial port.
	///
	/// Data received on the device can be put in a buffer by the OS untill you read it.
	/// This function clears that buffer: received but unread data is discarded by the OS.
	///
	/// This is particularly useful when communicating with a device that only responds to commands that you send to it.
	/// If you discard the input buffer before sending the command, you discard any noise that may have been received after the last command.
	pub fn discard_input_buffer(&self) -> std::io::Result<()> {
		self.inner.discard_input_buffer()
	}

	/// Discard the kernel output buffers for the serial port.
	///
	/// When you write to a serial port, the data is generally put in a buffer by the OS to be transmitted by the actual device later.
	/// This function clears that buffer: any untransmitted data is discarded by the OS.
	pub fn discard_output_buffer(&self) -> std::io::Result<()> {
		self.inner.discard_output_buffer()
	}

	/// Set the state of the Ready To Send line.
	///
	/// If hardware flow control is enabled on the serial port, it is platform specific what will happen.
	/// The function may fail with an error or it may silently be ignored.
	/// It may even succeed and interfere with the flow control.
	pub fn set_rts(&self, state: bool) -> std::io::Result<()> {
		self.inner.set_rts(state)
	}

	/// Read the state of the Clear To Send line.
	///
	/// If hardware flow control is enabled on the serial port, it is platform specific what will happen.
	/// The function may fail with an error, it may return a bogus value, or it may return the actual state of the CTS line.
	pub fn read_cts(&self) -> std::io::Result<bool> {
		self.inner.read_cts()
	}

	/// Set the state of the Data Terminal Ready line.
	///
	/// If hardware flow control is enabled on the serial port, it is platform specific what will happen.
	/// The function may fail with an error or it may silently be ignored.
	pub fn set_dtr(&self, state: bool) -> std::io::Result<()> {
		self.inner.set_dtr(state)
	}

	/// Read the state of the Data Set Ready line.
	///
	/// If hardware flow control is enabled on the serial port, it is platform specific what will happen.
	/// The function may fail with an error, it may return a bogus value, or it may return the actual state of the DSR line.
	pub fn read_dsr(&self) -> std::io::Result<bool> {
		self.inner.read_dsr()
	}

	/// Read the state of the Ring Indicator line.
	///
	/// This line is also sometimes also called the RNG or RING line.
	pub fn read_ri(&self) -> std::io::Result<bool> {
		self.inner.read_ri()
	}

	/// Read the state of the Carrier Detect (CD) line.
	///
	/// This line is also called the Data Carrier Detect (DCD) line
	/// or the Receive Line Signal Detect (RLSD) line.
	pub fn read_cd(&self) -> std::io::Result<bool> {
		self.inner.read_cd()
	}
}

impl TryFrom<serial2::SerialPort> for SerialPort {
	type Error = std::io::Error;

	fn try_from(other: serial2::SerialPort) -> Result<Self, Self::Error> {
		Ok(Self {
			inner: sys::Inner::from_blocking(other)?,
		})
	}
}

impl mio::event::Source for SerialPort {
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

#[cfg(unix)]
impl std::os::unix::io::AsRawFd for SerialPort {
	fn as_raw_fd(&self) -> std::os::unix::io::RawFd {
		self.inner.as_raw_fd()
	}
}

#[cfg(unix)]
impl std::os::unix::io::IntoRawFd for SerialPort {
	fn into_raw_fd(self) -> std::os::unix::prelude::RawFd {
		self.inner.into_raw_fd()
	}
}

#[cfg(unix)]
impl std::os::unix::io::FromRawFd for SerialPort {
	unsafe fn from_raw_fd(fd: std::os::unix::io::RawFd) -> Self {
		Self {
			inner: sys::Inner::from_raw_fd(fd),
		}
	}
}

#[cfg(windows)]
impl std::os::windows::io::AsRawHandle for SerialPort {
	fn as_raw_handle(&self) -> std::os::windows::io::RawHandle {
		self.inner.as_raw_handle()
	}
}

#[cfg(windows)]
impl std::os::windows::io::FromRawHandle for SerialPort {
	unsafe fn from_raw_handle(handle: std::os::windows::io::RawHandle) -> Self {
		Self {
			inner: sys::Inner::from_raw_handle(handle),
		}
	}
}
