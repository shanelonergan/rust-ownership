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

}
