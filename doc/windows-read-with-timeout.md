# Waiting for Console Input with a Timeout
This can be achieved using `WaitForSingleObject`.

```rust
use crate::{Error, Result};
use std::io;
use std::os::windows::io::AsRawHandle;
use std::time::Duration;
use terminal_trx::Transceive;
use windows_sys::Win32::Foundation::{WAIT_ABANDONED, WAIT_OBJECT_0, WAIT_TIMEOUT};
use windows_sys::Win32::System::Threading::WaitForSingleObject;

pub(crate) fn poll_read(terminal: &dyn Transceive, timeout: Duration) -> Result<()> {
    let handle = terminal.input_buffer_handle();
    match unsafe {
        WaitForSingleObject(handle.as_raw_handle() as isize, timeout.as_millis() as u32)
    } {
        // The state of the specified object is signaled.
        WAIT_OBJECT_0 => Ok(()),
        WAIT_ABANDONED | WAIT_TIMEOUT => Err(Error::Timeout(timeout)),
        _ => Err(Error::Io(io::Error::last_os_error())),
    }
}
```