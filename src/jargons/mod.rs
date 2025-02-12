#[macro_export]
macro_rules! double {
    ($x: expr) => {
        println!("{}", $x * 2);
    };
}

pub fn normal_fn() -> () {
    println!("Just a normal function from jargons module")
}

pub fn print_number(number: u32) -> () {
    println!("{number} from jargons module", number = number)
}
