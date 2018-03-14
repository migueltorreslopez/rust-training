


pub mod threads {
    use std::thread;
    pub fn two(){
        println!("In main thread");

        // Start second thread
        thread::spawn(move || {
            for i in 1..5{
                println!("In second thread {}", i);
            }
        });

        for i in 1..5 {
                println!("In main thread {}", i);
            }
    }
}