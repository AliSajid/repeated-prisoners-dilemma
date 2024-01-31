// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * Copyright (c) 2023-2024
// *
// * This project is dual-licensed under the MIT and Apache licenses.
// *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// ** APACHE 2.0 LICENSE
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// *
// * Licensed under the Apache License, Version 2.0 (the "License");
// * you may not use this file except in compliance with the License.
// * You may obtain a copy of the License at
// *
// * http://www.apache.org/licenses/LICENSE-2.0
// *
// * Unless required by applicable law or agreed to in writing, software
// * distributed under the License is distributed on an "AS IS" BASIS,
// * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// * See the License for the specific language governing permissions and
// * limitations under the License.
// *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * * MIT LICENSE
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// *
// * Permission is hereby granted, free of charge, to any person obtaining a copy
// * of this software and associated documentation files (the "Software"), to deal
// * in the Software without restriction, including without limitation the rights
// * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// * copies of the Software, and to permit persons to whom the Software is
// * furnished to do so, subject to the following conditions:
// *
// * The above copyright notice and this permission notice shall be included in all
// * copies or substantial portions of the Software.
// *
// * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// * SOFTWARE.
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

use std::{
    cmp::Ordering,
    io::{
        self,
        Write,
    },
};

use dilemma_tactix_lib::{
    Choice,
    GameGrid,
    GameOptions,
};
use rand::{
    seq::SliceRandom,
    SeedableRng,
};

fn parse_choice(choice: &str) -> Choice {
    match choice {
        "A" => Choice::Atlantis,
        "B" => Choice::Olympus,
        _ => {
            println!("Invalid choice, defaulting to A");

            Choice::Atlantis
        }
    }
}

fn get_computer_choice(seed: Option<u64>) -> Choice {
    let mut rng = rand_chacha::ChaCha12Rng::seed_from_u64(seed.unwrap_or(0));

    let choices = ["A", "B"];

    let choice = choices.choose(&mut rng).unwrap_or_else(|| &choices[0]);

    parse_choice(choice)
}

pub fn game_loop(game_options: GameOptions, game_grid: GameGrid) {
    game_grid.show_grid();

    println!("The choices available to you are: ");

    println!("A: {}", game_options.choice_atlantis());

    println!("B: {}", game_options.choice_olympus());

    let choice = read_user_input("Enter your choice (A or B): ");

    let choice = parse_choice(choice.trim());

    let computer_choice = get_computer_choice(None);

    let result = game_grid.return_score(choice, computer_choice);

    println!("You chose: {choice}");

    println!("The computer chose: {computer_choice}");

    println!(
        "Your Score: {}\nComputer Score: {}",
        result.first(),
        result.second()
    );

    match result.first().cmp(&result.second()) {
        Ordering::Greater => println!("You win!"),
        Ordering::Less => println!("The computer wins!"),
        Ordering::Equal => println!("It's a tie!"),
    }
}

fn read_user_input(prompt: &str) -> String {
    print!("{prompt}");
    match io::stdout().flush() {
        Ok(()) => (),
        Err(e) => eprintln!("Failed to flush stdout: {e}"),
    }

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(e) => eprintln!("Failed to read line: {e}"),
    }

    input
}

fn main() {
    let game_options = GameOptions::builder().build();

    let game_grid = GameGrid::new(game_options);

    println!("Welcome to Dilemma Tactix!");

    loop {
        game_loop(game_options, game_grid);

        let play_again = read_user_input("Play again? (Y/N): ").trim().to_string();

        if play_again.to_lowercase() != "y" {
            break;
        }
    }

    println!("Thanks for playing!");
}
