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
use rand_chacha::ChaCha12Rng;

/// A data struct that holds the names of the choices.
///
/// This is a simple data struct that is used to internally store
/// the names of choice pairs that can be given for a single game.
///
/// The names are taken from the [Universal Paperclips](https://www.decisionproblem.com/paperclips/index2.html)
/// game, and are used to make the game more interesting.
///
/// The names are stored in two arrays, one for each choice. The
/// names are then accessed by index, and returned as a pair. The table
/// of all choices and their respective indices can be found
/// [below](#available-options).
///
/// The struct also provides a way to get a random pair of names.
///
/// # Examples
///
/// ## Get a specific pair
///
/// ```
/// use dilemma_tactix_lib::ChoiceNameOptions;
///
/// let choice_name_options = ChoiceNameOptions::new();
/// let (choice_atlantis, choice_olympus) =
///     choice_name_options.get_choice_pair(0);
///
/// assert_eq!(choice_atlantis, "cooperate");
/// assert_eq!(choice_olympus, "defect");
/// ```
///
/// ## Get a random pair
/// ```
/// use dilemma_tactix_lib::ChoiceNameOptions;
///
/// let choice_name_options = ChoiceNameOptions::new();
/// let (choice_atlantis, choice_olympus) =
///     choice_name_options.get_random_pair();
/// assert_ne!(choice_atlantis, choice_olympus);
/// ```
///
/// # Notes
///
/// The names are taken from the [Universal Paperclips](https://www.decisionproblem.com/paperclips/index2.html).
///
/// I have also decided to make a static array that can be stored in the binary
/// itself to make it easier to distribute the game. This means that the names
/// are not configurable, and are the same for every game.
///
/// For this reason, you would usually want to use the `get_random_pair`
/// function, as it will return a random pair of names.
///
/// # Available Options
///
/// | Index | Choice Atlantis | Choice Olympus |
/// | ----- | -------- | -------- |
/// | 0     | cooperate | defect |
/// | 1     | swerve | straight |
/// | 2     | macro | micro |
/// | 3     | fight | back_down |
/// | 4     | bet | fold |
/// | 5     | raise_price | lower_price |
/// | 6     | opera | football |
/// | 7     | go | stay |
/// | 8     | heads | tails |
/// | 9     | particle | wave |
/// | 10    | discrete | continuous |
/// | 11    | peace | war |
/// | 12    | search | evaluate |
/// | 13    | lead | follow |
/// | 14    | accept | reject |
/// | 15    | accept | deny |
/// | 16    | attack | decay |
///
/// # See Also
///
/// * [Universal Paperclips](https://www.decisionproblem.com/paperclips/index2.html)
/// * [get_random_pair](ChoiceNameOptions::get_random_pair)
/// * [get_choice_pair](ChoiceNameOptions::get_choice_pair)
#[derive(Debug, Clone, Copy)]
#[allow(clippy::doc_markdown)]
pub struct ChoiceNameOptions {
    pub choice_atlantis_options: [&'static str; 17],
    pub choice_olympus_options:  [&'static str; 17],
    pub length:                  usize,
}

impl ChoiceNameOptions {
    /// Create a new `ChoiceNameOptions` struct.
    ///
    /// This function creates a new `ChoiceNameOptions` struct, and
    /// can then be used to generate a random name pair or extract a specific
    /// name pair.
    ///
    /// # Examples
    ///
    /// ```
    /// use dilemma_tactix_lib::ChoiceNameOptions;
    ///
    /// let choice_name_options = ChoiceNameOptions::new();
    ///
    /// let (choice_atlantis, choice_olympus) =
    ///     choice_name_options.get_choice_pair(0);
    ///
    /// assert_eq!(choice_atlantis, "cooperate");
    /// assert_eq!(choice_olympus, "defect");
    /// ```
    ///
    /// # Returns
    ///
    /// A new `ChoiceNameOptions` struct.
    ///
    /// # See Also
    ///
    /// * [`get_random_pair`](ChoiceNameOptions::get_random_pair)
    /// * [`get_choice_pair`](ChoiceNameOptions::get_choice_pair)
    #[must_use]
    pub const fn new() -> Self {
        let choice_atlantis_options = [
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
        let choice_olympus_options = [
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
        let length_atlantis = choice_atlantis_options.len();
        let length_olympus = choice_olympus_options.len();
        if length_atlantis != length_olympus {
            panic!("Atlantis and Olympus arrays are not the same length.");
        } else {
            let length = length_atlantis;
            Self {
                choice_atlantis_options,
                choice_olympus_options,
                length,
            }
        }
    }

    /// Get a specific choice pair.
    ///
    /// This function returns a specific choice pair, based on the index
    /// provided. The index must be less than the length of the array.
    ///
    /// # Arguments
    ///
    /// * `n` - The index of the choice pair to return.
    ///
    /// # Examples
    ///
    /// ```
    /// use dilemma_tactix_lib::ChoiceNameOptions;
    ///
    /// let choice_name_options = ChoiceNameOptions::new();
    ///
    /// let (choice_atlantis, choice_olympus) =
    ///     choice_name_options.get_choice_pair(0);
    ///
    /// assert_eq!(choice_atlantis, "cooperate");
    /// assert_eq!(choice_olympus, "defect");
    /// ```
    ///
    /// # Returns
    ///
    /// A tuple containing the two choices.
    ///
    /// # Panics
    ///
    /// This function will panic if the index is greater than the length of the
    /// array.
    ///
    /// # See Also
    ///
    /// * [`get_random_pair`](ChoiceNameOptions::get_random_pair)
    #[must_use]
    pub const fn get_choice_pair(&self, n: usize) -> (&'static str, &'static str) {
        if n < self.length {
            return (
                self.choice_atlantis_options[n],
                self.choice_olympus_options[n],
            );
        }
        panic!("Index out of bounds.");
    }

    /// Get a random choice pair.
    ///
    /// This function returns a random choice pair, based on the length
    /// of the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use dilemma_tactix_lib::ChoiceNameOptions;
    ///
    /// let choice_name_options = ChoiceNameOptions::new();
    ///
    /// let (choice_atlantis, choice_olympus) =
    ///     choice_name_options.get_random_pair();
    /// assert_ne!(choice_atlantis, choice_olympus);
    /// ```
    ///
    /// # Returns
    ///
    /// A tuple containing the two choices.
    ///
    /// # See Also
    ///
    /// * [`get_choice_pair`](ChoiceNameOptions::get_choice_pair)
    ///
    /// # Notes
    ///
    /// This function uses the [`rand`](https://crates.io/crates/rand) crate to generate a random number.
    ///
    /// # See Also
    ///
    /// * [`rand`](https://crates.io/crates/rand)
    /// * [`get_choice_pair`](ChoiceNameOptions::get_choice_pair)
    #[must_use]
    pub fn get_random_pair(&self) -> (&'static str, &'static str) {
        let mut rng = ChaCha12Rng::from_entropy();
        let choice = rng.gen_range(0..self.length);
        self.get_choice_pair(choice)
    }

    /// Get a random choice pair seeded.
    ///
    /// This function returns a random choice pair, based on the length
    /// of the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use dilemma_tactix_lib::ChoiceNameOptions;
    ///
    /// let choice_name_options = ChoiceNameOptions::new();
    ///
    /// let (choice_atlantis, choice_olympus) =
    ///     choice_name_options.get_random_pair_seeded(2024);
    /// assert_ne!(choice_atlantis, choice_olympus);
    /// assert_eq!(choice_atlantis, "discrete");
    /// assert_eq!(choice_olympus, "continuous");
    /// ```
    ///
    /// # Returns
    ///
    /// A tuple containing the two choices.
    ///
    /// # See Also
    ///
    /// * [`get_choice_pair`](ChoiceNameOptions::get_choice_pair)
    ///
    /// # Notes
    ///
    /// This function uses the [`rand`](https://crates.io/crates/rand) crate to generate a random number.
    ///
    /// # See Also
    ///
    /// * [`rand`](https://crates.io/crates/rand)
    /// * [`get_choice_pair`](ChoiceNameOptions::get_choice_pair)
    #[cfg(test)]
    #[doc(hidden)]
    pub fn get_random_pair_seeded(&self, seed: u64) -> (&'static str, &'static str) {
        let mut rng = ChaCha12Rng::seed_from_u64(seed);
        let choice = rng.gen_range(0..self.length);
        self.get_choice_pair(choice)
    }
}

#[cfg(test)]
mod tests {
    use rstest::{
        fixture,
        rstest,
    };

    use super::*;

    // This fixture provides a fixed seed for random number generation in tests
    #[fixture]
    fn seed() -> u64 {
        2024
    }

    // This test checks that the get_random_pair method returns two different
    // choices
    #[rstest]
    fn test_get_random_pair() {
        let choice_name_options = ChoiceNameOptions::new();
        let (choice_atlantis, choice_olympus) = choice_name_options.get_random_pair();
        // This assertion checks that the two choices are not the same,
        // ensuring that get_random_pair is correctly generating two distinct choices
        assert_ne!(choice_atlantis, choice_olympus);
        // This assertion checks that the choice_atlantis is pulled from the
        // choice_atlantis_options array, ensuring that get_random_pair is correctly
        // using the array to generate choices
        assert!(choice_name_options
            .choice_atlantis_options
            .contains(&choice_atlantis));
        // This assertion checks that the choice_olympus is pulled from the
        // choice_olympus_options array, ensuring that get_random_pair is correctly
        // using the array to generate choices
        assert!(choice_name_options
            .choice_olympus_options
            .contains(&choice_olympus));
    }

    // This test checks that the get_random_pair_seeded method returns the expected
    // choices for a given seed
    #[rstest]
    fn test_get_random_pair_seeded(seed: u64) {
        let choice_name_options = ChoiceNameOptions::new();
        let (choice_atlantis, choice_olympus) = choice_name_options.get_random_pair_seeded(seed);
        // This assertion checks that the two choices are not the same,
        // ensuring that get_random_pair_seeded is correctly generating two distinct
        // choices
        assert_ne!(choice_atlantis, choice_olympus);
        // These assertions check that the choices are as expected for the given seed,
        // ensuring that get_random_pair_seeded is correctly using the seed to generate
        // choices
        assert_eq!(choice_atlantis, "discrete");
        assert_eq!(choice_olympus, "continuous");
    }

    // This test checks that the get_random_pair_seeded method is repeatable for a
    // given seed
    #[rstest]
    fn test_get_random_pair_seeded_repeatable(seed: u64) {
        let choice_name_options = ChoiceNameOptions::new();
        let (choice_atlantis, choice_olympus) = choice_name_options.get_random_pair_seeded(seed);
        // This assertion checks that the two choices are not the same,
        // ensuring that get_random_pair_seeded is correctly generating two distinct
        // choices
        assert_ne!(choice_atlantis, choice_olympus);
        // These assertions check that the choices are as expected for the given seed,
        // ensuring that get_random_pair_seeded is correctly using the seed to generate
        // choices
        assert_eq!(choice_atlantis, "discrete");
        assert_eq!(choice_olympus, "continuous");
        // This part repeats the same checks, to ensure that the same seed will always
        // generate the same pair of choices
        let (choice_atlantis, choice_olympus) = choice_name_options.get_random_pair_seeded(seed);
        assert_ne!(choice_atlantis, choice_olympus);
        assert_eq!(choice_atlantis, "discrete");
        assert_eq!(choice_olympus, "continuous");
    }

    // This test checks that the get_choice_pair method returns the expected choices
    // for a given index
    #[rstest]
    fn test_get_choice_pair() {
        let choice_name_options = ChoiceNameOptions::new();
        let (choice_atlantis, choice_olympus) = choice_name_options.get_choice_pair(0);
        // These assertions check that the choices are as expected for the given index,
        // ensuring that get_choice_pair is correctly using the index to generate
        // choices
        assert_eq!(choice_atlantis, "cooperate");
        assert_eq!(choice_olympus, "defect");
    }
}
