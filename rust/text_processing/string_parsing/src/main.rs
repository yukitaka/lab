fn main() {
    collect_unicode_graphemes();
}

fn collect_unicode_graphemes() {
    use unicode_segmentation::UnicodeSegmentation;

    let name = "José Guimarães\r\n";
    let graphemes = UnicodeSegmentation::graphemes(name, true).collect::<Vec<&str>>();
    assert_eq!(graphemes[3], "é");
}
