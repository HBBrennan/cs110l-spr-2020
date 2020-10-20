// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate itertools;
extern crate rand;
use itertools::Itertools;
use rand::Rng;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn get_input_char() -> Option<char> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.chars().next()
}

fn get_word(secret_word_chars: &[char], guesses: &HashSet<char>) -> Vec<char> {
    let mut ret: Vec<char> = vec!['_'; secret_word_chars.len()];
    for (i, letter) in secret_word_chars.iter().enumerate() {
        if guesses.contains(letter) {
            ret[i] = *letter;
        }
    }
    ret
}

fn main() {
    print!("Welcome to CS110 HangMan");
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    // println!("random word: {}", secret_word);

    // Your code here! :)
    let mut correct_guesses: HashSet<char> = HashSet::new();
    let mut incorrect_guesses: HashSet<char> = HashSet::new();
    let num_letters_to_guess = secret_word_chars.iter().unique().count();
    let mut stdout = io::stdout();

    print!("Welcome to CS110 HangMan");
    while correct_guesses.len() < num_letters_to_guess {
        println!(
            "The word so far is {}",
            itertools::join(get_word(&secret_word_chars, &correct_guesses), "")
        );
        println!(
            "You have guessed the following letters: {}{}",
            itertools::join(&incorrect_guesses, ""),
            itertools::join(&correct_guesses, "")
        );
        println!(
            "You have {} guesses left.",
            NUM_INCORRECT_GUESSES - incorrect_guesses.len() as u32,
        );
        print!("Please guess a letter: ");
        stdout.flush().unwrap();
        let maybe_letter = get_input_char();
        match maybe_letter {
            Some(letter) => {
                if correct_guesses.contains(&letter) {
                    println!("Sorry, you already guessed {}", letter)
                } else if !secret_word.contains(letter) {
                    println!("Sorry, that letter is not in the word!");
                    incorrect_guesses.insert(letter);
                } else {
                    correct_guesses.insert(letter);
                }
            }
            None => println!("Please enter a guess."),
        };
        if correct_guesses.len() == num_letters_to_guess {
            println!(
                "\nCongratulations you guessed the secret word: {}!",
                secret_word
            )
        }
        if NUM_INCORRECT_GUESSES == incorrect_guesses.len() as u32 {
            println!("Sorry, you ran out of guesses!");
            break;
        }
        println!();
    }
}
