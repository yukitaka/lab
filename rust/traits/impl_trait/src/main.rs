mod argument_type;

fn main() {
    if let Err(e) = argument_type::return_type() {
        println!("{}", e);
    }
}
