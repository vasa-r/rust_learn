use chrono::{DateTime, Local};

fn main() {
    // println!("hi") // macro

    println!("hi");

    let ans: u32 = sum(14382, 2);

    println!("{}", ans);

    greet("Vasanth", "Evening");

    let time_now: DateTime<Local> = Local::now();

    let formatted_date = time_now.format("%Y/%m/%d");

    println!("{}", formatted_date);

    println!("{}", time_now);
}

fn sum(a: u32, b: u32) -> u32 {
    return a + b;
}

fn greet(name: &str, time: &str) -> () {
    println!(
        "Hey {name}, Good {time}. How are you?",
        name = name,
        time = time
    )
}
