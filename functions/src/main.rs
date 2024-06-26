fn before_main() {
    println!("This function is defined before main() !!");
}

fn main() {
    println!("Hello, world!");
    another_function(32, 'a');
    before_main();
}

fn another_function(x: i32, unit: char) {
    println!("Another function! {x} {unit}");
}
