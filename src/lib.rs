use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use squall_router::SquallRouter;

#[pyclass]
pub struct Router {
    router: SquallRouter,
}

#[pymethods]
impl Router {
    #[new]
    fn new() -> Self {
        Router {
            router: SquallRouter::new(),
        }
    }

    pub fn add_http_route(&mut self, method: String, path: String, handler: i32) -> PyResult<()> {
        Ok(self.router.add_http_route(method, path, handler))
    }

    pub fn add_http_location(&mut self, method: String, path: String, handler: i32) -> PyResult<()> {
        Ok(self.router.add_http_location(method, path, handler))
    }

    pub fn get_http_handler<'a>(
        &'a self,
        method: &str,
        path: &'a str,
    ) -> PyResult<Option<(i32, Vec<&str>, Vec<&'a str>)>> {
        if let Some(result) = self.router.get_http_handler(method, path) {
            return Ok(Some((result.0, result.1, result.2)));
        }
        Ok(None)

    }

    // pub fn add_validator(&mut self, validator: String, regex: String) -> PyResult<()> {
    //     match self.router.path_parser.add_validator(validator, regex) {
    //         Ok(v) => Ok(v),
    //         Err(e) => Err(PyValueError::new_err(e.to_string())),
    //     }
    // }

    pub fn set_ws_handler(&self, _path: String) -> PyResult<()> {
        Ok(())
    }


    pub fn get_ws_handler(&self, handler: PyObject) -> PyResult<PyObject> {
        Ok(handler)
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn squall_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Router>()?;
    Ok(())
}
