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

use rand::{self, Rng};

pub struct ChoiceNameOptions {
    choice_a_options: [&'static str; 17],
    choice_b_options: [&'static str; 17],
    length: usize,
}

impl ChoiceNameOptions {
    #[must_use]
    pub const fn new() -> Self {
        let choice_aleph_options = [
            "cooperate",
            "swerve",
            "macro",
            "fight",
            "bet",
            "raise_price",
            "opera",
            "go",
            "heads",
            "particle",
            "discrete",
            "peace",
            "search",
            "lead",
            "accept",
            "accept",
            "attack",
        ];
        let choice_bey_options = [
            "defect",
            "straight",
            "micro",
            "back_down",
            "fold",
            "lower_price",
            "football",
            "stay",
            "tails",
            "wave",
            "continuous",
            "war",
            "evaluate",
            "follow",
            "reject",
            "deny",
            "decay",
        ];
        let length = choice_aleph_options.len();
        Self {
            choice_a_options: choice_aleph_options,
            choice_b_options: choice_bey_options,
            length,
        }
    }

    #[must_use]
    pub const fn get_choice_pair(&self, n: usize) -> (&'static str, &'static str) {
        (self.choice_a_options[n], self.choice_b_options[n])
    }

    #[must_use]
    pub fn get_random_pair(&self) -> (&'static str, &'static str) {
        let mut rng = rand::thread_rng();
        let choice = rng.gen_range(0..self.length);
        self.get_choice_pair(choice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_pair() {
        let choice_name_options = ChoiceNameOptions::new();
        let (choice_a, choice_b) = choice_name_options.get_random_pair();
        assert_ne!(choice_a, choice_b);
    }

    #[test]
    fn test_get_choice_pair() {
        let choice_name_options = ChoiceNameOptions::new();
        let (choice_a, choice_b) = choice_name_options.get_choice_pair(0);
        assert_eq!(choice_a, "cooperate");
        assert_eq!(choice_b, "defect");
    }
}
