use std::thread;

/* thread lifetime <= scope lifetime
scope lifetime <= function lifetime
thts why this bloody rust allows to borrows referance */

pub fn thread_scope() {
    // let x = 10;
    // thread::scope(|s| {
    //     s.spawn(|| println!("from 1: {}", x));
    //     s.spawn(|| println!("from 2: {}", x));
    //     s.spawn(|| println!("from 3: {}", x));
    //     s.spawn(|| println!("from 4: {}", x));
    // });

    let data: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    // let first_half = &data[0..4];
    // let second_half = &data[4..];

    // thread::scope(|s| {
    //     s.spawn(|| {
    //         println!("{:?}", first_half);
    //     });
    //     s.spawn(|| {
    //         println!("{:?}", second_half);
    //     });
    // });

    thread::scope(|s| {
        for _ in 1..4 {
            s.spawn(|| println!("{:?}", data));
        }
    });

    println!("Outer thread scope");
}
