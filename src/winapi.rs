use crate::{Color, Error, QueryOptions, Result};
use std::os::windows::io::AsRawHandle;
use std::{env, io, mem};
use terminal_trx::{terminal, ConsoleHandles as _};
use windows_sys::Win32::Foundation::{BOOL, COLORREF, HANDLE};
use windows_sys::Win32::System::Console::{
    GetConsoleScreenBufferInfoEx, CONSOLE_SCREEN_BUFFER_INFOEX, COORD, SMALL_RECT,
};

pub(crate) fn foreground_color(_options: QueryOptions) -> Result<Color> {
    let (foreground, _) = colors_from_winapi()?;
    error_on_windows_terminal()?;
    Ok(foreground)
}

pub(crate) fn background_color(_options: QueryOptions) -> Result<Color> {
    let (_, background) = colors_from_winapi()?;
    error_on_windows_terminal()?;
    Ok(background)
}

// Windows Terminal does not correctly console colors:
// https://github.com/microsoft/terminal/issues/10639
fn error_on_windows_terminal() -> Result<()> {
    match env::var("WT_SESSION") {
        Ok(_) => Err(Error::UnsupportedTerminal),
        Err(_) => Ok(()),
    }
}

fn colors_from_winapi() -> Result<(Color, Color)> {
    let terminal = terminal()?;
    let screen_buffer = terminal.screen_buffer_handle();

    // SAFETY: GetConsoleScreenBufferInfo fills in the info structure
    // on successful return.
    let info: CONSOLE_SCREEN_BUFFER_INFOEX = unsafe {
        let mut info = empty_screen_buffer_info();
        to_io_result(GetConsoleScreenBufferInfoEx(
            screen_buffer.as_raw_handle() as HANDLE,
            &mut info,
        ))?;
        info
    };

    Ok(extract_colors(info))
}

// wAttributes contains contains a 4 bit index into ColorTable
// for each color (foreground, background).
//
// wAttributes:    x x x x  x x x x  x x x x  x x x x
//                                            ^^^^^^^ foreground color index
//            background color index ^^^^^^^
//
// The positions are derived from the BACKGROUND_* and FOREGROUND_* constants.
//
// See: https://stackoverflow.com/a/9509664
fn extract_colors(info: CONSOLE_SCREEN_BUFFER_INFOEX) -> (Color, Color) {
    let foreground_index = (info.wAttributes & 0b1111) as usize;
    let background_index = ((info.wAttributes >> 4) & 0b1111) as usize;
    (
        to_color(info.ColorTable[foreground_index]),
        to_color(info.ColorTable[background_index]),
    )
}

fn to_color(color: COLORREF) -> Color {
    // From the Win32 docs (https://learn.microsoft.com/en-us/windows/win32/gdi/colorref):
    //    When specifying an explicit RGB color, the COLORREF value has the following hexadecimal form:
    //    0x00bbggrr
    debug_assert!(color <= 0x00FFFFFF);
    let red = (color & 0xFF) as u8;
    let green = ((color >> 8) & 0xFF) as u8;
    let blue = ((color >> 16) & 0xFF) as u8;
    Color::from_8bit(red, green, blue)
}

fn to_io_result(result: BOOL) -> io::Result<()> {
    if result == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

fn empty_screen_buffer_info() -> CONSOLE_SCREEN_BUFFER_INFOEX {
    CONSOLE_SCREEN_BUFFER_INFOEX {
        cbSize: mem::size_of::<CONSOLE_SCREEN_BUFFER_INFOEX>() as u32,
        dwSize: empty_coord(),
        dwCursorPosition: empty_coord(),
        wAttributes: 0,
        srWindow: empty_rect(),
        dwMaximumWindowSize: empty_coord(),
        wPopupAttributes: 0,
        bFullscreenSupported: 0,
        ColorTable: [0; 16],
    }
}

fn empty_coord() -> COORD {
    COORD { X: 0, Y: 0 }
}

fn empty_rect() -> SMALL_RECT {
    SMALL_RECT {
        Left: 0,
        Top: 0,
        Right: 0,
        Bottom: 0,
    }
}
