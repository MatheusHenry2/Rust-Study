enum Flavor {
    Doce,
    Salgada,
}

struct Drink {
    fluid: f64,
    flavor: Flavor,
}


fn print_drink(drink: Drink) {

    match drink.flavor {
        Flavor:: Doce => println!("doce"),
        Flavor:: Salgada => println!("salgada"),
    }

    println!("fluid : {:?}", drink.fluid);
}

fn main() {
   let sweet = Drink {
        flavor: Flavor::Doce,
        fluid: 6.0
   };

   print_drink(sweet);
}