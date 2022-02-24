mod argument_type;
mod return_type;

fn main() {
    if let Err(e) = argument_type::return_type() {
        println!("{}", e);
    }
    return_type::return_type();
    return_type::add_closure();
    return_type::impl_closure();
}
