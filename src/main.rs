use std::{error::Error, time::Instant};

use wordle::{
    load_wordlist,
    scoring::{apply_mask, calc_guess_info, find_best_word, find_best_word_set},
    word, Lang, WMask,
};

fn _main() -> Result<(), Box<dyn Error>> {
    let lang = Lang::DE;
    let wordlist = load_wordlist(lang.wordlist())?;
    let sollist = load_wordlist(lang.sollist())?;

    for (word, score) in find_best_word_set(&wordlist, &sollist) {
        println!("{score:.1}\t{word}");
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let answer = word!(W I E S O);
    let guesses = [
        word!(R A T E N),
        word!(B I L D E),
        word!(S I E G S),
        answer
    ];

    let lang = Lang::DE;
    let wordlist = load_wordlist(lang.wordlist())?;
    let mut strict_wordlist = wordlist.clone();
    let mut sollist = load_wordlist(lang.sollist())?;

    let start_time = Instant::now();

    for (i, guess) in guesses.into_iter().enumerate() {
        println!("========== Guess {} ==========", i + 1);
        if guess == answer {
            println!("You got it!");
            break;
        }

        println!("----- Your Guess -----");
        println!("Word: {guess}");
        let mask = WMask::compare(guess, answer);
        println!("Colors: {mask}");
        let info = calc_guess_info(&sollist, mask);
        println!("Info: {info:.2}");

        if i > 0 {
            println!("\n----- Optimal Guess -----");
            let (best_guess, _best_info) = find_best_word(&wordlist, &sollist);
            println!("Word: {best_guess}");
            let best_mask = WMask::compare(best_guess, answer);
            println!("Colors: {best_mask}");
            let best_info = calc_guess_info(&sollist, best_mask);
            println!("Info: {best_info:.2}");

            println!("\n----- Strict Optimal Guess -----");
            if wordlist.len() == strict_wordlist.len() {
                println!("(same as Optimal)");
            } else {
                let (best_guess, _best_info) = find_best_word(&strict_wordlist, &sollist);
                println!("Word: {best_guess}");
                let best_mask = WMask::compare(best_guess, answer);
                println!("Colors: {best_mask}");
                let best_info = calc_guess_info(&sollist, best_mask);
                println!("Info: {best_info:.2}");
            }
        }

        println!();

        apply_mask(&mut strict_wordlist, mask);
        apply_mask(&mut sollist, mask);
    }

    println!("\nCalculation completed in {:.1?}", start_time.elapsed());

    Ok(())
}
