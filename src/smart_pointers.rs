use std::sync::Mutex;
use std::thread;
use std::{rc::Rc, sync::Arc};

pub fn smart_poniters() {
    let x = Box::new(5);
    println!("{:p}", x);

    let name = Rc::new(String::from("Vasa"));
    let a = Rc::clone(&name);
    let b = Rc::clone(&name);
    let count_before = Rc::strong_count(&name);
    let weak_count = Rc::weak_count(&name);

    println!("Count of Rc is {count_before}");
    println!("weak Count of Rc is {weak_count}");
    println!(
        "Pointers: name:{:p}, a:{:p}, b:{:p}",
        name.as_ptr(),
        a.as_ptr(),
        b.as_ptr()
    );

    let data = Arc::new(vec![1, 2, 3, 4]);
    let mut handles: Vec<_> = vec![];

    for i in 1..=3 {
        let shared = Arc::clone(&data);

        let handle = thread::spawn(move || println!("thread {} sees {:?}", i, shared));

        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Arc Mutex");

    let counter = Arc::new(Mutex::new(0));
    let mut handles1 = vec![];

    for i in 1..=10 {
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();

            *num += 1;
            println!("{i}: {num}");
        });

        handles1.push(handle);
    }

    for h in handles1 {
        h.join().unwrap();
    }
    println!("result {}", *counter.lock().unwrap());
}
