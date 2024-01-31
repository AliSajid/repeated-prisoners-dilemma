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

// Import the fmt module for formatting
use std::fmt;

// Define an enum called Choice with two variants: Atlantis and Olympus
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Choice {
    Atlantis,
    Olympus,
}

// Implement methods for the Choice enum
impl Choice {
    // Define a method to return the Atlantis variant of Choice
    // Example: let my_choice = Choice::atlantis(); // my_choice is now
    // Choice::Atlantis
    #[must_use]
    pub const fn atlantis() -> Self {
        Self::Atlantis
    }

    // Define a method to return the Olympus variant of Choice
    // Example: let my_choice = Choice::olympus(); // my_choice is now
    // Choice::Olympus
    #[must_use]
    pub const fn olympus() -> Self {
        Self::Olympus
    }
}

// Implement the Display trait for the Choice enum
// This allows a Choice to be formatted as a string using the "{}" format
// specifier
impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write the string representation of the Choice to the provided formatter
        // Example: println!("{}", Choice::Atlantis); // Prints: Atlantis
        write!(f, "{self:?}")
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_choice_to_string() {
        assert_eq!(Choice::Atlantis.to_string(), "Atlantis");

        assert_eq!(Choice::Olympus.to_string(), "Olympus");
    }

    #[test]
    fn test_choice_atlantis() {
        assert_eq!(Choice::atlantis(), Choice::Atlantis);
    }

    #[test]
    fn test_choice_olympus() {
        assert_eq!(Choice::olympus(), Choice::Olympus);
    }

    #[test]
    fn test_choice_display() {
        assert_eq!(format!("{}", Choice::Atlantis), "Atlantis");

        assert_eq!(format!("{}", Choice::Olympus), "Olympus");
    }
}
