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

use crate::{
    GameGrid,
    GameOptions,
    NumberPair,
};

/// A builder struct to create a [`GameGrid`](crate::GameGrid).
///
/// This struct is used to create the `GameGrid` struct by success
///
/// # Example
///
/// ```
/// use dilemma_tactix_lib::{
///     GameGrid,
///     GameGridBuilder,
///     NumberPair,
/// };
///
/// let game_grid = GameGridBuilder::new()
///     .max_value(10)
///     .min_value(1)
///     .choice_atlantis("A".to_string())
///     .choice_olympus("B".to_string())
///     .score_aa(NumberPair::new(1, 1))
///     .score_ab(NumberPair::new(1, 1))
///     .score_ba(NumberPair::new(1, 1))
///     .score_bb(NumberPair::new(1, 1))
///     .build();
///
/// assert_eq!(game_grid.max_value(), 10);
/// assert_eq!(game_grid.min_value(), 1);
/// assert_eq!(game_grid.choice_atlantis(), "A");
/// assert_eq!(game_grid.choice_olympus(), "B");
/// assert_eq!(game_grid.score_aa(), NumberPair::new(1, 1));
/// assert_eq!(game_grid.score_ab(), NumberPair::new(1, 1));
/// assert_eq!(game_grid.score_ba(), NumberPair::new(1, 1));
/// assert_eq!(game_grid.score_bb(), NumberPair::new(1, 1));
/// ```
///
/// # Notes
///
/// I chose to use the `Builder` pattern here because it allows for easier
/// manipulation of the `GameGrid` struct.
///
/// # Panics
///
/// This struct will panic if any of the required fields are not set.
///
/// # See Also
///
/// * [`GameGrid`](crate::GameGrid)
/// * [`GameGrid::new()`](crate::GameGrid::new())
#[derive(Debug, Default)]
pub struct GameGridBuilder<'a> {
    pub max_value:       Option<u32>,
    pub min_value:       Option<u32>,
    pub choice_atlantis: Option<&'a str>,
    pub choice_olympus:  Option<&'a str>,
    pub score_aa:        Option<NumberPair>,
    pub score_ab:        Option<NumberPair>,
    pub score_ba:        Option<NumberPair>,
    pub score_bb:        Option<NumberPair>,
}

impl<'a> GameGridBuilder<'a> {
    /// Creates a new `GameGridBuilder` struct.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::GameGridBuilder;
    ///
    /// let game_grid_builder = GameGridBuilder::new();
    ///
    /// assert_eq!(game_grid_builder.max_value, None);
    /// assert_eq!(game_grid_builder.min_value, None);
    /// assert_eq!(game_grid_builder.choice_atlantis, None);
    /// assert_eq!(game_grid_builder.choice_olympus, None);
    /// assert_eq!(game_grid_builder.score_aa, None);
    /// assert_eq!(game_grid_builder.score_ab, None);
    /// assert_eq!(game_grid_builder.score_ba, None);
    /// assert_eq!(game_grid_builder.score_bb, None);
    /// ```
    ///
    /// # Returns
    ///
    /// A new `GameGridBuilder` struct.
    ///
    /// # See Also
    ///
    /// * [`GameGridBuilder::build()`](GameGridBuilder::build())
    /// * [`GameGridBuilder::max_value()`](GameGridBuilder::max_value())
    /// * [`GameGridBuilder::min_value()`](GameGridBuilder::min_value())
    /// * [`GameGridBuilder::choice_atlantis()`](GameGridBuilder::choice_atlantis())
    /// * [`GameGridBuilder::choice_olympus()`](GameGridBuilder::choice_olympus())
    /// * [`GameGridBuilder::score_aa()`](GameGridBuilder::score_aa())
    /// * [`GameGridBuilder::score_ab()`](GameGridBuilder::score_ab())
    /// * [`GameGridBuilder::score_ba()`](GameGridBuilder::score_ba())
    /// * [`GameGridBuilder::score_bb()`](GameGridBuilder::score_bb())
    #[must_use]
    pub const fn new() -> Self {
        Self {
            max_value:       None,
            min_value:       None,
            choice_atlantis: None,
            choice_olympus:  None,
            score_aa:        None,
            score_ab:        None,
            score_ba:        None,
            score_bb:        None,
        }
    }

    /// Sets the maximum value for the `GameGrid`.
    ///
    /// # Arguments
    ///
    /// * `max_value` - The maximum value for the `GameGrid`.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::GameGridBuilder;
    ///
    /// let game_grid_builder = GameGridBuilder::new().max_value(10);
    ///
    /// assert_eq!(game_grid_builder.max_value, Some(10));
    /// ```
    ///
    /// # Returns
    ///
    /// The `GameGridBuilder` struct with the `max_value` field set.
    ///
    /// # See Also
    ///
    /// * [`GameGridBuilder::new()`](GameGridBuilder::new())
    /// * [`GameGridBuilder::build()`](GameGridBuilder::build())
    /// * [`GameGridBuilder::min_value()`](GameGridBuilder::min_value())
    /// * [`GameGridBuilder::choice_atlantis()`](GameGridBuilder::choice_atlantis())
    /// * [`GameGridBuilder::choice_olympus()`](GameGridBuilder::choice_olympus())
    /// * [`GameGridBuilder::score_aa()`](GameGridBuilder::score_aa())
    /// * [`GameGridBuilder::score_ab()`](GameGridBuilder::score_ab())
    /// * [`GameGridBuilder::score_ba()`](GameGridBuilder::score_ba())
    /// * [`GameGridBuilder::score_bb()`](GameGridBuilder::score_bb())
    #[must_use]
    pub const fn max_value(mut self, max_value: u32) -> Self {
        self.max_value = Some(max_value);
        self
    }

    /// Sets the minimum value for the `GameGrid`.
    ///
    /// # Arguments
    ///
    /// * `min_value` - The minimum value for the `GameGrid`.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::GameGridBuilder;
    ///
    /// let game_grid_builder = GameGridBuilder::new().min_value(1);
    ///
    /// assert_eq!(game_grid_builder.min_value, Some(1));
    /// ```
    ///
    /// # Returns
    ///
    /// The `GameGridBuilder` struct with the `min_value` field set.
    ///
    /// # See Also
    ///
    /// * [`GameGridBuilder::new()`](GameGridBuilder::new())
    /// * [`GameGridBuilder::build()`](GameGridBuilder::build())
    /// * [`GameGridBuilder::max_value()`](GameGridBuilder::max_value())
    /// * [`GameGridBuilder::choice_atlantis()`](GameGridBuilder::choice_atlantis())
    /// * [`GameGridBuilder::choice_olympus()`](GameGridBuilder::choice_olympus())
    /// * [`GameGridBuilder::score_aa()`](GameGridBuilder::score_aa())
    /// * [`GameGridBuilder::score_ab()`](GameGridBuilder::score_ab())
    /// * [`GameGridBuilder::score_ba()`](GameGridBuilder::score_ba())
    /// * [`GameGridBuilder::score_bb()`](GameGridBuilder::score_bb())
    #[must_use]
    pub const fn min_value(mut self, min_value: u32) -> Self {
        self.min_value = Some(min_value);
        self
    }

    /// Sets the first choice available to players in `GameGrid`.
    ///
    /// # Arguments
    ///
    /// * `choice_atlantis` - The first choice available to players in
    ///   `GameGrid`.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::GameGridBuilder;
    ///
    /// let game_grid_builder =
    ///     GameGridBuilder::new().choice_atlantis("cooperate".to_string());
    ///
    /// assert_eq!(
    ///     game_grid_builder.choice_atlantis,
    ///     Some("cooperate".to_string())
    /// );
    /// ```
    ///
    /// # Returns
    ///
    /// The `GameGridBuilder` struct with the `choice_atlantis` field set.
    ///
    /// # See Also
    ///
    /// * [`GameGridBuilder::new()`](GameGridBuilder::new())
    /// * [`GameGridBuilder::build()`](GameGridBuilder::build())
    /// * [`GameGridBuilder::max_value()`](GameGridBuilder::max_value())
    /// * [`GameGridBuilder::min_value()`](GameGridBuilder::min_value())
    /// * [`GameGridBuilder::choice_olympus()`](GameGridBuilder::choice_olympus())
    /// * [`GameGridBuilder::score_aa()`](GameGridBuilder::score_aa())
    /// * [`GameGridBuilder::score_ab()`](GameGridBuilder::score_ab())
    /// * [`GameGridBuilder::score_ba()`](GameGridBuilder::score_ba())
    /// * [`GameGridBuilder::score_bb()`](GameGridBuilder::score_bb())
    #[must_use]
    pub fn choice_atlantis(mut self, choice_atlantis: &'a str) -> Self {
        self.choice_atlantis = Some(choice_atlantis);
        self
    }

    /// Sets the second choice available to players in `GameGrid`.
    ///
    /// # Arguments
    ///
    /// * `choice_olympus` - The second choice available to players in
    ///   `GameGrid`.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::GameGridBuilder;
    ///
    /// let game_grid_builder =
    ///     GameGridBuilder::new().choice_olympus("defect".to_string());
    ///
    /// assert_eq!(game_grid_builder.choice_olympus, Some("defect".to_string()));
    /// ```
    ///
    /// # Returns
    ///
    /// The `GameGridBuilder` struct with the `choice_olympus` field set.
    ///
    /// # See Also
    ///
    /// * [`GameGridBuilder::new()`](GameGridBuilder::new())
    /// * [`GameGridBuilder::build()`](GameGridBuilder::build())
    /// * [`GameGridBuilder::max_value()`](GameGridBuilder::max_value())
    /// * [`GameGridBuilder::min_value()`](GameGridBuilder::min_value())
    /// * [`GameGridBuilder::choice_atlantis()`](GameGridBuilder::choice_atlantis())
    /// * [`GameGridBuilder::score_aa()`](GameGridBuilder::score_aa())
    /// * [`GameGridBuilder::score_ab()`](GameGridBuilder::score_ab())
    /// * [`GameGridBuilder::score_ba()`](GameGridBuilder::score_ba())
    /// * [`GameGridBuilder::score_bb()`](GameGridBuilder::score_bb())
    #[must_use]
    pub fn choice_olympus(mut self, choice_olympus: &'a str) -> Self {
        self.choice_olympus = Some(choice_olympus);
        self
    }

    /// Sets the score for the case when both players choose the first choice.
    ///
    /// # Arguments
    ///
    /// * `score_aa` - The score to set.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameGridBuilder,
    ///     NumberPair,
    /// };
    ///
    /// let game_grid_builder =
    ///     GameGridBuilder::new().score_aa(NumberPair::new(1, 1));
    ///
    /// assert_eq!(game_grid_builder.score_aa, Some(NumberPair::new(1, 1)));
    /// ```
    ///
    /// # Returns
    ///
    /// The `GameGridBuilder` struct with the `score_aa` field set.
    ///
    /// # See Also
    ///
    /// * [`GameGridBuilder::new()`](GameGridBuilder::new())
    /// * [`GameGridBuilder::build()`](GameGridBuilder::build())
    /// * [`GameGridBuilder::max_value()`](GameGridBuilder::max_value())
    /// * [`GameGridBuilder::min_value()`](GameGridBuilder::min_value())
    /// * [`GameGridBuilder::choice_atlantis()`](GameGridBuilder::choice_atlantis())
    /// * [`GameGridBuilder::choice_olympus()`](GameGridBuilder::choice_olympus())
    /// * [`GameGridBuilder::score_ab()`](GameGridBuilder::score_ab())
    /// * [`GameGridBuilder::score_ba()`](GameGridBuilder::score_ba())
    /// * [`GameGridBuilder::score_bb()`](GameGridBuilder::score_bb())
    #[must_use]
    pub const fn score_aa(mut self, score_aa: NumberPair) -> Self {
        self.score_aa = Some(score_aa);
        self
    }

    /// Sets the score for the case when the first player chooses the first
    /// choice and the second player chooses the second choice.
    ///
    /// # Arguments
    ///
    /// * `score_ab` - The score to set.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameGridBuilder,
    ///     NumberPair,
    /// };
    ///
    /// let game_grid_builder =
    ///     GameGridBuilder::new().score_ab(NumberPair::new(1, 1));
    ///
    /// assert_eq!(game_grid_builder.score_ab, Some(NumberPair::new(1, 1)));
    /// ```
    ///
    /// # Returns
    ///
    /// The `GameGridBuilder` struct with the `score_ab` field set.
    ///
    ///
    /// # See Also
    ///
    /// * [`GameGridBuilder::new()`](GameGridBuilder::new())
    /// * [`GameGridBuilder::build()`](GameGridBuilder::build())
    /// * [`GameGridBuilder::max_value()`](GameGridBuilder::max_value())
    /// * [`GameGridBuilder::min_value()`](GameGridBuilder::min_value())
    /// * [`GameGridBuilder::choice_atlantis()`](GameGridBuilder::choice_atlantis())
    /// * [`GameGridBuilder::choice_olympus()`](GameGridBuilder::choice_olympus())
    /// * [`GameGridBuilder::score_aa()`](GameGridBuilder::score_aa())
    /// * [`GameGridBuilder::score_ba()`](GameGridBuilder::score_ba())
    /// * [`GameGridBuilder::score_bb()`](GameGridBuilder::score_bb())
    #[must_use]
    pub const fn score_ab(mut self, score_ab: NumberPair) -> Self {
        self.score_ab = Some(score_ab);
        self
    }

    /// Sets the score for the case when the first player chooses the second
    /// choice and the second player chooses the first choice.
    ///
    /// # Arguments
    ///
    /// * `score_ba` - The score to set.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameGridBuilder,
    ///     NumberPair,
    /// };
    ///
    /// let game_grid_builder =
    ///     GameGridBuilder::new().score_ba(NumberPair::new(1, 1));
    ///
    /// assert_eq!(game_grid_builder.score_ba, Some(NumberPair::new(1, 1)));
    /// ```
    ///
    /// # Returns
    ///
    /// The `GameGridBuilder` struct with the `score_ba` field set.
    ///
    ///
    /// # See Also
    ///
    /// * [`GameGridBuilder::new()`](GameGridBuilder::new())
    /// * [`GameGridBuilder::build()`](GameGridBuilder::build())
    /// * [`GameGridBuilder::max_value()`](GameGridBuilder::max_value())
    /// * [`GameGridBuilder::min_value()`](GameGridBuilder::min_value())
    /// * [`GameGridBuilder::choice_atlantis()`](GameGridBuilder::choice_atlantis())
    /// * [`GameGridBuilder::choice_olympus()`](GameGridBuilder::choice_olympus())
    /// * [`GameGridBuilder::score_aa()`](GameGridBuilder::score_aa())
    /// * [`GameGridBuilder::score_ab()`](GameGridBuilder::score_ab())
    /// * [`GameGridBuilder::score_bb()`](GameGridBuilder::score_bb())
    #[must_use]
    pub const fn score_ba(mut self, score_ba: NumberPair) -> Self {
        self.score_ba = Some(score_ba);
        self
    }

    /// Sets the score for the case when both players choose the second choice.
    ///
    /// # Arguments
    ///
    /// * `score_bb` - The score to set.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameGridBuilder,
    ///     NumberPair,
    /// };
    ///
    /// let game_grid_builder =
    ///     GameGridBuilder::new().score_bb(NumberPair::new(1, 1));
    ///
    /// assert_eq!(game_grid_builder.score_bb, Some(NumberPair::new(1, 1)));
    /// ```
    ///
    /// # Returns
    ///
    /// The `GameGridBuilder` struct with the `score_bb` field set.
    ///
    ///
    /// # See Also
    ///
    /// * [`GameGridBuilder::new()`](GameGridBuilder::new())
    /// * [`GameGridBuilder::build()`](GameGridBuilder::build())
    /// * [`GameGridBuilder::max_value()`](GameGridBuilder::max_value())
    /// * [`GameGridBuilder::min_value()`](GameGridBuilder::min_value())
    /// * [`GameGridBuilder::choice_atlantis()`](GameGridBuilder::choice_atlantis())
    /// * [`GameGridBuilder::choice_olympus()`](GameGridBuilder::choice_olympus())
    /// * [`GameGridBuilder::score_aa()`](GameGridBuilder::score_aa())
    /// * [`GameGridBuilder::score_ab()`](GameGridBuilder::score_ab())
    /// * [`GameGridBuilder::score_ba()`](GameGridBuilder::score_ba())
    #[must_use]
    pub const fn score_bb(mut self, score_bb: NumberPair) -> Self {
        self.score_bb = Some(score_bb);
        self
    }

    /// Builds the `GameGrid` struct.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::{
    ///     GameGrid,
    ///     GameGridBuilder,
    ///     NumberPair,
    /// };
    ///
    /// let game_grid = GameGridBuilder::new()
    ///     .max_value(10)
    ///     .min_value(1)
    ///     .choice_atlantis("cooperate".to_string())
    ///     .choice_olympus("defect".to_string())
    ///     .score_aa(NumberPair::new(1, 1))
    ///     .score_ab(NumberPair::new(1, 1))
    ///     .score_ba(NumberPair::new(1, 1))
    ///     .score_bb(NumberPair::new(1, 1))
    ///     .build();
    ///
    /// assert_eq!(game_grid.max_value(), 10);
    /// assert_eq!(game_grid.min_value(), 1);
    /// assert_eq!(game_grid.choice_atlantis(), "cooperate");
    /// assert_eq!(game_grid.choice_olympus(), "defect");
    /// assert_eq!(game_grid.score_aa(), NumberPair::new(1, 1));
    /// assert_eq!(game_grid.score_ab(), NumberPair::new(1, 1));
    /// assert_eq!(game_grid.score_ba(), NumberPair::new(1, 1));
    /// assert_eq!(game_grid.score_bb(), NumberPair::new(1, 1));
    /// ```
    ///
    /// # Returns
    ///
    /// A new `GameGrid` struct.
    ///
    /// # Panics
    ///
    /// This function will panic if any of the required fields are not set.
    ///
    /// # See Also
    ///
    /// * [`GameGridBuilder::new()`](GameGridBuilder::new())
    /// * [`GameGridBuilder::max_value()`](GameGridBuilder::max_value())
    /// * [`GameGridBuilder::min_value()`](GameGridBuilder::min_value())
    /// * [`GameGridBuilder::choice_atlantis()`](GameGridBuilder::choice_atlantis())
    /// * [`GameGridBuilder::choice_olympus()`](GameGridBuilder::choice_olympus())
    /// * [`GameGridBuilder::score_aa()`](GameGridBuilder::score_aa())
    /// * [`GameGridBuilder::score_ab()`](GameGridBuilder::score_ab())
    /// * [`GameGridBuilder::score_ba()`](GameGridBuilder::score_ba())
    /// * [`GameGridBuilder::score_bb()`](GameGridBuilder::score_bb())
    #[must_use]
    pub fn build(self) -> GameGrid<'a> {
        let max_value = self.max_value.unwrap_or(10);
        let min_value = self.min_value.unwrap_or(1);
        let choice_atlantis = self.choice_atlantis.unwrap_or("cooperate");
        let choice_olympus = self.choice_olympus.unwrap_or("defect");
        let score_aa = self.score_aa.unwrap_or(NumberPair::new(4, 4));
        let score_ab = self.score_ab.unwrap_or(NumberPair::new(5, 0));
        let score_ba = self.score_ba.unwrap_or(NumberPair::new(0, 5));
        let score_bb = self.score_bb.unwrap_or(NumberPair::new(3, 3));
        let game_options = GameOptions::new(min_value, max_value, choice_atlantis, choice_olympus);

        GameGrid {
            game_options,
            score_aa,
            score_ab,
            score_ba,
            score_bb,
        }
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
            .choice_atlantis("A")
            .choice_olympus("B")
            .score_aa(NumberPair::new(1, 1))
            .score_ab(NumberPair::new(1, 1))
            .score_ba(NumberPair::new(1, 1))
            .score_bb(NumberPair::new(1, 1))
            .build();

        assert_eq!(game_grid.max_value(), 10);
        assert_eq!(game_grid.min_value(), 1);
        assert_eq!(game_grid.choice_atlantis(), "A");
        assert_eq!(game_grid.choice_olympus(), "B");
        assert_eq!(game_grid.score_aa(), NumberPair::new(1, 1));
        assert_eq!(game_grid.score_ab(), NumberPair::new(1, 1));
        assert_eq!(game_grid.score_ba(), NumberPair::new(1, 1));
        assert_eq!(game_grid.score_bb(), NumberPair::new(1, 1));
    }

    #[test]
    fn test_default_builder() {
        let game_grid = GameGridBuilder::new().build();

        assert_eq!(game_grid.max_value(), 10);
        assert_eq!(game_grid.min_value(), 1);
        assert_eq!(game_grid.choice_atlantis(), "cooperate");
        assert_eq!(game_grid.choice_olympus(), "defect");
        assert_eq!(game_grid.score_aa(), NumberPair::new(4, 4));
        assert_eq!(game_grid.score_ab(), NumberPair::new(5, 0));
        assert_eq!(game_grid.score_ba(), NumberPair::new(0, 5));
        assert_eq!(game_grid.score_bb(), NumberPair::new(3, 3));
    }
}
