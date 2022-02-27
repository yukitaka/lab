fn main() {
    drink("water");
    drink("lemonade");
}

fn drink(beverage: &str) {
    if beverage == "lemonade" {
        panic!("AAAaaaaa!!!!");
    }

    println!("Some refreshing {} is all I need.", beverage);
}
