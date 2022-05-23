use rand::Rng;
use std::io;

enum GameStatus {
    RUNNING, WON, LOST
}

struct Hangman {
    secret: String,
    mask: String,
    misses_left: i32,
    status: GameStatus
}

impl Hangman {
    pub fn new(secret: &str, misses_left: i32) -> Hangman {
        let mut hangman = Hangman {
            secret: String::from(secret),
            mask: String::new(),
            misses_left,
            status: GameStatus::RUNNING
        };

        let mut rng = rand::thread_rng();
        for i in 0..secret.len() {
            let current_char = &secret[i..i+1];
            if current_char.eq(" "){
                hangman.mask.push_str(" ");
            } else if rng.gen_range(0..10) < 10 {
                hangman.mask.push_str("_");
            } else {
                hangman.mask.push_str(current_char);
            }
        }
        hangman
    }

    pub fn guess_character(&mut self, guess: &str)-> () {
        if guess.len() > 1 {
            println!("guess must have length 1, had length {}", guess.len())
        }
        let mut matched = false;

        for i in 0..self.secret.len() {
            let current_char = &self.secret[i..i+1];
            if current_char.eq(guess) {
                self.mask.replace_range(i..i+1, guess);
                println!("Character found :D");
                self.print_mask();
                matched = true;
                if !self.mask.contains("_") {
                    self.status = GameStatus::WON
                }
            }
        }

        if !matched {
            if self.misses_left == 0{
                self.status = GameStatus::LOST;
            }
            self.misses_left = self.misses_left - 1;
            println!("No character found :( you have {} misses left", self.misses_left)
        }
    }

    pub fn print_mask(&self){
        println!("Mask: {}", self.mask);
    }
}

fn main() {
    // init
    let secret = "hello world";
    let rounds = 10;
    let mut h = Hangman::new(secret, rounds);
    h.print_mask();

    let mut guess = String::new();
    loop {
        match h.status {
            GameStatus::RUNNING => {
                println!("Yor guess: ");
                io::stdin()
                    .read_line(&mut guess)
                    .expect("Failed to read line");

                h.guess_character(&guess.trim());
                guess.clear();
            }
            GameStatus::WON => {
                println!("Congratulations! you have won :D {} misses are still left", h.misses_left);
                break
            }
            GameStatus::LOST => {
                println!("You have lost :(");
                break
            }
        }

    }
}