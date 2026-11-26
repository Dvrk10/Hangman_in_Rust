use rand::seq::IndexedRandom;
use std::io;
use std::io::Write;
use std::process::exit;
use std::thread;
use std::time::Duration;

fn display_word(word_list: &[String]) -> Option<String> {
    let mut rng = rand::rng();

    if let Some(random_word) = word_list.choose(&mut rng) {
        Some(random_word.clone())
    } else {
        println!("You didnt enter any words!");
        None
    }
}
fn display_vector(word_list: &[String]) {
    for word in word_list {
        println!("{}", word);
    }
}

fn gamehost_input() -> Vec<String> {
    println!(
        "Write words that will be randomly chosen for the game and type 'done' when you're finished or 'exit' to stop the program"
    );
    let mut word_list: Vec<String> = Vec::new();
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let input = input.trim().to_lowercase();

        if input.is_empty() {
            println!("You must enter at least one word!");
            continue;
        }

        if input == "done" {
            if word_list.is_empty() {
                println!("The list must contain at least one word!");
                continue;
            }
            loop {
                println!("Do you want to display the list for 5s before you begin? Y/N");
                let mut confirmation = String::new();
                io::stdin()
                    .read_line(&mut confirmation)
                    .expect("Failed to read line");
                let confirmation = confirmation.trim().to_lowercase();
                if confirmation == "y" {
                    display_vector(&word_list);
                    thread::sleep(Duration::from_secs(5));
                    print!("\x1B[2J\x1B[1;1H");
                    io::stdout().flush().unwrap();
                    return word_list;
                } else if confirmation == "n" {
                    print!("\x1B[2J\x1B[1;1H");
                    io::stdout().flush().unwrap();
                    return word_list;
                } else {
                    println!("Please input Y or N!");
                    continue;
                }
            }
        }

        if input == "exit" {
            std::process::exit(0);
        }

        if !input.is_empty() {
            (&mut word_list).push(input);
        }
    }
}

fn player_input() -> Vec<char> {
    loop {
        let mut p_input = String::new();
        io::stdin()
            .read_line(&mut p_input)
            .expect("Failed to read line.");

        let p_input = p_input.trim().to_lowercase();
        if !p_input.is_empty() {
            return p_input.chars().collect();
        } else {
            println!("there was no input!");
        }
    }
}
fn check_guess(
    p_input: Vec<char>,
    rand_word: &Option<String>,
    hidden_word: &mut Vec<char>,
) -> bool {
    if let Some(word) = rand_word {
        if p_input.len() == 1 {
            let mut found = false;
            for (i, c) in word.chars().enumerate() {
                if c == p_input[0] {
                    hidden_word[i] = c;
                    found = true;
                }
            }
            return found;
        } else if word.chars().eq(p_input.iter().copied()) {
            *hidden_word = word.chars().collect();
            return true;
        }
    }
    return false;
}

fn main() {
    let word_list = gamehost_input();
    let rand_word = display_word(&word_list);
    let mut wrong_guess: usize = 0;
    let mut has_been_input_before: Vec<String> = Vec::new();
    let hangman: [&str; 7] = [
        r#"+---+
  |   |
      |
      |
      |
      |
========="#,
        r#"  +---+
  |   |
  O   |
      |
      |
      |
========="#,
        r#"  +---+
  |   |
  O   |
  |   |
      |
      |
========="#,
        r#"  +---+
  |   |
  O   |
 /|   |
      |
      |
========="#,
        r#"  +---+
  |   |
  O   |
 /|\  |
      |
      |
========="#,
        r#"  +---+
  |   |
  O   |
 /|\  |
 /    |
      |
========="#,
        r#"  +---+
  |   |
  O   |
 /|\  |
 / \  |
      |
========="#,
    ];

    if let Some(word) = &rand_word {
        let mut hidden_word = vec!['_'; word.chars().count()];
        loop {
            println!("{}", hidden_word.iter().collect::<String>());
            println!("Enter your guess:");

            let player_input = player_input();

            if has_been_input_before.contains(&player_input.iter().collect::<String>()) {
                println!("You have tried this before already!");
                continue;
            }

            if player_input.len() == 1 {
                has_been_input_before.push(player_input[0].to_string());
            } else {
                (&mut has_been_input_before).push(player_input.iter().collect::<String>());
            }

            let t_f_guess = check_guess(player_input, &rand_word, &mut hidden_word);

            if !hidden_word.contains(&'_') {
                println!("You won! The word was: {}", word);
                break;
            }

            if t_f_guess {
                continue;
            } else {
                println!("You guessed wrong!");
                println!("{}", hangman[wrong_guess]);
                wrong_guess += 1;
            }
            if wrong_guess == 7 {
                print!("You've lost, the word was: {}", word);
                exit(0);
            }
        }
    }
}
