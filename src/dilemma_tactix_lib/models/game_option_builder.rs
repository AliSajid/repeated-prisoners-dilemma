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

use rand::{
    Rng,
    SeedableRng,
};

use crate::{
    GameOptions,
    NumberPair,
};

/// A builder struct to create a [`GameOptions`](crate::GameOptions).
///
/// This struct is used to create the `GameOptions` struct by success
///
/// # Example
///
/// ```
/// use dilemma_tactix_lib::{
///     GameOptionsBuilder,
///     NumberPair,
/// };
///
/// let game_options = GameOptionsBuilder::new()
///     .max_value(10)
///     .min_value(1)
///     .choice_atlantis("A".to_string())
///     .choice_olympus("B".to_string())
///     .atlantis_atlantis(NumberPair::new(1, 1))
///     .atlantis_olympus(NumberPair::new(1, 1))
///     .olympus_atlantis(NumberPair::new(1, 1))
///     .olympus_olympus(NumberPair::new(1, 1))
///     .build();
/// assert_eq!(game_options.min_value(), 1);
/// assert_eq!(game_options.max_value(), 10);
/// assert_eq!(game_options.choice_atlantis(), "A");
/// assert_eq!(game_options.choice_olympus(), "B");
/// assert_eq!(game_options.atlantis_atlantis(), NumberPair::new(1, 1));
/// assert_eq!(game_options.atlantis_olympus(), NumberPair::new(1, 1));
/// assert_eq!(game_options.olympus_atlantis(), NumberPair::new(1, 1));
/// assert_eq!(game_options.olympus_olympus(), NumberPair::new(1, 1));
/// ```
///
/// # Notes
///
/// I chose to use the `Builder` pattern here because it allows for easier
/// manipulation of the `GameOptions` struct.
///
/// # Panics
///
/// This struct will panic if any of the required fields are not set.
///
/// # See Also
///
/// * [`GameOptions`](crate::GameOptions)
/// * [`GameOptions::new()`](crate::GameOptions::new())
#[derive(Debug, Default)]
pub struct GameOptionsBuilder {
    pub min_value:         Option<u32>,
    pub max_value:         Option<u32>,
    pub choice_atlantis:   Option<&'static str>,
    pub choice_olympus:    Option<&'static str>,
    pub atlantis_atlantis: Option<NumberPair>,
    pub atlantis_olympus:  Option<NumberPair>,
    pub olympus_atlantis:  Option<NumberPair>,
    pub olympus_olympus:   Option<NumberPair>,
}

impl GameOptionsBuilder {
    /// Creates a new `GameOptionsBuilder` struct.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::GameOptionsBuilder;
    ///
    /// let game_options_builder = GameOptionsBuilder::new();
    /// assert_eq!(game_options_builder.max_value, None);
    /// assert_eq!(game_options_builder.min_value, None);
    /// assert_eq!(game_options_builder.choice_atlantis, None);
    /// assert_eq!(game_options_builder.choice_olympus, None);
    /// assert_eq!(game_options_builder.atlantis_atlantis, None);
    /// assert_eq!(game_options_builder.atlantis_olympus, None);
    /// assert_eq!(game_options_builder.olympus_atlantis, None);
    /// assert_eq!(game_options_builder.olympus_olympus, None);
    /// ```
    ///
    /// # Returns
    ///
    /// A new `GameOptionsBuilder` struct.
    ///
    /// # See Also
    ///
    /// * [`GameOptionsBuilder::build()`](GameOptionsBuilder::build())
    /// * [`GameOptionsBuilder::min_value()`](GameOptionsBuilder::min_value())
    /// * [`GameOptionsBuilder::max_value()`](GameOptionsBuilder::max_value())
    /// * [`GameOptionsBuilder::choice_atlantis()`](GameOptionsBuilder::choice_atlantis())
    /// * [`GameOptionsBuilder::choice_olympus()`](GameOptionsBuilder::choice_olympus())
    /// * [`GameOptionsBuilder::atlantis_atlantis()`](GameOptionsBuilder::atlantis_atlantis())
    /// * [`GameOptionsBuilder::atlantis_olympus()`](GameOptionsBuilder::atlantis_olympus())
    /// * [`GameOptionsBuilder::olympus_atlantis()`](GameOptionsBuilder::olympus_atlantis())
    /// * [`GameOptionsBuilder::olympus_olympus()`](GameOptionsBuilder::olympus_olympus())
    #[must_use]
    pub const fn new() -> Self {
        Self {
            max_value:         None,
            min_value:         None,
            choice_atlantis:   None,
            choice_olympus:    None,
            atlantis_atlantis: None,
            atlantis_olympus:  None,
            olympus_atlantis:  None,
            olympus_olympus:   None,
        }
    }

    /// Sets the minimum value for the `GameOptions`.
    ///
    /// # Arguments
    ///
    /// * `min_value` - The minimum value for the `GameOptions`.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::GameOptionsBuilder;
    ///
    /// let game_options_builder = GameOptionsBuilder::new().min_value(1);
    /// assert_eq!(game_options_builder.min_value, Some(1));
    /// ```
    ///
    /// # Returns
    ///
    /// The `GameOptionsBuilder` struct with the `min_value` field set.
    ///
    /// # See Also
    ///
    /// * [`GameOptionsBuilder::new()`](GameOptionsBuilder::new())
    /// * [`GameOptionsBuilder::build()`](GameOptionsBuilder::build())
    /// * [`GameOptionsBuilder::max_value()`](GameOptionsBuilder::max_value())
    /// * [`GameOptionsBuilder::choice_atlantis()`](GameOptionsBuilder::choice_atlantis())
    /// * [`GameOptionsBuilder::choice_olympus()`](GameOptionsBuilder::choice_olympus())
    /// * [`GameOptionsBuilder::atlantis_atlantis()`](GameOptionsBuilder::atlantis_atlantis())
    /// * [`GameOptionsBuilder::atlantis_olympus()`](GameOptionsBuilder::atlantis_olympus())
    /// * [`GameOptionsBuilder::olympus_atlantis()`](GameOptionsBuilder::olympus_atlantis())
    /// * [`GameOptionsBuilder::olympus_olympus()`](GameOptionsBuilder::olympus_olympus())
    #[must_use]
    pub const fn min_value(mut self, min_value: u32) -> Self {
        self.min_value = Some(min_value);
        self
    }

    /// Sets the maximum value for the `GameOptions`.
    ///
    /// # Arguments
    ///
    /// * `max_value` - The maximum value for the `GameOptions`.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::GameOptionsBuilder;
    ///
    /// let game_options_builder = GameOptionsBuilder::new().max_value(10);
    /// assert_eq!(game_options_builder.max_value, Some(10));
    /// ```
    ///
    /// # Returns
    ///
    /// The `GameOptionsBuilder` struct with the `max_value` field set.
    ///
    /// # See Also
    ///
    /// * [`GameOptionsBuilder::new()`](GameOptionsBuilder::new())
    /// * [`GameOptionsBuilder::build()`](GameOptionsBuilder::build())
    /// * [`GameOptionsBuilder::min_value()`](GameOptionsBuilder::min_value())
    /// * [`GameOptionsBuilder::choice_atlantis()`](GameOptionsBuilder::choice_atlantis())
    /// * [`GameOptionsBuilder::choice_olympus()`](GameOptionsBuilder::choice_olympus())
    /// * [`GameOptionsBuilder::atlantis_atlantis()`](GameOptionsBuilder::atlantis_atlantis())
    /// * [`GameOptionsBuilder::atlantis_olympus()`](GameOptionsBuilder::atlantis_olympus())
    /// * [`GameOptionsBuilder::olympus_atlantis()`](GameOptionsBuilder::olympus_atlantis())
    /// * [`GameOptionsBuilder::olympus_olympus()`](GameOptionsBuilder::olympus_olympus())
    #[must_use]
    pub const fn max_value(mut self, max_value: u32) -> Self {
        self.max_value = Some(max_value);
        self
    }

    /// Sets the first choice available to players in `GameOptions`.
    ///
    /// # Arguments
    ///
    /// * `choice_atlantis` - The first choice available to players in
    ///   `GameOptions`.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::GameOptionsBuilder;
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new().choice_atlantis("cooperate");
    /// assert_eq!(game_options_builder.choice_atlantis, Some("cooperate"));
    /// ```
    ///
    /// # Returns
    ///
    /// The `GameOptionsBuilder` struct with the `choice_atlantis` field set.
    ///
    /// # See Also
    ///
    /// * [`GameOptionsBuilder::new()`](GameOptionsBuilder::new())
    /// * [`GameOptionsBuilder::build()`](GameOptionsBuilder::build())
    /// * [`GameOptionsBuilder::max_value()`](GameOptionsBuilder::max_value())
    /// * [`GameOptionsBuilder::min_value()`](GameOptionsBuilder::min_value())
    /// * [`GameOptionsBuilder::choice_olympus()`](GameOptionsBuilder::choice_olympus())
    /// * [`GameOptionsBuilder::atlantis_atlantis()`](GameOptionsBuilder::atlantis_atlantis())
    /// * [`GameOptionsBuilder::atlantis_olympus()`](GameOptionsBuilder::atlantis_olympus())
    /// * [`GameOptionsBuilder::olympus_atlantis()`](GameOptionsBuilder::olympus_atlantis())
    /// * [`GameOptionsBuilder::olympus_olympus()`](GameOptionsBuilder::olympus_olympus())
    #[must_use]
    pub const fn choice_atlantis(mut self, choice_atlantis: &'static str) -> Self {
        self.choice_atlantis = Some(choice_atlantis);
        self
    }

    /// Sets the second choice available to players in `GameOptions`.
    ///
    /// # Arguments
    ///
    /// * `choice_olympus` - The second choice available to players in
    ///   `GameOptions`.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::GameOptionsBuilder;
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new().choice_olympus("defect");
    /// assert_eq!(game_options_builder.choice_olympus, Some("defect"));
    /// ```
    ///
    /// # Returns
    ///
    /// The `GameOptionsBuilder` struct with the `choice_olympus` field set.
    ///
    /// # See Also
    ///
    /// * [`GameOptionsBuilder::new()`](GameOptionsBuilder::new())
    /// * [`GameOptionsBuilder::build()`](GameOptionsBuilder::build())
    /// * [`GameOptionsBuilder::max_value()`](GameOptionsBuilder::max_value())
    /// * [`GameOptionsBuilder::min_value()`](GameOptionsBuilder::min_value())
    /// * [`GameOptionsBuilder::choice_atlantis()`](GameOptionsBuilder::choice_atlantis())
    /// * [`GameOptionsBuilder::atlantis_atlantis()`](GameOptionsBuilder::atlantis_atlantis())
    /// * [`GameOptionsBuilder::atlantis_olympus()`](GameOptionsBuilder::atlantis_olympus())
    /// * [`GameOptionsBuilder::olympus_atlantis()`](GameOptionsBuilder::olympus_atlantis())
    /// * [`GameOptionsBuilder::olympus_olympus()`](GameOptionsBuilder::olympus_olympus())
    #[must_use]
    pub const fn choice_olympus(mut self, choice_olympus: &'static str) -> Self {
        self.choice_olympus = Some(choice_olympus);
        self
    }

    /// Sets the score for the case when both players choose the first choice.
    ///
    /// # Arguments
    ///
    /// * `atlantis_atlantis` - The score to set.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     NumberPair,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new().atlantis_atlantis(NumberPair::new(1, 1));
    /// assert_eq!(
    ///     game_options_builder.atlantis_atlantis,
    ///     Some(NumberPair::new(1, 1))
    /// );
    /// ```
    ///
    /// # Returns
    ///
    /// The `GameOptionsBuilder` struct with the `atlantis_atlantis` field set.
    ///
    /// # See Also
    ///
    /// * [`GameOptionsBuilder::new()`](GameOptionsBuilder::new())
    /// * [`GameOptionsBuilder::build()`](GameOptionsBuilder::build())
    /// * [`GameOptionsBuilder::max_value()`](GameOptionsBuilder::max_value())
    /// * [`GameOptionsBuilder::min_value()`](GameOptionsBuilder::min_value())
    /// * [`GameOptionsBuilder::choice_atlantis()`](GameOptionsBuilder::choice_atlantis())
    /// * [`GameOptionsBuilder::choice_olympus()`](GameOptionsBuilder::choice_olympus())
    /// * [`GameOptionsBuilder::atlantis_olympus()`](GameOptionsBuilder::atlantis_olympus())
    /// * [`GameOptionsBuilder::olympus_atlantis()`](GameOptionsBuilder::olympus_atlantis())
    /// * [`GameOptionsBuilder::olympus_olympus()`](GameOptionsBuilder::olympus_olympus())
    #[must_use]
    pub const fn atlantis_atlantis(mut self, atlantis_atlantis: NumberPair) -> Self {
        self.atlantis_atlantis = Some(atlantis_atlantis);
        self
    }

    /// Sets the score for the case when the first player chooses the first
    /// choice and the second player chooses the second choice.
    ///
    /// # Arguments
    ///
    /// * `atlantis_olympus` - The score to set.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     NumberPair,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new().atlantis_olympus(NumberPair::new(1, 1));
    /// assert_eq!(
    ///     game_options_builder.atlantis_olympus,
    ///     Some(NumberPair::new(1, 1))
    /// );
    /// ```
    ///
    /// # Returns
    ///
    /// The `GameOptionsBuilder` struct with the `atlantis_olympus` field set.
    ///
    ///
    /// # See Also
    ///
    /// * [`GameOptionsBuilder::new()`](GameOptionsBuilder::new())
    /// * [`GameOptionsBuilder::build()`](GameOptionsBuilder::build())
    /// * [`GameOptionsBuilder::max_value()`](GameOptionsBuilder::max_value())
    /// * [`GameOptionsBuilder::min_value()`](GameOptionsBuilder::min_value())
    /// * [`GameOptionsBuilder::choice_atlantis()`](GameOptionsBuilder::choice_atlantis())
    /// * [`GameOptionsBuilder::choice_olympus()`](GameOptionsBuilder::choice_olympus())
    /// * [`GameOptionsBuilder::atlantis_atlantis()`](GameOptionsBuilder::atlantis_atlantis())
    /// * [`GameOptionsBuilder::olympus_atlantis()`](GameOptionsBuilder::olympus_atlantis())
    /// * [`GameOptionsBuilder::olympus_olympus()`](GameOptionsBuilder::olympus_olympus())
    #[must_use]
    pub const fn atlantis_olympus(mut self, atlantis_olympus: NumberPair) -> Self {
        self.atlantis_olympus = Some(atlantis_olympus);
        self
    }

    /// Sets the score for the case when the first player chooses the second
    /// choice and the second player chooses the first choice.
    ///
    /// # Arguments
    ///
    /// * `olympus_atlantis` - The score to set.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     NumberPair,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new().olympus_atlantis(NumberPair::new(1, 1));
    /// assert_eq!(
    ///     game_options_builder.olympus_atlantis,
    ///     Some(NumberPair::new(1, 1))
    /// );
    /// ```
    ///
    /// # Returns
    ///
    /// The `GameOptionsBuilder` struct with the `olympus_atlantis` field set.
    ///
    ///
    /// # See Also
    ///
    /// * [`GameOptionsBuilder::new()`](GameOptionsBuilder::new())
    /// * [`GameOptionsBuilder::build()`](GameOptionsBuilder::build())
    /// * [`GameOptionsBuilder::max_value()`](GameOptionsBuilder::max_value())
    /// * [`GameOptionsBuilder::min_value()`](GameOptionsBuilder::min_value())
    /// * [`GameOptionsBuilder::choice_atlantis()`](GameOptionsBuilder::choice_atlantis())
    /// * [`GameOptionsBuilder::choice_olympus()`](GameOptionsBuilder::choice_olympus())
    /// * [`GameOptionsBuilder::atlantis_atlantis()`](GameOptionsBuilder::atlantis_atlantis())
    /// * [`GameOptionsBuilder::atlantis_olympus()`](GameOptionsBuilder::atlantis_olympus())
    /// * [`GameOptionsBuilder::olympus_olympus()`](GameOptionsBuilder::olympus_olympus())
    #[must_use]
    pub const fn olympus_atlantis(mut self, olympus_atlantis: NumberPair) -> Self {
        self.olympus_atlantis = Some(olympus_atlantis);
        self
    }

    /// Sets the score for the case when both players choose the second choice.
    ///
    /// # Arguments
    ///
    /// * `olympus_olympus` - The score to set.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameOptionsBuilder,
    ///     NumberPair,
    /// };
    ///
    /// let game_options_builder =
    ///     GameOptionsBuilder::new().olympus_olympus(NumberPair::new(1, 1));
    /// assert_eq!(
    ///     game_options_builder.olympus_olympus,
    ///     Some(NumberPair::new(1, 1))
    /// );
    /// ```
    ///
    /// # Returns
    ///
    /// The `GameOptionsBuilder` struct with the `olympus_olympus` field set.
    ///
    ///
    /// # See Also
    ///
    /// * [`GameOptionsBuilder::new()`](GameOptionsBuilder::new())
    /// * [`GameOptionsBuilder::build()`](GameOptionsBuilder::build())
    /// * [`GameOptionsBuilder::max_value()`](GameOptionsBuilder::max_value())
    /// * [`GameOptionsBuilder::min_value()`](GameOptionsBuilder::min_value())
    /// * [`GameOptionsBuilder::choice_atlantis()`](GameOptionsBuilder::choice_atlantis())
    /// * [`GameOptionsBuilder::choice_olympus()`](GameOptionsBuilder::choice_olympus())
    /// * [`GameOptionsBuilder::atlantis_atlantis()`](GameOptionsBuilder::atlantis_atlantis())
    /// * [`GameOptionsBuilder::atlantis_olympus()`](GameOptionsBuilder::atlantis_olympus())
    /// * [`GameOptionsBuilder::olympus_atlantis()`](GameOptionsBuilder::olympus_atlantis())
    #[must_use]
    pub const fn olympus_olympus(mut self, olympus_olympus: NumberPair) -> Self {
        self.olympus_olympus = Some(olympus_olympus);
        self
    }

    /// Builds the `GameOptions` struct.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameOptions,
    ///     GameOptionsBuilder,
    ///     NumberPair,
    /// };
    ///
    /// let game_options = GameOptionsBuilder::new()
    ///     .min_value(1)
    ///     .max_value(10)
    ///     .choice_atlantis("cooperate")
    ///     .choice_olympus("defect")
    ///     .atlantis_atlantis(NumberPair::new(1, 1))
    ///     .atlantis_olympus(NumberPair::new(1, 1))
    ///     .olympus_atlantis(NumberPair::new(1, 1))
    ///     .olympus_olympus(NumberPair::new(1, 1))
    ///     .build();
    /// assert_eq!(game_options.max_value(), 10);
    /// assert_eq!(game_options.min_value(), 1);
    /// assert_eq!(game_options.choice_atlantis(), "cooperate");
    /// assert_eq!(game_options.choice_olympus(), "defect");
    /// assert_eq!(game_options.atlantis_atlantis(), NumberPair::new(4, 4));
    /// assert_eq!(game_options.atlantis_olympus(), NumberPair::new(5, 0));
    /// assert_eq!(game_options.olympus_atlantis(), NumberPair::new(0, 5));
    /// assert_eq!(game_options.olympus_olympus(), NumberPair::new(3, 3));
    /// ```
    ///
    /// # Returns
    ///
    /// A new `GameOptions` struct.
    ///
    /// # Panics
    ///
    /// This function will panic if any of the required fields are not set.
    ///
    /// # See Also
    ///
    /// * [`GameOptionsBuilder::new()`](GameOptionsBuilder::new())
    /// * [`GameOptionsBuilder::max_value()`](GameOptionsBuilder::max_value())
    /// * [`GameOptionsBuilder::min_value()`](GameOptionsBuilder::min_value())
    /// * [`GameOptionsBuilder::choice_atlantis()`](GameOptionsBuilder::choice_atlantis())
    /// * [`GameOptionsBuilder::choice_olympus()`](GameOptionsBuilder::choice_olympus())
    /// * [`GameOptionsBuilder::atlantis_atlantis()`](GameOptionsBuilder::atlantis_atlantis())
    /// * [`GameOptionsBuilder::atlantis_olympus()`](GameOptionsBuilder::atlantis_olympus())
    /// * [`GameOptionsBuilder::olympus_atlantis()`](GameOptionsBuilder::olympus_atlantis())
    /// * [`GameOptionsBuilder::olympus_olympus()`](GameOptionsBuilder::olympus_olympus())
    #[must_use]
    pub fn build(self) -> GameOptions {
        // Unwrap the min_value and max_value fields of the builder, providing default
        // values if they are None
        let min_value = self.min_value.unwrap_or(1);
        let max_value = self.max_value.unwrap_or(10);

        // Unwrap the choice_atlantis and choice_olympus fields of the builder,
        // providing default values if they are None
        let choice_atlantis = self.choice_atlantis.unwrap_or("cooperate");
        let choice_olympus = self.choice_olympus.unwrap_or("defect");

        // Define a closure that generates a NumberPair
        // If min_value and max_value are None, it returns the default pair
        // Otherwise, it generates a new NumberPair with two random values between
        // min_value and max_value
        let generate_number_pair = |default_pair: NumberPair| -> NumberPair {
            // Create a new random number generator
            let mut rng = rand_chacha::ChaCha12Rng::from_entropy();

            if (self.min_value, self.max_value) == (None, None) {
                default_pair
            } else {
                let random_value1 = rng.gen_range(min_value..=max_value);
                let random_value2 = rng.gen_range(min_value..=max_value);
                NumberPair::new(random_value1, random_value2)
            }
        };

        // Generate the atlantis_atlantis, atlantis_olympus, olympus_atlantis, and
        // olympus_olympus fields using the closure
        let atlantis_atlantis = generate_number_pair(NumberPair::new(4, 4));
        let atlantis_olympus = generate_number_pair(NumberPair::new(5, 0));
        let olympus_atlantis = generate_number_pair(NumberPair::new(0, 5));
        let olympus_olympus = generate_number_pair(NumberPair::new(3, 3));

        // Return a new GameOptions instance with the generated and unwrapped fields
        GameOptions {
            min_value,
            max_value,
            choice_atlantis,
            choice_olympus,
            atlantis_atlantis,
            atlantis_olympus,
            olympus_atlantis,
            olympus_olympus,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_options_builder() {
        let game_options = GameOptionsBuilder::new()
            .max_value(10)
            .min_value(1)
            .choice_atlantis("A")
            .choice_olympus("O")
            .atlantis_atlantis(NumberPair::new(1, 1))
            .atlantis_olympus(NumberPair::new(1, 1))
            .olympus_atlantis(NumberPair::new(1, 1))
            .olympus_olympus(NumberPair::new(1, 1))
            .build();

        assert_eq!(game_options.max_value(), 10);
        assert_eq!(game_options.min_value(), 1);
        assert_eq!(game_options.choice_atlantis(), "A");
        assert_eq!(game_options.choice_olympus(), "O");
        assert_eq!(game_options.atlantis_atlantis(), NumberPair::new(1, 1));
        assert_eq!(game_options.atlantis_olympus(), NumberPair::new(1, 1));
        assert_eq!(game_options.olympus_atlantis(), NumberPair::new(1, 1));
        assert_eq!(game_options.olympus_olympus(), NumberPair::new(1, 1));
    }

    #[test]
    fn test_default_builder() {
        let game_options = GameOptionsBuilder::new().build();

        assert_eq!(game_options.max_value(), 10);
        assert_eq!(game_options.min_value(), 1);
        assert_eq!(game_options.choice_atlantis(), "cooperate");
        assert_eq!(game_options.choice_olympus(), "defect");
        assert_eq!(game_options.atlantis_atlantis(), NumberPair::new(4, 4));
        assert_eq!(game_options.atlantis_olympus(), NumberPair::new(5, 0));
        assert_eq!(game_options.olympus_atlantis(), NumberPair::new(0, 5));
        assert_eq!(game_options.olympus_olympus(), NumberPair::new(3, 3));
    }
}
