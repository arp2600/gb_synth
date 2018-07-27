use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    println!("{}", input);
    Ok(())
}
