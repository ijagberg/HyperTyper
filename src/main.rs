use std::error::Error;
use std::fs;
use std::io;
use std::io::Write;
use std::{thread, time};

fn main() {
    let contents = fs::read_to_string("wordlist.txt").expect("Could not read file!");

    let words = match get_words(&contents) {
        Ok(words) => words,
        _error => {
            eprintln!("Could not get words from wordlist.txt");
            return;
        }
    };
    match run(words) {
        Ok(_) => return,
        _error => {
            eprintln!("Some error occurred!");
            return;
        }
    }
}

fn get_words<'a>(contents: &'a String) -> Result<Vec<&'a str>, Box<dyn Error>> {
    let mut words = Vec::new();

    for line in contents.lines() {
        words.push(line);
    }

    Ok(words)
}

fn run<'a>(words: Vec<&'a str>) -> Result<(), io::Error> {
    println!("Welcome to HyperTyper!");

    let one_second = time::Duration::from_secs(1);
    let mut seconds_left = 3;

    while seconds_left > 0 {
        print!("\rStarting in... {}", seconds_left);
        io::stdout().flush()?;
        thread::sleep(one_second);
        seconds_left -= 1;
    }
    println!("\rGo!                   ");
    thread::sleep(one_second);

    for word in words {
        println!("{}", word);
        thread::sleep(one_second);
    }

    Ok(())
}
