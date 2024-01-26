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

use std::fmt::{
    self,
    Display,
    Formatter,
};

use rand::Rng;

/// A convenience struct to represent a pair of numbers.
///
/// This struct is primarily here to be used as a stand-in for the
/// prize values for the Dilemma game.
///
/// # Example
///
/// ```
/// use dilemma_tactix_lib::NumberPair;
///
/// let number_pair = NumberPair::new(1, 2);
/// assert_eq!(number_pair.first(), 1);
/// assert_eq!(number_pair.second(), 2);
/// ```
///
/// ```
/// use dilemma_tactix_lib::NumberPair;
///
/// let number_pair = NumberPair::default();
/// assert!(number_pair.first() <= 10);
/// assert!(number_pair.second() <= 10);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NumberPair {
    /// The first number in the pair.
    first:  u32,
    /// The second number in the pair.
    second: u32,
}

impl NumberPair {
    /// Creates a new `NumberPair` struct.
    ///
    /// # Arguments
    ///
    /// * `first` - The first number in the pair.
    /// * `second` - The second number in the pair.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::NumberPair;
    ///
    /// let number_pair = NumberPair::new(1, 2);
    /// assert_eq!(number_pair.first(), 1);
    /// assert_eq!(number_pair.second(), 2);
    /// ```
    ///
    /// # Returns
    ///
    /// A new `NumberPair` struct with the given values.
    #[must_use]
    pub const fn new(first: u32, second: u32) -> Self {
        Self { first, second }
    }

    /// Creates a new `NumberPair` struct with random values.
    ///
    /// # Arguments
    ///
    /// * `min_value` - The minimum value for that can be assigned to a choice.
    /// * `max_value` - The maximum value for that can be assigned to a choice.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::NumberPair;
    ///
    /// let number_pair = NumberPair::random(1, 10);
    /// assert!(number_pair.first() <= 10);
    /// assert!(number_pair.second() <= 10);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `min_value` is greater than `max_value`.
    ///
    /// # Returns
    ///
    /// A new `NumberPair` struct with random values between `min_value` and
    /// `max_value` for each of `first` and `second`.
    #[must_use]
    pub fn random(min_value: u32, max_value: u32) -> Self {
        if min_value < max_value {
            let mut rng = rand::thread_rng();
            Self::new(
                rng.gen_range(min_value..=max_value),
                rng.gen_range(min_value..=max_value),
            )
        } else {
            panic!("min_value must be less than max_value");
        }
    }

    /// Returns the value of `first`.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::NumberPair;
    ///
    /// let number_pair = NumberPair::new(1, 2);
    /// assert_eq!(number_pair.first(), 1);
    /// ```
    ///
    /// ```
    /// use dilemma_tactix_lib::NumberPair;
    ///
    /// let number_pair = NumberPair::default();
    ///
    /// assert!(number_pair.first() <= 10);
    /// ```
    ///
    /// # Returns
    ///
    /// The value of `first` as a `u32`.
    #[must_use]
    pub const fn first(&self) -> u32 {
        self.first
    }

    /// Returns the value of `second`.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::NumberPair;
    ///
    /// let number_pair = NumberPair::new(1, 2);
    /// assert_eq!(number_pair.second(), 2);
    /// ```
    ///
    /// ```
    /// use dilemma_tactix_lib::NumberPair;
    ///
    /// let number_pair = NumberPair::default();
    ///
    /// assert!(number_pair.second() <= 10);
    /// ```
    ///
    /// # Returns
    ///
    /// The value of `second` as a `u32`.
    #[must_use]
    pub const fn second(&self) -> u32 {
        self.second
    }
}

impl Display for NumberPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.first, self.second)
    }
}

impl Default for NumberPair {
    /// Creates a new `NumberPair` struct with default values.
    ///
    /// # Example
    ///
    /// ```
    /// use dilemma_tactix_lib::NumberPair;
    ///
    /// let number_pair = NumberPair::default();
    /// assert!(number_pair.first() <= 10);
    /// assert!(number_pair.second() <= 10);
    /// ```
    ///
    /// # Returns
    ///
    /// A new `NumberPair` struct with random values between 1 and 10 for each
    /// of `first` and `second`.
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        Self::new(rng.gen_range(1..=10), rng.gen_range(1..=10)) // TODO - Make
                                                                // this range
                                                                // 0-9
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_pair_display() {
        let number_pair = NumberPair::new(1, 2);
        assert_eq!(format!("{}", number_pair), "(1, 2)");
    }

    #[test]
    fn test_number_pair_default() {
        let number_pair = NumberPair::default();
        assert!(number_pair.first() <= 10);
        assert!(number_pair.second() <= 10);
    }

    #[test]
    fn test_number_pair_random() {
        let number_pair = NumberPair::random(1, 10);
        assert!(number_pair.first() <= 10);
        assert!(number_pair.second() <= 10);
    }

    #[test]
    fn test_number_pair_new() {
        let number_pair = NumberPair::new(1, 2);
        assert_eq!(number_pair.first(), 1);
        assert_eq!(number_pair.second(), 2);
    }

    #[test]
    fn test_number_pair_first() {
        let number_pair = NumberPair::new(1, 2);
        assert_eq!(number_pair.first(), 1);
    }

    #[test]
    fn test_number_pair_second() {
        let number_pair = NumberPair::new(1, 2);
        assert_eq!(number_pair.second(), 2);
    }
}
