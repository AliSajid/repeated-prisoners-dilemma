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

use clap::{Parser, ValueEnum};

/// The strategy to use for the prisoner's dilemma.
///
/// The strategy is used to determine whether to cooperate or defect.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
enum Strategy {
    AlwaysCooperate,
    AlwaysDefect,
    TitForTat,
    TitForTwoTats,
    Random,
}

#[derive(Parser, Debug)]
#[command(
    author = "Ali Sajid Imami",
    version,
    about,
    name = "Iterated Prisoner's Dilemma"
)]
pub struct Cli {
    /// Number of Iterations
    ///
    /// This argument specifies the number of iterations to run the prisoner's dilemma.
    /// This can be used to run the prisoner's dilemma for a fixed number of iterations and then analyze the results.
    #[clap(short, long, value_name = "ITERATIONS", default_value_t = 100)]
    num_iterations: u32,

    /// Strategy
    ///
    /// This argument specifies the strategy to use for the prisoner's dilemma.
    /// The strategy is used to determine whether to cooperate or defect.
    ///
    /// The following strategies are available:
    /// 1. AlwaysCooperate - Always cooperate.
    /// 2. AlwaysDefect - Always defect.
    /// 3. TitForTat - Cooperate on the first iteration, then copy the opponent's previous move.
    /// 4. TitForTwoTats - Cooperate on the first two iterations, then defect if the opponent defects twice in a row.
    /// 5. Random - Randomly cooperate or defect.
    #[clap(short, long, value_enum, value_name = "STRATEGY", default_value_t = Strategy::TitForTat)]
    strategy: Strategy,

    /// Random Payoff
    ///
    /// This argument specifies whether to use random payoffs for the prisoner's dilemma.
    ///
    /// If this argument is set to true, then the payoffs for the prisoner's dilemma will be randomly generated.
    /// Otherwise, the payoffs will be set to the following:
    /// 1. Reward - 3
    /// 2. Sucker - 0
    /// 3. Temptation - 5
    /// 4. Punishment - 1
    #[clap(short, long, default_value_t = true)]
    random_payoff: bool,
}
