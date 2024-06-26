use std::io;

fn main() {
    compound_types();
}

fn scalar_types() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let tup = (500, 6.4, 1);

    let (_t, y, _z) = tup;

    println!("The value of y is: {y}, {}", tup.2);
}

fn compound_types() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index:");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
