// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// * Copyright (c) 2023
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
// *     http://www.apache.org/licenses/LICENSE-2.0
// *
// * Unless required by applicable law or agreed to in writing, software
// * distributed under the License is distributed on an "AS IS" BASIS,
// * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// * See the License for the specific language governing permissions and
// * limitations under the License.
// *
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// ** MIT LICENSE
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

use dilemma_tactix_lib::GameGrid;
use prettytable::{format::Alignment, Cell, Row, Table};

fn display_game_grid(game_grid: &GameGrid) {
    let mut table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new(""),
        Cell::new_align("Player 2", Alignment::CENTER).with_hspan(2),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Player 1"),
        Cell::new(game_grid.choice_aleph()),
        Cell::new(game_grid.choice_bey()),
    ]));
    table.add_row(Row::new(vec![
        Cell::new(game_grid.choice_aleph()),
        Cell::new(&game_grid.score_aa().to_string()),
        Cell::new(&game_grid.score_ab().to_string()),
    ]));
    table.add_row(Row::new(vec![
        Cell::new(game_grid.choice_bey()),
        Cell::new(&game_grid.score_ba().to_string()),
        Cell::new(&game_grid.score_bb().to_string()),
    ]));
    table.printstd();
}

fn parse_choice(choice: &str, game_grid: &GameGrid) -> String {
    match choice {
        "A" => game_grid.choice_aleph().to_owned(),
        "B" => game_grid.choice_bey().to_owned(),
        _ => {
            println!("Invalid choice, defaulting to A");
            game_grid.choice_aleph().to_owned()
        }
    }
}

fn main() {
    let game_grid = GameGrid::default();
    display_game_grid(&game_grid);

    println!("The choices available to you are: ");
    println!("A: {}", game_grid.choice_aleph());
    println!("B: {}", game_grid.choice_bey());
    println!("Enter your choice (A or B): ");
    let mut choice_aleph = String::new();
    std::io::stdin()
        .read_line(&mut choice_aleph)
        .expect("Failed to read line");
    let choice_aleph = parse_choice(choice_aleph.trim(), &game_grid);

    println!("Enter opponents choice (A or B): ");
    let mut choice_bey = String::new();
    std::io::stdin()
        .read_line(&mut choice_bey)
        .expect("Failed to read line");
    let choice_bey = parse_choice(choice_bey.trim(), &game_grid);

    let result = game_grid.play(&choice_aleph, &choice_bey);

    if (result.first() == game_grid.score_aa().first())
        && (result.second() == game_grid.score_aa().second())
    {
        println!(
            "Both players both chose {}. Player 1 scored {}, Player 2 scored {}",
            game_grid.choice_aleph(),
            game_grid.score_aa().first(),
            game_grid.score_aa().second()
        );
    } else if (result.first() == game_grid.score_ab().first())
        && (result.second() == game_grid.score_ab().second())
    {
        println!(
            "Player 1 chose {}, Player 2 chose {}. Player 1 scored {}, Player 2 scored {}",
            game_grid.choice_aleph(),
            game_grid.choice_bey(),
            game_grid.score_ab().first(),
            game_grid.score_ab().second()
        );
    } else if (result.first() == game_grid.score_ba().first())
        && (result.second() == game_grid.score_ba().second())
    {
        println!(
            "Player 1 chose {}, Player 2 chose {}. Player 1 scored {}, Player 2 scored {}",
            game_grid.choice_bey(),
            game_grid.choice_aleph(),
            game_grid.score_ba().first(),
            game_grid.score_ba().second()
        );
    } else if (result.first() == game_grid.score_bb().first())
        && (result.second() == game_grid.score_bb().second())
    {
        println!(
            "Both players chose {}, Player 1 scored {}, Player 2 scored {}",
            game_grid.choice_bey(),
            game_grid.score_bb().first(),
            game_grid.score_bb().second()
        );
    } else {
        println!("Something went wrong");
    }
}
