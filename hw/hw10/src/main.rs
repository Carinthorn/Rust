mod lib;
use lib::{vflip, hflip, vcat, hcat};

fn main() {
    let data = [
"<--",
"#####",
"<=="
].map(|v| v.to_string());
    let result = hcat(&data[..2], &data);
    println!("{:?}", result);
}
