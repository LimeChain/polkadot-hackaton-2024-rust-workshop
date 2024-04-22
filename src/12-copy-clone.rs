pub fn main() {
    // Simple types (like integers) can be implicitly copied
    let a = 5;
    //           V moved by default, but copied here, because of the println below
    let b = a;

    // this required both `a` and `b` to have values
    println!("a is {}, b is {}", a, b);

    // More complex types (ones involving pointers, most commonly)
    // cannot be implicitly copied

    let s1 = String::from("string1");
    //               VV moved, not copied
    let s2 = s1.clone();

    // error
    println!("s1 is \"{}\", s2 is \"{}\"", s1, s2);
}
