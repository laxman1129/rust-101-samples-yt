// string and string operations
// There are two types of string
// Premitive str = str is immutable, fixed length string somewhere in memory
// String = Growable, heap allocated datastructure - used when there is need to modify the string

pub fn run() {
    // of type &str
    let hello = "Hello world";
    println!("{}", hello);

    // type String
    let mut abc = String::from("From"); // need to add mut so that we can update the variable
    println!("{}", abc);

    // get length
    println!("hello : {}", hello.len());
    println!("abc : {}", abc.len());

    // add character to string
    abc.push('!'); // if abc is not mut thn this will give error

    // add more tha one char
    abc.push_str("string");

    println!("{}", abc);

    // capacity in bytes
    println!("capacity : {}", abc.capacity());
    println!("is empty : {}", abc.is_empty());
    println!("conatins sub str : {}", abc.contains("Fr"));
    println!("conatins sub str : {}", abc.contains("lax"));

    // replace word
    // abc.replace("ing", "ed"); => id not assigned thn value does not change and warning at runtime/compiletime
    abc = abc.replace("ing", "ed");
    println!("replacing ing to ed : {}", abc);

    // loop through string by whitespace
    for word in "abc sdlkj lk sdo ls slkdj lskdj;ls k".split_whitespace() {
        println!("{}", word);
    }

    // create string with certain capacity
    let mut s = String::with_capacity(20);
    s.push('a');
    s.push('b');

    assert_eq!(2, s.len()); // passes; // assert_eq!(3,s.len()); // fails
    assert_eq!(20, s.capacity()); // if fails thn error in the console else no issue
}
