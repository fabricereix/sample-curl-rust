use std::io::{stdout, Write};

use curl::easy::Easy;

fn main() {
    // Write the contents of rust-lang.org to stdout
    let mut easy = Easy::new();
    easy.url("https://www.rust-lang.org/").unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    })
    .unwrap();
    easy.perform().unwrap();
    println!("Hello, world!");
}
