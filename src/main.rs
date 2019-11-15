use rand::prelude::*;
use std::error::Error;
use std::fs;
use std::io;
use std::io::Write;
use std::{thread, time};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Config {
    /// Minimum length of words to display
    ///
    /// There is no limit by default
    #[structopt(short, long)]
    difficulty: Option<usize>,

    /// The player's username
    #[structopt(short, long)]
    username: Option<String>,

    /// Number of words in a round
    #[structopt(short, long, default_value = "10")]
    word_count: usize,

    /// Number of words to display
    #[structopt(long, default_value = "3")]
    display_words_count: usize,

    /// File to use as source for words
    #[structopt(long, default_value = "wordlist.txt")]
    word_list_file: String,
}

fn main() {
    let config = Config::from_args();

    start_game(config);
}

fn start_game(config: Config) {
    // Get wordlist from file and split into vector
    let contents = fs::read_to_string(&config.word_list_file).expect("could not read file");
    let words = get_words(&contents, &config);

    // Print welcoming message and countdown
    print_countdown().expect("Could not print countdown message");

    match run(&config, words) {
        Ok(elapsed) => match config.username {
            None => println!("Time: {:?}", elapsed),
            Some(username) => println!("Time: {:?} for user: {}", elapsed, username),
        },
        Err(_) => eprintln!("Some error occurred"),
    }
}

fn get_words<'a>(contents: &'a str, config: &Config) -> Vec<&'a str> {
    let mut words = Vec::new();
    let mut rng = rand::thread_rng();

    for line in contents.lines() {
        if config.difficulty.is_none() || line.len() <= config.difficulty.unwrap() {
            words.push(line);
        }
    }
    words.shuffle(&mut rng);
    words
}

fn print_countdown() -> Result<(), Box<dyn Error>> {
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

fn print_display_words(display_words: &[&str]) {
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
            if display_word == &user_input {
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
