
pub(crate)mod extend_lib{
    pub struct TestNode{
        a : i64,
        b : i64,
        data: Option<i128>

    }

    impl TestNode{
     pub  fn init(&mut self, a:i64, b: i64){
         self.a = a;
         self.b = b;
         self.data = None;
        }
       pub fn data_set(&mut self,s: Option<i128> ) {
            self.data = s
        }
    pub    fn f() -> i8 {
          unsafe {
              let  c: &[i8] = &([2,3,4]);
              let u : i8 = (&c)[0];
              u
          }
        }

    }

}