use std::thread;
use std::time::Duration;
pub fn sum_int(a: i64, b: i64) -> i64{
    a + b
}

pub fn test_thread()-> i64{
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    return 1;

}