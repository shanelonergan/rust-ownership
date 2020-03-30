fn main() {
    // variable scope
    let s = "hello";

    {                       // s is not valid here, it’s not yet declared
        let s = "hello";    // s is valid from this point forward

        // do stuff with s
    }                       // this scope is now over, and s is no longer valid

    // The *String* type
    let mut s = String::from("hello");

    s.push_str(", world!"); // this appends a literal into a String

    println!("{}", s);

    // Variable and Data Interaction
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2); // s1 is now invalid

    // *Cloning* copies heap data
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // *Copying* copies only stack data if the size of the value is known at compile time
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);


}
