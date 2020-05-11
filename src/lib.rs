#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

    }

    #[test]
    fn test() {

    }
}

//#[macro_use]
extern crate cpython;

use cpython::{PyResult, Python, py_module_initializer, py_fn};

pub fn s(c:i64){

}

py_module_initializer!(libfastpython, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "sum_as_string", py_fn!(py, s(c)))?;
    Ok(())
});

