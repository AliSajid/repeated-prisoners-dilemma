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
// ** MIT LICENSE
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *
// *
// * Permission is hereby granted, free of charge, to any person obtaining a
//   copy
// * of this software and associated documentation files (the "Software"), to
//   deal
// * in the Software without restriction, including without limitation the
//   rights
// * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// * copies of the Software, and to permit persons to whom the Software is
// * furnished to do so, subject to the following conditions:
// *
// * The above copyright notice and this permission notice shall be included in
//   all
// * copies or substantial portions of the Software.
// *
// * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
//   FROM,
// * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
//   THE
// * SOFTWARE.
// * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

use std::fmt::Display;

use crate::ChoiceNameOptions;

/// This is a struct that holds the options for a game.
///
/// This struct is used to encapsulate the parameters to be used for generating
/// a single grid from the parameters that may be related to a tournament or a
/// series of grids.
///
/// The four parameters are:
///
/// * `min_value` - The minimum value for that can be assigned to a choice.
/// * `max_value` - The maximum value for that can be assigned to a choice.
/// * `choice_atlantis` - The label for the first choice that can be made.
/// * `choice_olympus` - The label for the second choice that can be made.
///
/// # Example
///
/// ## Explicit Options
///
/// ```
/// use dilemma_tactix_lib::GameOptions;
///
/// let game_options =
///     GameOptions::new(1, 10, "cooperate".to_string(), "defect".to_string());
/// assert_eq!(game_options.min_value(), 1);
/// assert_eq!(game_options.max_value(), 10);
/// assert_eq!(game_options.choice_atlantis(), "cooperate");
/// assert_eq!(game_options.choice_olympus(), "defect");
/// ```
///
/// ## Default Options
///
/// ```
/// use dilemma_tactix_lib::GameOptions;
///
/// let game_options = GameOptions::default();
/// assert_eq!(game_options.min_value(), 1);
/// assert_eq!(game_options.max_value(), 10);
/// assert_ne!(
///     game_options.choice_atlantis(),
///     game_options.choice_olympus()
/// );
/// ```
///
/// # Notes
///
/// The `GameOptions` struct implements the `Default` trait, and can be created
/// with the `default()` method.
///
/// # See Also
///
/// * [`GameOptions::new()`](#method.new)
/// * [`GameOptions::default()`](#method.default)
#[derive(Debug, Clone)]
pub struct GameOptions<'a> {
    /// The minimum value for that can be assigned to a choice.
    min_value:       u32,
    /// The maximum value for that can be assigned to a choice.
    max_value:       u32,
    /// The label for the first choice that can be made
    choice_atlantis: &'a str,
    /// The label for the second choice that can be made
    choice_olympus:  &'a str,
}

impl<'a> GameOptions<'a> {
    /// Creates a new `GameOptions` struct.
    ///
    /// # Arguments
    ///
    /// * `min_value` - The minimum value for that can be assigned to a choice.
    /// * `max_value` - The maximum value for that can be assigned to a choice.
    /// * `choice_atlantis` - The label for the first choice that can be made.
    /// * `choice_olympus` - The label for the second choice that can be made.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::GameOptions;
    ///
    /// let game_options = GameOptions::new(1, 10, "A", "B");
    /// assert_eq!(game_options.min_value(), 1);
    /// assert_eq!(game_options.max_value(), 10);
    /// assert_eq!(game_options.choice_atlantis(), "A");
    /// assert_eq!(game_options.choice_olympus(), "B");
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `min_value` is greater than `max_value` or if
    /// `choice_atlantis` or `choice_olympus` are empty.
    ///
    /// # See Also
    ///
    /// * [`GameOptions::default()`](#method.default)
    #[must_use]
    pub fn new(
        min_value: u32,
        max_value: u32,
        choice_atlantis: &'a str,
        choice_olympus: &'a str,
    ) -> Self {
        Self {
            min_value,
            max_value,
            choice_atlantis,
            choice_olympus,
        }
    }

    /// Returns the value of `min_value`.
    ///
    /// This function returns the value of `min_value`.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::GameOptions;
    ///
    /// let game_options = GameOptions::default();
    /// assert_eq!(game_options.min_value(), 1);
    /// ```
    ///
    /// # Returns
    ///
    /// The value of `min_value`.
    ///
    /// # See Also
    ///
    /// * [`GameOptions::max_value()`](#method.max_value)
    /// * [`GameOptions::choice_atlantis()`](#method.choice_atlantis)
    /// * [`GameOptions::choice_olympus()`](#method.choice_olympus)
    #[must_use]
    pub const fn min_value(&self) -> u32 {
        self.min_value
    }

    /// Returns the value of `max_value`.
    ///
    /// This function returns the value of `max_value`.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::GameOptions;
    ///
    /// let game_options = GameOptions::default();
    /// assert_eq!(game_options.max_value(), 10);
    /// ```
    ///
    /// # Returns
    ///
    /// The value of `max_value`.
    ///
    /// # See Also
    ///
    /// * [`GameOptions::min_value()`](#method.min_value)
    /// * [`GameOptions::choice_atlantis()`](#method.choice_atlantis)
    /// * [`GameOptions::choice_olympus()`](#method.choice_olympus)
    #[must_use]
    pub const fn max_value(&self) -> u32 {
        self.max_value
    }

    /// Returns the value of `choice_atlantis`.
    ///
    /// This function returns the value of `choice_atlantis`.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     ChoiceNameOptions,
    ///     GameOptions,
    /// };
    ///
    /// let choice_name_options = ChoiceNameOptions::new();
    /// let game_options = GameOptions::default();
    /// assert!(choice_name_options
    ///     .choice_atlantis_options
    ///     .contains(&game_options.choice_atlantis()))
    /// ```
    ///
    /// # Returns
    ///
    /// The value of `choice_atlantis`.
    ///
    /// # See Also
    ///
    /// * [`GameOptions::min_value()`](#method.min_value)
    /// * [`GameOptions::max_value()`](#method.max_value)
    /// * [`GameOptions::choice_olympus()`](#method.choice_olympus)
    #[must_use]
    pub fn choice_atlantis(&self) -> &str {
        self.choice_atlantis
    }

    /// Returns the value of `choice_olympus`.
    ///
    /// This function returns the value of `choice_olympus`.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     ChoiceNameOptions,
    ///     GameOptions,
    /// };
    ///
    /// let choice_name_options = ChoiceNameOptions::new();
    /// let game_options = GameOptions::default();
    /// assert!(choice_name_options
    ///     .choice_olympus_options
    ///     .contains(&game_options.choice_olympus()))
    /// ```
    ///
    /// # Returns
    ///
    /// The value of `choice_olympus`.
    ///
    /// # See Also
    ///
    /// * [`GameOptions::min_value()`](#method.min_value)
    /// * [`GameOptions::max_value()`](#method.max_value)
    /// * [`GameOptions::choice_atlantis()`](#method.choice_atlantis)
    #[must_use]
    pub fn choice_olympus(&self) -> &str {
        self.choice_olympus
    }
}

impl<'a> Default for GameOptions<'a> {
    /// Creates a new `GameOptions` struct with default values.
    ///
    /// This function creates a new `GameOptions` struct with default values.
    ///
    /// The default values are:
    ///
    /// * `min_value` - 1
    /// * `max_value` - 10
    /// * `choice_atlantis` - "cooperate"
    /// * `choice_olympus` - "defect"
    ///
    /// # Returns
    ///
    /// A new `GameOptions` struct with default values.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     ChoiceNameOptions,
    ///     GameOptions,
    /// };
    ///
    /// let choice_name_options = ChoiceNameOptions::new();
    /// let game_options = GameOptions::default();
    /// assert_eq!(game_options.min_value(), 1);
    /// assert_eq!(game_options.max_value(), 10);
    /// assert_ne!(
    ///     game_options.choice_atlantis(),
    ///     game_options.choice_olympus()
    /// );
    /// assert!(choice_name_options
    ///     .choice_atlantis_options
    ///     .contains(&game_options.choice_atlantis()));
    /// assert!(choice_name_options
    ///     .choice_olympus_options
    ///     .contains(&game_options.choice_olympus()));
    /// ```
    ///
    /// # See Also
    ///
    /// * [`GameOptions::new()`](#method.new)
    fn default() -> Self {
        let choice_name_options = ChoiceNameOptions::new();
        let (choice_atlantis, choice_olympus) = choice_name_options.get_random_pair();
        Self::new(1, 10, choice_atlantis, choice_olympus)
    }
}

impl<'a> Display for GameOptions<'a> {
    /// Implements the Display trait for `GameOptions`.
    ///
    /// This function implements the Display trait for `GameOptions`.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::GameOptions;
    ///
    /// let game_options = GameOptions::new(1, 10, "cooperate", "defect");
    /// assert_eq!(
    ///     format!("{}", game_options),
    ///     "min_value: 1, max_value: 10, choice_atlantis: cooperate, \
    ///      choice_olympus: defect"
    /// );
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "min_value: {}, max_value: {}, choice_atlantis: {}, choice_olympus: {}",
            self.min_value, self.max_value, self.choice_atlantis, self.choice_olympus
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_options_default() {
        let choice_name_options = ChoiceNameOptions::new();
        let game_options = GameOptions::default();
        assert_eq!(game_options.min_value(), 1);
        assert_eq!(game_options.max_value(), 10);
        assert_ne!(
            game_options.choice_atlantis(),
            game_options.choice_olympus()
        );
        assert!(choice_name_options
            .choice_atlantis_options
            .contains(&game_options.choice_atlantis()));
        assert!(choice_name_options
            .choice_olympus_options
            .contains(&game_options.choice_olympus()));
    }

    #[test]
    fn test_game_options_new() {
        let game_options = GameOptions::new(1, 10, "A", "B");
        assert_eq!(game_options.min_value(), 1);
        assert_eq!(game_options.max_value(), 10);
        assert_eq!(game_options.choice_atlantis(), "A");
        assert_eq!(game_options.choice_olympus(), "B");
    }

    #[test]
    fn test_game_options_display() {
        let game_options = GameOptions::new(1, 10, "A", "B");
        assert_eq!(
            format!("{}", game_options),
            "min_value: 1, max_value: 10, choice_atlantis: A, choice_olympus: B"
        );
    }
}
