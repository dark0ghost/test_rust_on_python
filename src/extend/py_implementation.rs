use cpython::{Python, PyResult};

pub(crate) mod prototypes;


pub fn py_sum_int(py: Python, a: i64, b: i64) -> PyResult<i64>{
    Ok(prototypes::sum_int(a,b))
}

pub fn py_test_thread(py: Python)-> PyResult<i64>{
    Ok(prototypes::test_thread())
}