use std::time::Duration;

use pyo3::{create_exception, exceptions::PyException, prelude::*, types::PyInt};
use terminal_colorsaurus as imp;

create_exception!(colorsaurus, ColorsaurusError, PyException);

/// Formats the sum of two numbers as string.
#[pyfunction]
fn color_scheme(timeout: Option<Timeout>) -> PyResult<ColorScheme> {
    terminal_colorsaurus::color_scheme(imp::QueryOptions::default())
        .map(ColorScheme::from)
        .map_err(|e| ColorsaurusError::new_err(e.to_string()))
}

struct Timeout(Duration);

impl<'py> FromPyObject<'py> for Timeout {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Duration::extract_bound(ob)
            .or_else(|_| u64::extract_bound(ob).map(Duration::from_secs))
            .map(Timeout)
    }
}

// impl From<Timeout> for Duration {
//     fn from(value: Timeout) -> Self {
//         match value {
//             Timeout::WholeSeconds(s) => Duration::from_secs(s),
//             Timeout::Seconds(s) => Duration::try_from_secs_f64(secs),
//         }
//     }
// }

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

/// A Python module implemented in Rust.
#[pymodule]
fn colorsaurus(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(color_scheme, m)?)?;
    m.add("ColorsaurusError", py.get_type_bound::<ColorsaurusError>())?;
    m.add("ColorScheme", py.get_type_bound::<ColorScheme>())?;
    Ok(())
}
