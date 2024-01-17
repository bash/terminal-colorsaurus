use libc::{grantpt, unlockpt, TIOCSCTTY};
use std::ffi::{c_int, CStr, CString, OsStr};
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, OwnedFd, RawFd};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};
use std::str::from_utf8;
use std::{io, ptr};

#[test]
fn pty() {
    let mut controller = openpty().unwrap();
    to_io_result(unsafe { grantpt(controller.as_raw_fd()) }).unwrap();
    to_io_result(unsafe { unlockpt(controller.as_raw_fd()) }).unwrap();

    let pts = ptsname(controller.as_fd()).unwrap();
    let user = OpenOptions::new()
        .read(true)
        .write(true)
        .open(OsStr::from_bytes(pts.as_bytes()))
        .unwrap();

    // let user =

    let mut builder = Command::new("yes");
    builder
        .stdin(unsafe { Stdio::from_raw_fd(user.as_raw_fd()) })
        .stdout(unsafe { Stdio::from_raw_fd(user.as_raw_fd()) })
        .stderr(unsafe { Stdio::from_raw_fd(user.as_raw_fd()) });
    unsafe {
        let controller = controller.as_raw_fd();
        let user = user.as_raw_fd();
        builder.pre_exec(move || {
            to_io_result(libc::setsid())?;
            set_controlling_terminal(user)?;
            libc::close(user);
            libc::close(controller);
            Ok(())
        });
    }

    let mut child = builder.spawn().unwrap();
    // child.wait().unwrap();

    let mut buf = vec![0; 100];
    let bytes_read = controller.read(&mut buf).unwrap();
    dbg!(bytes_read);
    dbg!(from_utf8(&buf[..bytes_read]).unwrap());

    // dbg!(&controller);
    // dbg!(&user);
    panic!();
}

fn openpty() -> io::Result<File> {
    // O_RDWR:
    //   Open the device for both reading and writing.
    // O_NOCTTY:
    //   Do not make this device the controlling terminal for the process.
    let fd = unsafe { libc::posix_openpt(libc::O_RDWR | libc::O_NOCTTY) };
    if fd == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(unsafe { File::from_raw_fd(fd) })
    }
}

fn ptsname(fd: BorrowedFd<'_>) -> io::Result<CString> {
    // Yeah, yeah this means that I only support very small strings. But I don't care for my tests :)
    let mut buf = vec![0; 256];
    let code = unsafe { libc::ptsname_r(fd.as_raw_fd(), buf.as_mut_ptr().cast(), buf.len()) };
    if code == 0 {
        Ok(unsafe { CStr::from_ptr(buf.as_ptr()).to_owned() })
    } else {
        Err(io::Error::from_raw_os_error(code))
    }
}

fn to_io_result(value: c_int) -> io::Result<c_int> {
    if value == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(value)
    }
}

/// Really only needed on BSD, but should be fine elsewhere.
fn set_controlling_terminal(fd: c_int) -> io::Result<()> {
    let res = unsafe {
        // TIOSCTTY changes based on platform and the `ioctl` call is different
        // based on architecture (32/64). So a generic cast is used to make sure
        // there are no issues. To allow such a generic cast the clippy warning
        // is disabled.
        #[allow(clippy::cast_lossless)]
        libc::ioctl(fd, TIOCSCTTY as _, 0)
    };
    to_io_result(res)?;
    Ok(())
}
