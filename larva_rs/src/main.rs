use std::fs;
fn main() {
    let img = fs::read_to_string("image.txt").expect("File not found");
    let img = img.trim();
    larva::convert(img);
}