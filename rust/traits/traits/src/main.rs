#[derive(Clone)]
struct Sheep {
    naked: bool,
    name: &'static str,
}

trait Animal {
    fn new(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep { name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }

    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

#[derive(Clone)]
struct Dog {
    name: &'static str,
}

impl Animal for Dog {
    fn new(name: &'static str) -> Dog {
        Dog { name }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        "woof!"
    }
}

struct Owner<T: Animal> {
    pet: Pet<T>,
}

enum Pet<T: Animal> {
    Some(Box<T>),
    None,
}

impl<T: Animal + Clone> Owner<T> {
    fn new() -> Self {
        Owner { pet: Pet::None }
    }

    fn keep(mut self, one: T) {
        println!("We keep a {}", one.name());
        println!("{}", one.noise());
        self.pet = Pet::Some(Box::new(one));
    }
}

fn main() {
    let mut dolly: Sheep = Animal::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();

    let owner = Owner::new();
    owner.keep(dolly);

    let mocha: Dog = Animal::new("Mocha");
    mocha.talk();
    let owner = Owner::new();
    owner.keep(mocha);
}
