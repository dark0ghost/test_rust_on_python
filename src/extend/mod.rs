pub(crate) mod py_implementation;
pub(crate) mod py_class_;

pub(crate) mod test{
    use crate::extend::py_implementation::prototypes::sum_int;

    pub fn test_sum_int() -> bool {
      sum_int(34,11) == 45
    }

}