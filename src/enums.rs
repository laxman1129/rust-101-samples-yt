enum Movement {
    // varients
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    // match is similar to switch

    match m {
        Movement::Up => println!("Avatar moving up"),
        Movement::Down => println!("Avatar moving up"),
        Movement::Left => println!("Avatar moving up"),
        Movement::Right => println!("Avatar moving up"),
    }
}

fn move_avatar_ref(m: &Movement) {
    // match is similar to switch

    match m {
        Movement::Up => println!("Avatar moving up"),
        Movement::Down => println!("Avatar moving down"),
        Movement::Left => println!("Avatar moving left"),
        Movement::Right => println!("Avatar moving right"),
    }
}

pub fn run() {
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Down;
    let avatar = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
    
    // move_avatar(avatar4); // value used here after move

    // This declaration solves the issue of move
    move_avatar_ref(&avatar);
    move_avatar_ref(&avatar);

}
