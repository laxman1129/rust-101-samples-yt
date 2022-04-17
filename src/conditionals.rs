pub fn run() {
    let age: u8 = 18;
    let check_id: bool = true;
    let knows_person_of_age: bool = true;

    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender says what would you like to drink");
    } else if age < 21 && check_id {
        println!("Bartender says sorry u hv 2 lv");
    } else {
        println!("I need to see ur id")
    }

    // shorthand , no ternary operator
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age : {}", is_of_age);
}
