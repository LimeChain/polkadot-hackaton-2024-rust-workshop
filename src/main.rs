fn main() {
    let x = 5;
    let mut reff = &x;

    {
        let y = 8;
        reff = &y;
    }


    println!("{}", *reff);
}
