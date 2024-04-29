use std::collections::BTreeMap;
use std::io;
use std::fs;

fn main() {
    let stdin = io::stdin();
    let input = &mut String::new();

    let file_path = "./words_alpha.txt";
    let wordlist = fs::read_to_string(file_path)
        .expect("Could not read the file.");

    //println!("With text:\n{wordlist}");

    println!("What are the available letters?");
    let _ = stdin.read_line(input);
    //getting Vec of chars from string
    let mut letters: Vec<char> = input.chars().collect();
    letters.pop();

    println!("Which letter is in the center?");
    input.clear();
    let _ = stdin.read_line(input);
    let center_input: Vec<char> = input.chars().collect();
    let must_use = center_input[0];
    let mut petal_letters: Vec<char> = vec![];
    
    for &letter in &letters {
        if letter != must_use {petal_letters.push(letter);}
    }

    let mut valid_wordlist: Vec<&str> = vec![];
    let mut word_scores: BTreeMap<i32, &str> = BTreeMap::new();

    //first for loop to find the valid wordlist from the letters given
    for word in wordlist.split_whitespace() {
        let mut valid = true;
        if word.len() as i32 > 3 {
            for letter_to_check in word.chars() {
                if !(word.contains(must_use) && letters.contains(&letter_to_check)) {
                    valid = false;
                    break;
                }
            }
            if valid {
                valid_wordlist.push(word);
            }
        }
    }
    
    for petal in petal_letters {
        println!("\nWord scores when {} is glowing:", petal);
        for word in &valid_wordlist {
            let mut score = score_word(word, petal);
            if pangram(word, letters.clone()) {score += 7;};
            word_scores.insert(score, word);
        }

        println!("{:?}", word_scores);
        word_scores.clear();
    }    

}

fn score_word(word: &str, glowing: char) -> i32 {
    let mut score = 0;
    match word.len() as i32 {
        4 => score += 2,
        5 => score += 4,
        6 => score += 6,
        7 => score += 12,
        _ => score += 12 + 3*(word.len() as i32 - 7),
    }

    if word.contains(glowing) {
        let occurences = word.chars().filter(|c| *c == glowing).count() as i32;
        score += 5*occurences;
    }

    return score;
}

fn pangram(word: &str, letters: Vec<char>) -> bool {
    for letter in letters {
        if !word.contains(letter) {return false;}
    }
    return true;
}