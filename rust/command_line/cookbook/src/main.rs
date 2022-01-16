fn main() {
    argument_parsing();
    printing_colored_text_to_the_terminal();
}

fn argument_parsing() {
    use clap::{App, Arg};

    let matches = App::new("My Test Program")
        .version("0.1.0")
        .author("Takayuki Sato <s.yukkiy@gmail.com")
        .about("Teaches argument parsing")
        .arg(
            Arg::new("file")
                .short("f".parse().unwrap())
                .long("file")
                .takes_value(true)
                .help("A cool file"),
        )
        .arg(
            Arg::new("num")
                .short("n".parse().unwrap())
                .long("number")
                .takes_value(true)
                .help("Five less than your favorite number"),
        )
        .get_matches();

    let myfile = matches.value_of("file").unwrap_or("input.txt");
    println!("The file passed is: {}", myfile);

    let num_str = matches.value_of("num");
    match num_str {
        None => println!("No idea what your favorite number is."),
        Some(s) => match s.parse::<i32>() {
            Ok(n) => println!("Your favorite number must be {}.", n + 5),
            Err(_) => println!("That's not a number! {}", s),
        },
    }
}

fn printing_colored_text_to_the_terminal() {
    use ansi_term::Colour;

    println!(
        "This is {} in color, {} in color and {} in color",
        Colour::Red.paint("red"),
        Colour::Blue.paint("blue"),
        Colour::Green.paint("green")
    );
}
