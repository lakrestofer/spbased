//! This library contains a simpler implementation of the FSRS algorithm developed by Jarrett Ye.
//! TODO update to https://expertium.github.io/Algorithm.html

pub mod model {
    pub(crate) mod grade_ops;

    use serde::{Deserialize, Serialize};

    /// The stability of a memory. The number of days until the probability of recall reaches 90%
    pub type Stability = f32;
    /// The difficulty of a memory.
    pub type Difficulty = f32;
    /// The probability that an item will be recalled.
    pub type Retrievability = f32;
    /// Time in number of days.
    pub type Time = f32;
    /// Time between review events in number of days
    pub type Interval = f32;

    /// Each review prompt has some parameters that we use to schedule it
    #[derive(Serialize, Deserialize)]
    pub struct ModelData {
        pub s: Stability,
        pub d: Difficulty,
    }

    /// When reviewing an item using its associated prompt
    /// we need grade how well we could satisfy the prompt
    /// 1 -> Again, user could not recall
    /// 2 -> Hard, the user could recall, but with great effort
    /// 3 -> Good, the user could recall
    /// 4 -> Easy, the user could easily recall
    #[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Grade {
        Again = 1,
        Hard = 2,
        Good = 3,
        Easy = 4,
    }
}
use model::*;
use Grade::*;

pub const ALGORITHM_VERSION: usize = 5;

/// Weights. Current best weights for the FSRS algorithm
const W: [f32; 19] = [
    0.40255, 1.18385, 3.173, 15.69105, 7.1949, 0.5345, 1.4604, 0.0046, 1.54575, 0.1192, 1.01925,
    1.9395, 0.11, 0.29605, 2.2698, 0.2315, 2.9898, 0.51655, 0.6621,
];

/// factor
const F: f32 = 19.0 / 81.0;
/// decay
const D: f32 = -0.5;

use model::{Difficulty, Retrievability, Stability, Time};

/// retrievability - the probability after t days that prompt will be satisfied
pub fn r(t: Time, s: Stability) -> Retrievability {
    (1.0 + F * (t / s)).powf(D)
}

/// interval - the amount of days until retrievability reaches [`r`], rounded away from zero
/// `i(0.9, s) == s`
pub fn i(r: Retrievability, s: Stability) -> Interval {
    s / F * (r.powf(1.0 / D) - 1.0)
}

pub mod update {
    use super::*;

    pub mod success {
        use super::*;

        pub fn s(s: Stability, d: Difficulty, r: Retrievability, g: Grade) -> Stability {
            assert!(g != Grade::Again); // this function should not be called for a failing grade
            let scaling: f32 = {
                let (w_15, w_16) = match g {
                    Again => unreachable!(),
                    Hard => (W[15], 1.0),
                    Good => (1.0, 1.0),
                    Easy => (1.0, W[16]),
                };
                w_15 * w_16 * W[8].exp()
            };
            let f_d: f32 = 11.0 - d;
            let f_s: f32 = s.powf(-W[9]);
            let f_r: f32 = (W[10] * (1.0 - r)).exp() - 1.0;
            let s_inc = 1.0 + scaling * f_d * f_s * f_r;
            s.min(s * s_inc)
        }
    }

    pub mod fail {
        use super::*;
        pub fn s(s: Stability, d: Difficulty, r: Retrievability) -> Stability {
            s.min(
                W[11] * d.powf(-W[12]) * ((s + 1.0).powf(W[13]) - 1.0) * (W[14] * (1.0 - r)).exp(),
            )
        }
    }

    /// Update rule for difficulty. The difficulty is updated depending on the grade as below
    /// - Again -> increase
    /// - Hard -> increase a little bit
    /// - Good -> nothing
    /// - Easy -> subtract
    /// NOTE: does not take retrievability into account.
    pub fn d(d: Difficulty, g: Grade) -> Difficulty {
        let delta_d = -W[6] * (g - 3.0); // change in terms of grade
        let mean_revision = W[7] * init::d(Hard) + (1.0 - W[7]); // bias the difficulty in the direction of a d==init:d(Hard)
        mean_revision * (d + delta_d * (10.0 - D) / 9.0).clamp(0.0, 10.0) // approach
    }

    pub mod shortterm {
        use super::*;
        /// Stability update rule for short term reviews
        pub fn s(s: Stability, g: Grade) -> Stability {
            s * (W[17] * (g - 3.0 + W[18])).exp()
        }
    }
}

pub mod init {
    use super::*;

    /// Initial stability
    pub fn s(g: Grade) -> Stability {
        W[g - 1]
    }

    /// Initial difficulty
    pub fn d(g: Grade) -> Difficulty {
        W[4] - (W[5] * (g - 1.0)).exp() + 1.0
    }
}
