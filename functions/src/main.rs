fn before_main() {
    println!("This function is defined before main() !!");
}

fn main() {
    println!("Hello, world!");
    another_function(32, 'a');
    before_main();
    whats_this();
    implicit_return();
    ex_plus_one();
}

fn another_function(x: i32, unit: char) {
    println!("Another function! {x} {unit}");
}

fn whats_this() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn implicit_return() {
    fn five() -> i32 {
        5
    }

    let x = five();
    println!("The value of x is: {x}");
}

fn ex_plus_one() {
    fn plus_one(x: i32) -> i32 {
        x + 1
    }

    let x = plus_one(5);
    println!("The value of x is: {x}");
}
