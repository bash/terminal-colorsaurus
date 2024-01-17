use libc::{grantpt, unlockpt, TIOCSCTTY};
use std::ffi::{c_int, CStr, CString, OsStr};
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Read, Write};
use std::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::process::CommandExt;
use std::process::Command;

const FG_EXE: &str = env!("CARGO_BIN_EXE_fg");

#[test]
fn pty() {
    let mut controller = openpty().unwrap();

    let pts = ptsname_r(controller.as_fd()).unwrap();
    let user = OpenOptions::new()
        .read(true)
        .write(true)
        .open(OsStr::from_bytes(pts.as_bytes()))
        .unwrap();

    let mut builder = Command::new(FG_EXE);
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
    builder
        .env_clear()
        .stdin(user.try_clone().unwrap())
        .stdout(user.try_clone().unwrap())
        .stderr(user);

    let mut child = builder.spawn().unwrap();
    drop(builder);
    // drop(user);
    let mut buf = [0; 1];

    while buf[0] != b'\x1b' {
        controller.read(&mut buf).unwrap();
    }
    while buf[0] != b'c' {
        controller.read(&mut buf).unwrap();
    }

    controller.write_all(b"\x1b[?1;2c").unwrap();
    while buf[0] != b'\x1b' {
        controller.read(&mut buf).unwrap();
    }
    while buf[0] != b'\x07' {
        controller.read(&mut buf).unwrap();
    }

    controller
        .write_all(b"\x1b]11;rgb:dcaa/dcab/dcaa\x07")
        .unwrap();

    let mut buf = String::new();
    controller.read_to_string(&mut buf).unwrap();

    assert_eq!("Color { red: 56490, green: 56491, blue: 56490 }\r\n", buf);
    assert!(child.wait().unwrap().success());
}

fn openpty() -> io::Result<File> {
    // O_RDWR:
    //   Open the device for both reading and writing.
    // O_NOCTTY:
    //   Do not make this device the controlling terminal for the process.
    let fd = to_io_result(unsafe { libc::posix_openpt(libc::O_RDWR | libc::O_NOCTTY) })?;
    to_io_result(unsafe { grantpt(fd) })?;
    to_io_result(unsafe { unlockpt(fd) })?;
    Ok(unsafe { File::from_raw_fd(fd) })
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

#[cfg(not(target_os = "macos"))]
fn ptsname_r(fd: BorrowedFd<'_>) -> io::Result<CString> {
    // Yeah, yeah this means that I only support very small strings. But I don't care for my tests :)
    let mut buf = vec![0; 256];
    let code = unsafe { libc::ptsname_r(fd.as_raw_fd(), buf.as_mut_ptr().cast(), buf.len()) };
    if code == 0 {
        Ok(unsafe { CStr::from_ptr(buf.as_ptr()).to_owned() })
    } else {
        Err(io::Error::from_raw_os_error(code))
    }
}

#[cfg(target_os = "macos")]
// Based on: https://github.com/Mobivity/nix-ptsname_r-shim/blob/master/src/lib.rs
fn ptsname_r(fd: BorrowedFd<'_>) -> io::Result<CString> {
    // This is based on
    // https://blog.tarq.io/ptsname-on-osx-with-rust/
    // and its derivative
    // https://github.com/philippkeller/rexpect/blob/a71dd02/src/process.rs#L67
    use libc::{c_ulong, ioctl, TIOCPTYGNAME};
    use std::os::unix::prelude::*;

    // the buffer size on OSX is 128, defined by sys/ttycom.h
    let buf: [i8; 128] = [0; 128];

    unsafe {
        match ioctl(fd.as_raw_fd(), TIOCPTYGNAME as c_ulong, &buf) {
            0 => {
                let res = CStr::from_ptr(buf.as_ptr()).to_owned();
                Ok(res)
            }
            _ => Err(io::Error::last_os_error()),
        }
    }
}
