

fn coordinate() -> (i32, i32){
    (5, 4)
}

fn main() {
   
    let (x, y) = coordinate();

    if x > 5 {
        println!("{:?} is greater than 5", x)
    }
    else if x < 5{
        println!("{:?} is less than 5", x)
    }
    else{
        println!("{:?} is equal than 5", x)
    }

    if y > 5 {
        println!("{:?} is greater than 5", y)
    }
    else if y < 5{
        println!("{:?} is less than 5", y)
    }
    else{
        println!("{:?} is equal than 5", y)
    }

}

