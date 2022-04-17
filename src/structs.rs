// used to create custom data types

// Traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple struct
struct Colour(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first_name: &str, last_name: &str) -> Person {
        Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        }
    }

    fn full_name(&self) -> String {
        // this is a macro
        //  similar to print but it returns the value rather than printing it
        // format!("{} {}", &self.first_name, &self.last_name) // & can be removed
        format!("{} {}", self.first_name, self.last_name)
    }

    // because we want to mutate self its &mut self
    // because we mutated the self no need to return any value
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    println!("Colot : {} {} {}", c.red, c.green, c.blue);

    // change props
    c.red = 200;
    println!("Colot : {} {} {}", c.red, c.green, c.blue);

    let mut p = Colour(1, 12, 9);
    println!("Colot : {} {} {}", p.0, p.1, p.2);

    // change props
    p.0 = 0;
    println!("Colot : {} {} {}", p.0, p.1, p.2);

    let mut p = Person::new("Lax", "C");
    println!("Person {} {} ", p.first_name, p.last_name);
    println!("Person's full name is  {} ", p.full_name());
    p.set_last_name("Chari");
    println!("Person's full name is  {} ", p.full_name());
    println!("Person's tuple is  {:?} ", p.to_tuple());
}
