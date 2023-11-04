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

use std::fmt::Display;

/// GameOptions is a struct that holds the options for a game.
///
/// # Example
/// ```
/// use dilemma_tactix_lib::GameOptions;
///
/// let game_options = GameOptions::default();
/// assert_eq!(game_options.min_value(), 0);
/// assert_eq!(game_options.max_value(), 10);
/// assert_eq!(game_options.choice_aleph(), "Cooperate");
/// assert_eq!(game_options.choice_bey(), "Defect");
/// ```
#[derive(Debug, Clone)]
pub struct GameOptions {
    /// The minimum value for that can be assigned to a choice.
    min_value: u32,
    /// The maximum value for that can be assigned to a choice.
    max_value: u32,
    /// The label for the first choice that can be made
    choice_aleph: String,
    /// The label for the second choice that can be made
    choice_bey: String,
}

impl GameOptions {
    /// Creates a new GameOptions struct.
    ///
    /// # Arguments
    ///
    /// * `min_value` - The minimum value for that can be assigned to a choice.
    /// * `max_value` - The maximum value for that can be assigned to a choice.
    /// * `choice_aleph` - The label for the first choice that can be made.
    /// * `choice_bey` - The label for the second choice that can be made.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::GameOptions;
    ///
    /// let game_options = GameOptions::new(1, 10, "A".to_string(), "B".to_string());
    /// assert_eq!(game_options.min_value(), 1);
    /// assert_eq!(game_options.max_value(), 10);
    /// assert_eq!(game_options.choice_aleph(), "A");
    /// assert_eq!(game_options.choice_bey(), "B");
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `min_value` is greater than `max_value` or if `choice_aleph` or `choice_bey` are empty.
    ///
    pub fn new(min_value: u32, max_value: u32, choice_aleph: String, choice_bey: String) -> Self {
        if min_value > max_value {
            panic!("min_value must be less than or equal to max_value");
        }

        if choice_aleph.is_empty() {
            panic!("choice_aleph cannot be empty");
        }

        if choice_bey.is_empty() {
            panic!("choice_bey cannot be empty");
        }
        Self {
            min_value,
            max_value,
            choice_aleph,
            choice_bey,
        }
    }

    /// Returns the value of `min_value`.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::GameOptions;
    ///
    /// let game_options = GameOptions::default();
    /// assert_eq!(game_options.min_value(), 0);
    /// ```
    ///
    pub fn min_value(&self) -> u32 {
        self.min_value
    }

    /// Returns the value of `max_value`.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::GameOptions;
    ///
    /// let game_options = GameOptions::default();
    /// assert_eq!(game_options.max_value(), 10);
    /// ```
    pub fn max_value(&self) -> u32 {
        self.max_value
    }

    /// Returns the value of `choice_aleph`.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::GameOptions;
    ///
    /// let game_options = GameOptions::default();
    /// assert_eq!(game_options.choice_aleph(), "Cooperate");
    /// ```
    pub fn choice_aleph(&self) -> &str {
        &self.choice_aleph
    }

    /// Returns the value of `choice_bey`.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::GameOptions;
    ///
    /// let game_options = GameOptions::default();
    /// assert_eq!(game_options.choice_bey(), "Defect");
    /// ```
    pub fn choice_bey(&self) -> &str {
        &self.choice_bey
    }
}

impl Default for GameOptions {
    /// Creates a new GameOptions struct with default values.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::GameOptions;
    ///
    /// let game_options = GameOptions::default();
    /// assert_eq!(game_options.min_value(), 0);
    /// assert_eq!(game_options.max_value(), 10);
    /// assert_eq!(game_options.choice_aleph(), "Cooperate");
    /// assert_eq!(game_options.choice_bey(), "Defect");
    /// ```
    fn default() -> Self {
        Self::new(1, 10, "Cooperate".to_string(), "Defect".to_string())
    }
}

impl Display for GameOptions {
    /// Implements the Display trait for GameOptions.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::GameOptions;
    ///
    /// let game_options = GameOptions::default();
    /// assert_eq!(format!("{}", game_options), "min_value: 0, max_value: 10, choice_aleph: Cooperate, choice_bey: Defect");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "min_value: {}, max_value: {}, choice_aleph: {}, choice_bey: {}",
            self.min_value, self.max_value, self.choice_aleph, self.choice_bey
        )
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_game_options_default() {
        let game_options = GameOptions::default();
        assert_eq!(game_options.min_value(), 1);
        assert_eq!(game_options.max_value(), 10);
        assert_eq!(game_options.choice_aleph(), "Cooperate");
        assert_eq!(game_options.choice_bey(), "Defect");
    }

    #[test]
    fn test_game_options_new() {
        let game_options = GameOptions::new(1, 10, "A".to_string(), "B".to_string());
        assert_eq!(game_options.min_value(), 1);
        assert_eq!(game_options.max_value(), 10);
        assert_eq!(game_options.choice_aleph(), "A");
        assert_eq!(game_options.choice_bey(), "B");
    }

    #[test]
    fn test_game_options_display() {
        let game_options = GameOptions::default();
        assert_eq!(
            format!("{}", game_options),
            "min_value: 1, max_value: 10, choice_aleph: Cooperate, choice_bey: Defect"
        );
    }
}
