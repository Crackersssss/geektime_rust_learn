use std::fs;

fn main() {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";
    println!("Fectching url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    println!("Converting html to markdown...");
    let markdown = html2md::parse_html(&body);
    fs::write(output, markdown.as_bytes()).unwrap();
    println!("Converted to markdown has been saved to {}", output);
}
