use rand::prelude::*;
use std::collections::VecDeque;
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
    let mut display_words: VecDeque<&str> = VecDeque::new();
    for i in 0..=2 {
        display_words.push_back(words[i]);
    }
    let mut next_word_index = 3;

    while written_words < 30 && next_word_index < words.len() - 1 {
        println!(
            "{} ::: {} ::: {}",
            display_words[0], display_words[1], display_words[2]
        );
        user_input.clear();
        io::stdin().read_line(&mut user_input)?;
        user_input = user_input.trim().to_string();

        match display_words.front() {
            Some(front_string) => {
                if front_string.eq(&user_input) {
                    written_words += 1;
                    display_words.pop_front();
                    display_words.push_back(words[next_word_index]);
                    next_word_index += 1;
                }
            }
            None => {}
        }
    }
    let elapsed = timer.elapsed();
    println!("Time: {}.{}s", elapsed.as_secs(), elapsed.subsec_millis());

    Ok(())
}
