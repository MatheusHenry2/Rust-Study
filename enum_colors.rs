enum Colors {
    Red,
    Yellow,
    Blue,
}

fn which_color(color: Colors) {
    match color {
        Colors::Red => println!("RED"),
        Colors::Yellow => println!("Yellow"),
        Colors::Blue => println!("Blue"),
    }
}

fn main() {
    which_color(Colors:: Blue);
}