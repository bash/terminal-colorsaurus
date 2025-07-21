use pyo3::{
    create_exception,
    exceptions::{PyException, PyIndexError, PyValueError},
    prelude::*,
    types::PyString,
    PyTypeInfo,
};
use std::time::Duration;
use terminal_colorsaurus as imp;

/// Determines the background and foreground color of the terminal
/// using the OSC 10 and OSC 11 escape sequences.
///
/// This package helps answer the question "Is this terminal dark or light?".
#[pymodule]
fn colorsaurus(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(theme_mode, m)?)?;
    m.add_function(wrap_pyfunction!(foreground_color, m)?)?;
    m.add_function(wrap_pyfunction!(background_color, m)?)?;
    m.add_function(wrap_pyfunction!(color_palette, m)?)?;
    m.add("ColorsaurusError", py.get_type::<ColorsaurusError>())?;
    m.add("ColorScheme", py.get_type::<ThemeMode>())?;
    m.add("ColorPalette", py.get_type::<ColorPalette>())?;
    m.add("Color", py.get_type::<Color>())?;
    Ok(())
}

create_exception!(colorsaurus, ColorsaurusError, PyException);

/// Detects if the terminal is dark or light.
#[pyfunction]
#[pyo3(signature = (*, timeout=None))]
fn theme_mode(timeout: Option<Timeout>) -> PyResult<ThemeMode> {
    imp::theme_mode(query_options(timeout))
        .map(ThemeMode::from)
        .map_err(to_py_error)
}

/// Queries the terminal for it's foreground and background color.
#[pyfunction]
#[pyo3(signature = (*, timeout=None))]
fn color_palette(timeout: Option<Timeout>) -> PyResult<ColorPalette> {
    imp::color_palette(query_options(timeout))
        .map(ColorPalette)
        .map_err(to_py_error)
}

/// Queries the terminal for it's foreground color.
#[pyfunction]
#[pyo3(signature = (*, timeout=None))]
fn foreground_color(timeout: Option<Timeout>) -> PyResult<Color> {
    imp::foreground_color(query_options(timeout))
        .map(Color)
        .map_err(to_py_error)
}

/// Queries the terminal for it's background color.
#[pyfunction]
#[pyo3(signature = (*, timeout=None))]
fn background_color(timeout: Option<Timeout>) -> PyResult<Color> {
    imp::background_color(query_options(timeout))
        .map(Color)
        .map_err(to_py_error)
}

fn query_options(timeout: Option<Timeout>) -> imp::QueryOptions {
    let mut options = imp::QueryOptions::default();
    options.timeout = timeout.map(|t| t.0).unwrap_or(options.timeout);
    options
}

fn to_py_error(err: imp::Error) -> PyErr {
    ColorsaurusError::new_err(err.to_string())
}

struct Timeout(Duration);

impl<'py> FromPyObject<'py> for Timeout {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Duration::extract_bound(ob)
            .or_else(|_| u64::extract_bound(ob).map(Duration::from_secs))
            .or_else(|_| {
                f64::extract_bound(ob).and_then(|x| {
                    Duration::try_from_secs_f64(x).map_err(|e| PyValueError::new_err(e.to_string()))
                })
            })
            .map(Timeout)
    }
}

/// The terminal's theme mode (i.e. dark or light).
/// This can be retrieved by calling the color_scheme function.
#[pyclass(
    eq,
    eq_int,
    frozen,
    hash,
    module = "colorsaurus",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
#[derive(PartialEq, Eq, Hash)]
enum ThemeMode {
    /// The terminal uses a dark background with light text.
    Dark,
    /// The terminal uses a light background with dark text.
    Light,
}

impl From<imp::ThemeMode> for ThemeMode {
    fn from(value: imp::ThemeMode) -> Self {
        match value {
            imp::ThemeMode::Dark => Self::Dark,
            imp::ThemeMode::Light => Self::Light,
        }
    }
}

/// The color palette i.e. foreground and background colors of the terminal.
/// Retrieved by calling the color_palette function.
#[pyclass(eq, frozen, module = "colorsaurus")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColorPalette(imp::ColorPalette);

#[pymethods]
impl ColorPalette {
    #[getter]
    fn foreground(&self) -> Color {
        Color(self.0.foreground.clone())
    }

    #[getter]
    fn background(&self) -> Color {
        Color(self.0.background.clone())
    }

    #[getter]
    fn theme_mode(&self) -> ThemeMode {
        self.0.theme_mode().into()
    }

    #[pyo3(name = "__repr__")]
    fn repr(&self, python: Python<'_>) -> PyResult<String> {
        let ty = type_name::<Self>(&python)?;
        Ok(format!(
            "<{ty} foreground={fg}, background={bg}>",
            fg = self.foreground().repr(python)?,
            bg = self.background().repr(python)?
        ))
    }
}

/// An RGB color with 8 bits per channel.
#[derive(Debug, Clone, Eq, PartialEq)]
#[pyclass(eq, frozen, module = "colorsaurus")]
pub struct Color(imp::Color);

#[pymethods]
impl Color {
    #[classattr]
    #[pyo3(name = "BLACK")]
    fn black() -> Self {
        Self(imp::Color::default())
    }

    #[new]
    fn new(red: u8, green: u8, blue: u8) -> Self {
        Self(imp::Color::rgb(
            scale_to_u16(red),
            scale_to_u16(green),
            scale_to_u16(blue),
        ))
    }

    #[getter]
    fn red(&self) -> u8 {
        self.0.scale_to_8bit().0
    }

    #[getter]
    fn green(&self) -> u8 {
        self.0.scale_to_8bit().1
    }

    #[getter]
    fn blue(&self) -> u8 {
        self.0.scale_to_8bit().2
    }

    fn perceived_lightness(&self) -> f32 {
        self.0.perceived_lightness()
    }

    #[pyo3(name = "__len__")]
    fn get_length(&self) -> usize {
        3
    }

    #[pyo3(name = "__getitem__")]
    fn get_item(&self, n: usize) -> PyResult<u8> {
        match n {
            0 => Ok(self.red()),
            1 => Ok(self.green()),
            2 => Ok(self.blue()),
            _ => Err(PyIndexError::new_err(())),
        }
    }

    #[pyo3(name = "__repr__")]
    fn repr(&self, python: Python<'_>) -> PyResult<String> {
        let (r, g, b) = self.0.scale_to_8bit();
        let ty = type_name::<Self>(&python)?;
        Ok(format!("<{ty} #{r:02x}{g:02x}{b:02x}>"))
    }
}

fn scale_to_u16(channel: u8) -> u16 {
    (channel as u32 * (u16::MAX as u32) / (u8::MAX as u32)) as u16
}

fn type_name<'py, T: PyTypeInfo>(python: &Python<'py>) -> PyResult<Bound<'py, PyString>> {
    python.get_type::<T>().name()
}
