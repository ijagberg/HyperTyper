use rand::prelude::*;
use std::error::Error;
use std::fs;
use std::io;
use std::io::Write;
use std::{thread, time};

fn main() {
    // Handle command line stuff
    start_game();
}

fn start_game() {
    // Get wordlist from file and split into vector
    let contents = fs::read_to_string("wordlist.txt").expect("Could not read file!");
    let words = match get_words(&contents) {
        Ok(words) => words,
        _error => {
            eprintln!("Could not get words from wordlist.txt");
            return;
        }
    };

    // Print welcoming message and countdown
    print_countdown().expect("Could not print countdown message");

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
    let mut rng = rand::thread_rng();

    for line in contents.lines() {
        words.push(line);
    }
    words.shuffle(&mut rng);

    Ok(words)
}

fn print_countdown() -> Result<(), Box<Error>> {
    println!("Welcome to HyperTyper!");

    let one_second = time::Duration::from_secs(1);
    let mut seconds_left = 3;

    while seconds_left > 0 {
        print!("\rStarting in... {}", seconds_left);
        io::stdout().flush()?;
        thread::sleep(one_second);
        seconds_left -= 1;
    }
    println!("\rGo!                      ");
    thread::sleep(one_second);

    Ok(())
}

fn run(words: Vec<&str>) -> Result<(), io::Error> {
    let mut written_words = 0;
    let timer = time::Instant::now();
    let mut user_input = String::new();

    // Add the first three words
    let mut display_words: [&str; 3] = [""; 3];
    for i in 0..=2 {
        display_words[i] = words[i];
    }
    let mut next_word_index = 3;

    while timer.elapsed().as_secs() < 15 && next_word_index < words.len() - 1 {
        println!(
            "{} ::: {} ::: {}",
            display_words[0], display_words[1], display_words[2]
        );
        user_input.clear();
        io::stdin().read_line(&mut user_input)?;
        user_input = user_input.trim().to_string();

        // Check if display_words contains user_input
        for i in 0..display_words.len() {
            if display_words[i].eq(&user_input) {
                written_words += 1;
                display_words[i] = words[next_word_index];
                next_word_index += 1;
            }
        }
    }
    println!("Time: {}, words: {}", timer.elapsed().as_secs(), written_words);

    Ok(())
}
