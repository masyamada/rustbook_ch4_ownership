use std::io;

fn main() {
    let mut s = String::from("program started");  // s comes into scope

    println!("{s}");
    
    s = takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
    println!("{s}");    // unless takes_ownership returns it!!!

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(mut some_string: String) -> String { // some_string comes into scope
    println!("the string passed is: {some_string}");

    let mut your_name = String::new(); // Why do I have to declare this for every cycle?

    println!("What is your name?");
    io::stdin()
        .read_line(&mut your_name)
        .expect("Failed to read your name on the input");


    // ". I was added in the function.");
    some_string.push_str(". I was added");
    let addendum = String::from(" in the function.");
    some_string.push_str(addendum.as_str());
    
    println!("the string is now: {some_string}");
 
    some_string.clear();
    some_string.push_str("Hello, ");
    some_string.push_str(your_name.as_str());

    println!("the string is now: {some_string}");
 
    some_string

} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.