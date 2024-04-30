use std::io;
use std::fs;

fn main() {
    let stdin = io::stdin();
    let input = &mut String::new();

    let file_path = "./words_alpha.txt";
    let wordlist = fs::read_to_string(file_path)
        .expect("Could not read the file.");

    //println!("With text:\n{wordlist}");

    //prompt and parse the 7 letters available
    //e.g. "abcdefg"
    println!("What are the available letters?");
    let _ = stdin.read_line(input);
    //getting Vec of chars from string
    let mut letters: Vec<char> = input.chars().collect();

    //prompt and parse the center letter, which must be used in every word
    //e.g. "a"
    println!("Which letter is in the center?");
    input.clear();
    let _ = stdin.read_line(input);
    let center_input: Vec<char> = input.chars().collect();
    let must_use = center_input[0];

    //create a new Vec<char> that has all letters except the center
    let mut petal_letters: Vec<char> = vec![];
    for letter in letters.clone() {
        if !letter.is_alphabetic() {letters.pop();}
        else if letter != must_use {petal_letters.push(letter);}
    }

    //string vec to hold valid words found
    let mut valid_wordlist: Vec<&str> = vec![];

    //binary tree to hold words sorted by their score
    let mut word_scores: Vec<(&str, i32)> = Vec::new();

    //first for loop to find the valid wordlist from the letters given
    for word in wordlist.split_whitespace() {
        let mut valid = true;

        //only check words with 4 or more letters
        if (word.len() as i32 > 3) && word.contains(must_use) {
            for letter_to_check in word.chars() {
                //words are only valid if they are some combination of the 7 letters
                if !letters.contains(&letter_to_check) {
                    valid = false;
                    break;
                }
            }

            //add word to valid list if it satisfies all conditions checked
            if valid {
                valid_wordlist.push(word);
            }
        }
    }
    
    //second for loop to calculate the scores of words when each petal is glowing
    for petal in petal_letters {
        println!("\nWord scores when {} is glowing:", petal);
        for word in &valid_wordlist {
            //calculate initial score
            let mut score = score_word(word, petal);

            //add 7 if each letter is used
            if pangram(word, &letters) {score += 7;};

            //add score and word pair to the binary tree map
            if score > 30 {word_scores.push((word, score));}
            
        }

        word_scores.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

        println!("{:?}", word_scores);
        word_scores.clear();
    }


}

//calculates score of given word with bonus point character
fn score_word(word: &str, glowing: char) -> i32 {
    let mut score = 0;

    match word.len() as i32 {
        4 => score += 2, //lowest score possible
        5 => score += 4,
        6 => score += 6,
        7 => score += 12,
        _ => score += 12 + 3*(word.len() as i32 - 7), //each letter above 7 adds 3 points
    }

    //count occurences of the bonus letter and add 5 to the score for each
    if word.contains(glowing) {
        let occurences = word.chars().filter(|c| *c == glowing).count() as i32;
        score += 5*occurences;
    }

    return score;
}

//checks if each character in letters is used in the given word
fn pangram(word: &str, letters: &Vec<char>) -> bool {
    for letter in letters {
        if !word.contains(*letter) {return false;}
    }
    return true;
}