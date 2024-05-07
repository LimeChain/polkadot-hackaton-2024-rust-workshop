// {{{
#![allow(unused, clippy::ptr_arg)]
// }}}

// NOTE: options to keep string:

// - return it back
fn string_length_returns(s: String) -> (usize, String) {
    (s.len(), s)
}

// - only get a reference
// NOTE:            V non-mutable reference
fn string_length(s: &String) -> usize {
    s.len()
}

// {{{
// NOTE:             VVVV mutable reference
fn append_name(base: &mut String, name: &String) {
    base.push_str(name);
}
// }}}

// {{{
fn print_my_string(s: &String, len: usize) {
    println!("String \"{}\" has length {}", s, len);
}
// }}}

pub fn main() {
    let mut s = String::from("test");
    let len = string_length(&s);

    append_name(&mut s, &String::from(" test"));

    // {{{

    // We can have EITHER MULTIPLE readers (non-mut references)
    //                 OR just ONE writer  (    mut reference )

    // NOTE: Uncomment to show that we can only have 1 writer
    //
    // let ref_1 = &mut s;
    // let ref_2 = &mut s;
    // append_name(ref_1, &String::from("david"));
    // append_name(ref_2, &String::from("pavel"));

    // NOTE: Uncomment to show that we can have no readers when there is already a writer
    //
    // let ref_1 = &mut s;
    // print_my_string(&s, len);
    // append_name(ref_1, &String::from("david"));

    // }}}

    let len = string_length(&s);

    println!("String \"{}\" has length {}", s, len);
}

// vim: set foldmethod=marker:
