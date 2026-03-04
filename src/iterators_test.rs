// trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;
// }

pub fn test_iterators() {
    let num = vec![2, 3, 4, 5, 6, 7, 8, 9];
    let iterators = num.iter();
    println!("{:?}", &iterators);

    for num in iterators {
        println!("{}", num);
    }

    let double = num.iter().map(|x| x * 2);
    println!("{:?}", double);
    let result: Vec<i32> = double.collect();
    println!("{:?}", result);

    let even: Vec<&i32> = num.iter().filter(|x| *x % 2 == 0).collect();
    print!("{:?}", even);
    println!();

    let sum = num.iter().fold(0, |acc, curr| acc + curr);
    println!("{}", sum);

    let name: Vec<char> = "Vasanth".chars().collect();
    println!("{:?}", name);

    for (i, value) in vec![10, 20, 30].iter().enumerate() {
        println!("{i} {value}");
    }

    let res: Vec<i32> = vec![1, 2, 3, 4, 5].into_iter().skip(2).take(2).collect();
    println!("{:?}", res);

    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];

    let pairs: Vec<_> = a.into_iter().zip(b).collect();
    println!("{:?}", pairs);
}
