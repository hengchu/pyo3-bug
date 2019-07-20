use pyo3::prelude::*;
use pyo3::PyNumberProtocol;
use pyo3::types::PyAny;

#[pyclass]
#[derive(Debug)]
struct BinaryArithmetic {}

#[pyproto]
impl PyNumberProtocol for BinaryArithmetic {
    fn __add__(lhs: &PyAny, rhs: &PyAny) -> PyResult<String> {
        Ok(format!("{:?} + {:?}", lhs, rhs))
    }

    fn __radd__(&self, other: &PyAny) -> PyResult<String> {
        Ok(format!("{:?} + {:?}", self, other))
    }
}
