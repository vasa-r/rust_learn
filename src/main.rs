mod jargons;
use chrono::{DateTime, Local};
use jargons::{normal_fn, print_number};
mod iterators_test;

fn main() {
    // println!("hi") // macro
    // println!("hi");

    let ans: u32 = sum(14382, 2);

    // println!("{}", ans);

    // -------------------------------------dynamic args - string-----------------------------------------------------

    greet("Vasanth", "Evening");

    // ------------------------------------------date - chrono------------------------------------------------

    let time_now: DateTime<Local> = Local::now();

    let formatted_date = time_now.format("%Y/%m/%d");

    // println!("{}", formatted_date);

    // println!("{}", time_now);

    // -----------------------------------------signed & unsigned-------------------------------------------------

    // signed --> i32, i64... which includes sign(+,-) | both negative and positive numbers
    // un-signed --> u32, u64... which includes only positive numbers

    let signed: i32 = -32;

    let b = signed;

    // print!("{signed} {b}");

    // println!("{}", signed);

    // -----------------------------------------boolean-------------------------------------------------

    // println!("{}", is_eligible(32));

    // ----------------------------------------macro from jargons module--------------------------------

    double!(2);

    // ----------------------------------------functions from jargons module----------------------------

    normal_fn();
    print_number(10);

    // ----------------------------------------mutable and immutable-------------------------------------

    // let num: u32 = 10;
    // num = 4; this wont compile. by default all the data are immutable in rust

    let mut num: u32 = 10;
    // println!("Initial value: {}", num); // ✅ Now `10` is used before overwriting

    num = 4;
    // println!("Updated value: {}", num);

    // ----------------------------------------borrowing-------------------------------------

    /*
    You can have multiple immutable references
    OR one mutable reference
    But not both at the same time
     */

    let mut owner = String::from("Vasa");
    let borrower1 = &owner;
    // println!("{borrower1}"); // works
    let borrower2: &mut String = &mut owner;
    borrower2.push_str(" Raman");

    // println!("{borrower1}"); wont work

    // ----------------------------------------iterators-------------------------------------

    iterators_test::test_iterators();
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

fn is_eligible(age: u32) -> bool {
    return age >= 18;
}
