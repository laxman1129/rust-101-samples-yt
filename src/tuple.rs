pub fn run() {
    // fn, plc age(0-255)
    let person: (&str, &str, u8) = ("Lax", "C", 32);

    // one of the ways to access values from tuples
    println!("{} is from {} and is {}", person.0, person.1, person.2);
}
