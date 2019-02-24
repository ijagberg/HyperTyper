extern crate clap;
use clap::{App, Arg};
use rand::prelude::*;
use std::error::Error;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::{thread, time};

struct Config {
    difficulty: usize,
    username: String,
    word_count: usize,
    word_list_file: PathBuf,
    display_words_count: usize,
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
                .help("Sets maximum length of words to display")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("username")
                .short("u")
                .long("username")
                .help("Sets username to display with scores")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("word-count")
                .short("w")
                .long("word-count")
                .help("Sets the number of words to type in one round")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("word-list-file")
                .long("word-list-file")
                .help("File containing a custom word list")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("display-words-count")
                .long("display-words-count")
                .help("Number of words to display")
                .takes_value(true),
        )
        .get_matches();

    let config = Config {
        difficulty: match matches.value_of("difficulty") {
            Some(d) => d
                .parse::<usize>()
                .expect("Could not parse integer value of argument difficulty (-d)"),
            None => 0,
        },
        username: match matches.value_of("username") {
            Some(u) => u.to_string(),
            None => String::from(""),
        },
        word_count: match matches.value_of("word-count") {
            Some(w) => w
                .parse::<usize>()
                .expect("Could not parse integer value of argument word-count (-w)"),
            None => 15,
        },
        word_list_file: match matches.value_of("word-list-file") {
            Some(path_string) => PathBuf::from(path_string),
            None => {
                let mut path = PathBuf::new();
                path.push("wordlist.txt");
                path
            }
        },
        display_words_count: match matches.value_of("display-words-count") {
            Some(i) => i
                .parse::<usize>()
                .expect("Could not parse integer value of argument display-words-count"),
            None => 3,
        },
    };

    start_game(config);
}

fn start_game(config: Config) {
    // Get wordlist from file and split into vector
    let contents = fs::read_to_string(&config.word_list_file).expect("Could not read file!");
    let words = match get_words(&contents, &config) {
        Ok(words) => {
            if words.len() >= config.display_words_count {
                words
            } else {
                eprintln!(
                    "Fewer than {} words in file {}/{}",
                    config.display_words_count,
                    config.word_list_file.parent().unwrap().to_str().unwrap(),
                    config.word_list_file.file_name().unwrap().to_str().unwrap(),
                );
                return;
            }
        }
        _error => {
            eprintln!(
                "Could not get words from {}/{}",
                config.word_list_file.parent().unwrap().to_str().unwrap(),
                config.word_list_file.file_name().unwrap().to_str().unwrap(),
            );
            return;
        }
    };

    // Print welcoming message and countdown
    print_countdown().expect("Could not print countdown message");

    match run(&config, words) {
        Ok(elapsed) => {
            if config.username.is_empty() {
                println!("Time: {:?}", elapsed);
            } else {
                println!("Time: {:?} for user: {}", elapsed, config.username);
            }
        }
        Err(_) => eprintln!("Some error occurred!"),
    }
}

fn get_words<'a>(contents: &'a str, config: &Config) -> Result<Vec<&'a str>, Box<dyn Error>> {
    let mut words = Vec::new();
    let mut rng = rand::thread_rng();

    for line in contents.lines() {
        if config.difficulty == 0 || line.len() <= config.difficulty {
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

fn print_display_words(display_words: &Vec<&str>) {
    println!("#####");
    for display_word in display_words {
        println!("{}", display_word);
    }
    println!("#####");
}

fn run(config: &Config, words: Vec<&str>) -> Result<std::time::Duration, io::Error> {
    let mut written_words = 0;
    let mut user_input = String::new();

    // Add the first three words
    let mut display_words = vec![""; config.display_words_count];
    display_words.clone_from_slice(&words[..config.display_words_count]);

    let mut next_word_index = config.display_words_count + 1;

    let timer = time::Instant::now();
    while written_words < config.word_count && next_word_index < words.len() - 1 {
        print_display_words(&display_words);

        user_input.clear();
        io::stdin().read_line(&mut user_input)?;
        user_input = user_input.trim().to_string();

        // iterate over display words to check for a match
        for (index, display_word) in display_words.iter().enumerate() {
            if display_word.eq(&user_input) {
                written_words += 1;
                display_words[index] = words[next_word_index];
                next_word_index += 1;
                break;
            }
        }
    }
    let elapsed = timer.elapsed();

    Ok(elapsed)
}
