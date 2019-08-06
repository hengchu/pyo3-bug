use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyclass]
#[derive(Debug)]
struct FooBar {}

#[pyfunction]
fn say_hello(_py: Python, _foobar: &FooBar) -> PyResult<String> {
    Ok("hello".to_string())
}

#[pymodule]
fn pyo3_bug(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<FooBar>().unwrap();
    m.add_wrapped(wrap_pyfunction!(say_hello));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let foobar = FooBar {};
        let code = r#"
import sys

sys.path.append('.')

from pyo3_bug import say_hello

print(say_hello(foobar))
            "#;

        let main_module = py.import("__main__").unwrap();
        let globals = main_module.dict().copy().unwrap();
        globals
            .set_item("foobar", PyRef::new(py, foobar).unwrap())
            .unwrap();

        py.run(code, Some(globals), None).map_err(|e| {
            e.print(py);
            assert!(false);
        });
    }
}
