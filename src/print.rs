// create a function here and run it in the main.rs file

// pub =>  public , can be used in another files
pub fn run() {
    // print to the console
    println!("hello from the print.rs file");
    // give error as string is expected
    // println!(1);
    // to solve issue, suggestion is provided by the compiler
    println!("{}", 1);

    // basic formatting
    println!("hello {}, good {}", "Lax", "morning");

    // positional arguments
    println!("hello {0}, good {1} {0}", "Lax", "morning");

    // named args
    println!("hello {name}, good {time}", name = "Lax", time = "morning");

    // placeholder traits
    println!("binary: {:b}, hex: {:x}, octal: {:o}", 10, 10, 10);

    // placeholder for debug traits , used for printing entire array etc.
    println!("{:?}",(1, true, "Hello"));

    // basic math
    println!("10 + 10 = {}", 10+10);
}
