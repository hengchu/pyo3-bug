#![feature(specialization)]

use pyo3::prelude::*;
use pyo3::PyNumberProtocol;
use pyo3::types::PyAny;

#[pyclass]
#[derive(Debug)]
struct BinaryArithmetic {}

#[pyproto]
impl<'p> PyNumberProtocol<'p> for BinaryArithmetic {
    fn __add__(lhs: &PyAny, rhs: &PyAny) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        if !py.is_instance::<BinaryArithmetic, _>(lhs).unwrap() {
            return Ok(py.NotImplemented())
        }
        Ok(format!("{:?} + {:?}", lhs, rhs).into_object(py))
    }

    fn __radd__(&self, other: &PyAny) -> PyResult<String> {
        panic!("called");
        Ok(format!("{:?} + {:?}", self, other))
    }
}

#[pymethods]
impl BinaryArithmetic {
    #[new]
    fn new(obj: &PyRawObject) {
        obj.init(BinaryArithmetic {})
    }
}

#[pymodule]
fn pyo3_bug(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<BinaryArithmetic>().unwrap();
    Ok(())
}
