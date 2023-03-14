use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..20 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..7 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(2));
    }

    handle.join().unwrap();
}
