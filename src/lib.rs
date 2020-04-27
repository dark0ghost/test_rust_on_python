#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!()
    }
}

#[macro_use]
extern crate cpython;



use cpython::{Python, PyResult};

fn count_doubles(_: Python,a: i64 , b: i64) -> PyResult<i64> {
    let c: i64 = a*b;
    Ok(c)
}

py_module_initializer!(rust2py, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "sum_as_string", py_fn!(py,count_doubles(a: i64 , b: i64)))?;
    Ok(())
});
