use pyo3::{
    create_exception,
    exceptions::{PyException, PyIndexError, PyValueError},
    prelude::*,
};
use std::{time::Duration, u16};
use terminal_colorsaurus as imp;

create_exception!(colorsaurus, ColorsaurusError, PyException);

/// Detects if the terminal is dark or light.
#[pyfunction]
#[pyo3(signature = (timeout=None))]
fn color_scheme(timeout: Option<Timeout>) -> PyResult<ColorScheme> {
    imp::color_scheme(query_options(timeout))
        .map(ColorScheme::from)
        .map_err(to_py_error)
}

#[pyfunction]
#[pyo3(signature = (timeout=None))]
fn color_palette(timeout: Option<Timeout>) -> PyResult<ColorPalette> {
    imp::color_palette(query_options(timeout))
        .map(ColorPalette)
        .map_err(to_py_error)
}

#[pyfunction]
#[pyo3(signature = (timeout=None))]
fn foreground_color(timeout: Option<Timeout>) -> PyResult<Color> {
    imp::foreground_color(query_options(timeout))
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

#[pyclass(eq, eq_int, frozen, hash)]
#[derive(PartialEq, Eq, Hash)]
enum ColorScheme {
    #[pyo3(name = "DARK")]
    Dark,
    #[pyo3(name = "LIGHT")]
    Light,
}

impl From<imp::ColorScheme> for ColorScheme {
    fn from(value: imp::ColorScheme) -> Self {
        match value {
            imp::ColorScheme::Dark => Self::Dark,
            imp::ColorScheme::Light => Self::Light,
        }
    }
}

/// The color palette i.e. foreground and background colors of the terminal.
/// Retrieved by calling [`color_palette`].
#[pyclass(eq, frozen)]
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
}

/// An RGB color with 16 bits per channel.
/// You can use [`Color::scale_to_8bit`] to convert to an 8bit RGB color.
#[derive(Debug, Clone, Eq, PartialEq)]
#[pyclass(eq, frozen)]
pub struct Color(imp::Color);

impl From<imp::Color> for Color {
    fn from(value: imp::Color) -> Self {
        Self(value)
    }
}

#[pymethods]
impl Color {
    #[classattr]
    #[pyo3(name = "BLACK")]
    fn black() -> Self {
        Self(imp::Color::default())
    }

    #[new]
    fn new(red: u16, green: u16, blue: u16) -> Self {
        Self(imp::Color {
            r: red,
            g: green,
            b: blue,
        })
    }

    #[getter]
    fn red(&self) -> u16 {
        self.0.r
    }

    #[getter]
    fn green(&self) -> u16 {
        self.0.g
    }

    #[getter]
    fn blue(&self) -> u16 {
        self.0.b
    }

    #[pyo3(name = "__len__")]
    fn get_length(&self) -> usize {
        3
    }

    fn scale_to_8bit(&self) -> (u8, u8, u8) {
        self.0.scale_to_8bit()
    }

    #[pyo3(name = "__getitem__")]
    fn get_item(&self, n: usize) -> PyResult<u16> {
        match n {
            0 => Ok(self.red()),
            1 => Ok(self.green()),
            2 => Ok(self.blue()),
            _ => Err(PyIndexError::new_err(())),
        }
    }

    #[pyo3(name = "__repr__")]
    fn repr(&self) -> String {
        let (r, g, b) = self.0.scale_to_8bit();
        format!("Color(#{r:02x}{g:02x}{b:02x})")
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn colorsaurus(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(color_scheme, m)?)?;
    m.add_function(wrap_pyfunction!(foreground_color, m)?)?;
    m.add_function(wrap_pyfunction!(color_palette, m)?)?;
    m.add("ColorsaurusError", py.get_type_bound::<ColorsaurusError>())?;
    m.add("ColorScheme", py.get_type_bound::<ColorScheme>())?;
    m.add("ColorPalette", py.get_type_bound::<ColorPalette>())?;
    m.add("Color", py.get_type_bound::<Color>())?;
    Ok(())
}
