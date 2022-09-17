use std::{error::Error, io::Write};

use wordle::*;

fn main() -> Result<(), Box<dyn Error>> {
    print!("Guess:  ");
    std::io::stdout().flush()?;

    let mut guess = String::with_capacity(5);
    std::io::stdin().read_line(&mut guess)?;

    let guess = guess.trim().parse()?;

    let answer = "gerne".parse()?;

    let mask = WMask::compare(guess, answer);

    println!("Answer: {answer}\n{mask}");

    assert!(mask.matches(answer));

    Ok(())
}
