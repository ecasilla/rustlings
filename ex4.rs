// Make me compile!

fn something() -> Result<i32, std::num::ParseIntError> {
    let x:i32 = "3".parse().expect("Error Parsing str to i32");
    Ok(x * 4)
}

fn main() {
    match something() {
        Ok(..) => println!("You win!"),
        Err(e) => println!("Oh no something went wrong: {}", e),
    }
}
