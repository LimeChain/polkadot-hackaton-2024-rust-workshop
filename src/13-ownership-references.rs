#![allow(unused, clippy::ptr_arg)]

// NOTE: options to keep string:
// - return it back
// - only get a reference

fn string_length(s: String) -> usize {
    s.len()
}

// fn append_name(base: String, name: String) {
//     base.push_str(&name);
// }

// fn print_my_string(s: &String) {
//     let l = string_length(s);
//     println!("String \"{}\" has length {}", s, l);
// }

pub fn main() {
    let s = String::from("test");
    // let l = string_length(&s);
    // println!("String \"{}\" has length {}", s, l);

    // let my_ref = &mut s;
    // append_name(my_ref, String::from("david"));
    // print_my_string(&s);
    // append_name(my_ref, String::from("pavel"));
}
