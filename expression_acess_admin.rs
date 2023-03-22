
enum Acess {
    Admin, 
    Manager,
    User,
    Guest
}



fn main() {
   
    //secret file: admin only
    let access_level = Acess::Guest;
    let can_access_file = match access_level{
        Acess::Admin => true,
        _ => false,
    };
    println!("{:?}", can_access_file)

}

