extern crate cpython;

mod extend;

use cpython::{py_module_initializer, py_fn};
use extend::py_implementation::{
    py_sum_int,
    py_test_thread
};




#[cfg(test)]
mod tests {
    use crate::extend::test::{test_sum_int};
    #[test]
    fn test_sum() {
        assert!(test_sum_int(),"test sum int")
    }
    #[test]
    fn test2(){

    }
}



py_module_initializer!(libfastpython, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py,"sum_int",py_fn!(py,py_sum_int(a: i64, b:i64)))?;
    m.add(py,"test_thread",py_fn!(py,py_test_thread()))?;
    Ok(())
});

