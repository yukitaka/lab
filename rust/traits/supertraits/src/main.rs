trait Person {
    fn name(&self) -> String;
}

trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

struct Man;

impl Man {
    fn new() -> Self {
        Self
    }
}

impl Programmer for Man {
    fn fav_language(&self) -> String {
        "rust".to_string()
    }
}

impl Student for Man {
    fn university(&self) -> String {
        "foolish".to_string()
    }
}

impl Person for Man {
    fn name(&self) -> String {
        "Tester".to_string()
    }
}

impl CompSciStudent for Man {
    fn git_username(&self) -> String {
        "tester".to_string()
    }
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username(),
    )
}

fn main() {
    let man = Man::new();
    println!("{}", comp_sci_student_greeting(&man));
}
