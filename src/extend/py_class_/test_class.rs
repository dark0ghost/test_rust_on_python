use cpython::{py_class,PyResult};


py_class!(pub class Test |py| {
    data number: i32;
    def __new__(_cls, arg: i32) -> PyResult<Test> {
        Test::create_instance(py, arg)
    }
    def half(&self) -> PyResult<i32> {
        println!("half() was called with self={:?}", self.number(py));
        Ok(self.number(py) / 2)
    }
    def print(&self)-> PyResult<i32>{
     println!("1");
     Ok(0)
    }
});
