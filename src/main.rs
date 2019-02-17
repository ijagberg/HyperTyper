extern crate clap;
use clap::{App, Arg};
use rand::prelude::*;
use std::collections::VecDeque;
use std::error::Error;
use std::fs;
use std::io;
use std::io::Write;
use std::{thread, time};

struct Config {
    difficulty: usize,
    username: String,
}

fn main() {
    // Handle command line stuff
    let matches = App::new("HyperTyper")
        .version("1.0")
        .author("Isak Jägberg <ijagberg@gmail.com>")
        .about("Simple command line typing game")
        .arg(
            Arg::with_name("difficulty")
                .short("d")
                .long("difficulty")
                .value_name("INTEGER")
                .help("Sets maximum length of words to display")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("username")
                .short("u")
                .long("username")
                .value_name("USERNAME")
                .help("Sets username to display with scores")
                .takes_value(true),
        )
        .get_matches();

    let config = Config {
        difficulty: match matches.value_of("difficulty") {
            Some(d) => d
                .parse::<usize>()
                .expect("Could not parse value of difficulty"),
            None => 0,
        },
        username: match matches.value_of("username") {
            Some(u) => u.to_string(),
            None => String::from(""),
        },
    };

    start_game(&config);
}

fn start_game(config: &Config) {
    // Get wordlist from file and split into vector
    let contents = fs::read_to_string("wordlist.txt").expect("Could not read file!");
    let words = match get_words(&contents, &config) {
        Ok(words) => words,
        _error => {
            eprintln!("Could not get words from wordlist.txt");
            return;
        }
    };

    // Print welcoming message and countdown
    print_countdown().expect("Could not print countdown message");

    match run(words) {
        Ok(elapsed) => println!(
            "Time: {}.{}s for Username: {}",
            elapsed.as_secs(),
            elapsed.subsec_millis(),
            config.username
        ),
        _error => eprintln!("Some error occurred!"),
    }
}

fn get_words<'a>(contents: &'a String, config: &Config) -> Result<Vec<&'a str>, Box<dyn Error>> {
    let mut words = Vec::new();
    let mut rng = rand::thread_rng();

    for line in contents.lines() {
        if config.difficulty <= 0 || line.len() <= config.difficulty {
            words.push(line);
        }
    }
    words.shuffle(&mut rng);

    Ok(words)
}

fn print_countdown() -> Result<(), Box<Error>> {
    println!("Welcome to HyperTyper!");

    let mut seconds_left = 3;
    while seconds_left > 0 {
        print!("\rStarting in... {}", seconds_left);
        io::stdout().flush()?;
        thread::sleep(time::Duration::from_secs(1));
        seconds_left -= 1;
    }
    println!("\rGo!                      ");
    thread::sleep(time::Duration::from_secs(1));

    Ok(())
}

fn run(words: Vec<&str>) -> Result<std::time::Duration, io::Error> {
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

    Ok(elapsed)
}
