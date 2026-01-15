use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            test_threads(i)
        }
    });

    for i in 1..10 {
        println!("Hi number {i} from main thread!");
        thread::sleep(Duration::from_millis(10));
    }
}


fn test_threads(x: u16) {
    println!("Hi number {x} from spawn thread!");
    thread::sleep(Duration::from_millis(2));
}
