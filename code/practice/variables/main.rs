fn main() {
    println!("Hello, world!");
    let _x: i32 = 5;
    //assert_eq!(x, 3);
    println!("Success!");
    scope_def();
    strin_def();
}

fn scope_def() {
    let x: i32 = 10;
    let y: i32 = 89;
    {
        println!("the value of x is {} and y is {}", x, y)
    }
}

fn strin_def() {
    let y: &str = "I am writing Rust";
    println!("the contents of y is {}", y);
}
