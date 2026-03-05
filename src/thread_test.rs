use ::std::thread;
use ::std::time::{Duration, Instant};

pub fn thread_test() {
    let start = Instant::now();
    // thread::spawn(|| println!("Hello fom new thread"));

    // println!("first");

    // let handle = thread::spawn(|| {
    //     for i in 1..=1_000_000 {
    //         println!("{i}");
    //     }
    // });

    // handle.join().expect("error");

    // for i in 1..=1_000_000 {
    //     println!("{i}")
    // }

    // let sleep_time = Duration::from_secs(5);

    // let handle1 = thread::spawn(|| {
    //     let sum = 2 + 2;

    //     panic!("error in this thread");
    // });

    // let result = handle1.join();
    // match result {
    //     Ok(d) => println!("{:?}", d),
    //     Err(e) => {
    //         if let Some(s) = e.downcast_ref::<&'static str>() {
    //             eprintln!("Panic message: {}", s);
    //         }
    //     }
    // }

    // let message = String::from("Vasa here");

    // thread::spawn(move || println!("{}", message));

    let mut handles: Vec<thread::JoinHandle<u8>> = vec![];

    for i in 1..=5 {
        let handle = thread::spawn(move || {
            println!("{i}");
            i
        });
        handles.push(handle);
    }

    // println!("{:?}", handles);

    for handle in handles {
        println!("{:?}", handle.join().unwrap());
    }

    // thread::sleep(sleep_time); // blocking main thread for 5 secs
    println!("from main thread");

    let total_time = start.elapsed();
    println!("Total time in single thread: {}", total_time.as_secs()); // in one thread avg is 9 sec, well wiht threads its only 6 secs. ps: machine dep
}
