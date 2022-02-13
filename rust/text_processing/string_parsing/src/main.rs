fn main() {
    collect_unicode_graphemes();
    implement_the_from_str_trait_for_a_custom_struct();
}

fn collect_unicode_graphemes() {
    use unicode_segmentation::UnicodeSegmentation;

    let name = "José Guimarães\r\n";
    let graphemes = UnicodeSegmentation::graphemes(name, true).collect::<Vec<&str>>();
    assert_eq!(graphemes[3], "é");
}

use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl FromStr for RGB {
    type Err = std::num::ParseIntError;

    fn from_str(hex_code: &str) -> Result<Self, Self::Err> {
        let r: u8 = u8::from_str_radix(&hex_code[1..3], 16)?;
        let g: u8 = u8::from_str_radix(&hex_code[3..5], 16)?;
        let b: u8 = u8::from_str_radix(&hex_code[5..7], 16)?;
        Ok(RGB { r, g, b })
    }
}

fn implement_the_from_str_trait_for_a_custom_struct() {
    let code: &str = &r"#fa7268";
    match RGB::from_str(code) {
        Ok(rgb) => {
            println!(
                r"The RGB color code is: R: {} G: {} B: {}",
                rgb.r, rgb.g, rgb.b
            );
        }
        Err(_) => {
            println!("{} is not a valid color hex code!", code);
        }
    }
    assert_eq!(
        RGB::from_str(&r"#fa7268").unwrap(),
        RGB {
            r: 250,
            g: 114,
            b: 104,
        }
    );
}
