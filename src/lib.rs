#[cfg(test)]
mod tests {
    use crate::extend::math::Math;
    use cpython::{ ToPyObject};
    #[test]
    fn it_works() {
        let a = Math::__init__();

    }
}

//#[macro_use]
extern crate cpython;
use cpython::{py_module_initializer, py_fn};

mod extend;

use extend::py_implementation::{py_sum_int};



py_module_initializer!(libfastpython, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py,"sum_int",py_fn!(py,py_sum_int(a: i64, b:i64)))?;
    Ok(())
});

