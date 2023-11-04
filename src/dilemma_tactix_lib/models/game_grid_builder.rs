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

use crate::{GameGrid, NumberPair};

#[derive(Debug, Default)]
pub struct GameGridBuilder {
    max_value: Option<u32>,
    min_value: Option<u32>,
    choice_aleph: Option<String>,
    choice_bey: Option<String>,
    score_aa: Option<NumberPair>,
    score_ab: Option<NumberPair>,
    score_ba: Option<NumberPair>,
    score_bb: Option<NumberPair>,
}

impl GameGridBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            max_value: None,
            min_value: None,
            choice_aleph: None,
            choice_bey: None,
            score_aa: None,
            score_ab: None,
            score_ba: None,
            score_bb: None,
        }
    }

    #[must_use]
    pub const fn max_value(mut self, max_value: u32) -> Self {
        self.max_value = Some(max_value);
        self
    }

    #[must_use]
    pub const fn min_value(mut self, min_value: u32) -> Self {
        self.min_value = Some(min_value);
        self
    }

    #[must_use]
    pub fn choice_aleph(mut self, choice_aleph: String) -> Self {
        self.choice_aleph = Some(choice_aleph);
        self
    }

    #[must_use]
    pub fn choice_bey(mut self, choice_bey: String) -> Self {
        self.choice_bey = Some(choice_bey);
        self
    }

    #[must_use]
    pub const fn score_aa(mut self, score_aa: NumberPair) -> Self {
        self.score_aa = Some(score_aa);
        self
    }

    #[must_use]
    pub const fn score_ab(mut self, score_ab: NumberPair) -> Self {
        self.score_ab = Some(score_ab);
        self
    }

    #[must_use]
    pub const fn score_ba(mut self, score_ba: NumberPair) -> Self {
        self.score_ba = Some(score_ba);
        self
    }

    #[must_use]
    pub const fn score_bb(mut self, score_bb: NumberPair) -> Self {
        self.score_bb = Some(score_bb);
        self
    }

    #[must_use]
    pub fn build(self) -> GameGrid {
        GameGrid::new(
            self.max_value.expect("max_value must be set"),
            self.min_value.expect("min_value must be set"),
            self.choice_aleph.expect("choice_aleph must be set"),
            self.choice_bey.expect("choice_bey must be set"),
            self.score_aa.expect("score_aa must be set"),
            self.score_ab.expect("score_ab must be set"),
            self.score_ba.expect("score_ba must be set"),
            self.score_bb.expect("score_bb must be set"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_grid_builder() {
        let game_grid = GameGridBuilder::new()
            .max_value(10)
            .min_value(1)
            .choice_aleph("A".to_string())
            .choice_bey("B".to_string())
            .score_aa(NumberPair::new(1, 1))
            .score_ab(NumberPair::new(1, 1))
            .score_ba(NumberPair::new(1, 1))
            .score_bb(NumberPair::new(1, 1))
            .build();

        assert_eq!(game_grid.max_value(), 10);
        assert_eq!(game_grid.min_value(), 1);
        assert_eq!(game_grid.choice_aleph(), "A");
        assert_eq!(game_grid.choice_bey(), "B");
        assert_eq!(game_grid.score_aa(), NumberPair::new(1, 1));
        assert_eq!(game_grid.score_ab(), NumberPair::new(1, 1));
        assert_eq!(game_grid.score_ba(), NumberPair::new(1, 1));
        assert_eq!(game_grid.score_bb(), NumberPair::new(1, 1));
    }
}
