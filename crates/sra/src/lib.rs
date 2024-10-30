//! This library contains a simpler implementation of the FSRS algorithm developed by Jarrett Ye.

pub mod model {

    /// Each review prompt has some parameters that we use to schedule it
    pub struct ModelData {
        pub stability: f32,
        pub difficulty: f32,
    }

    /// When reviewing an item using its associated prompt
    /// we need grade how well we could satisfy the prompt
    /// 1 -> fail
    /// 2 -> hard
    /// 3 -> ok
    /// 4 -> easy
    pub enum Grade {
        Fail = 1,
        Hard = 2,
        Ok = 3,
        Easy = 4,
    }
}
use model::*;
use Grade::*;

/// Weights
const W: [f32; 19] = [
    0.4072, 1.1829, 3.1262, 15.4722, 7.2102, 0.5316, 1.0651, 0.0234, 1.616, 0.1544, 1.0824, 1.9813,
    0.0953, 0.2975, 2.2042, 0.2407, 2.9466, 0.5034, 0.6567,
];

/// factor
const F: f32 = 19.0 / 81.0;
/// decay
const D: f32 = -0.5;

/// retrievability - the probability after t days that prompt will be satisfied
pub fn retrievability(t: f32, s: f32) -> f32 {
    (1.0 + F * t / s).powf(D)
}

/// interval - the amount of days until retrievability reaches [`r`], rounded away from zero
/// `i(0.9, s) == s`
pub fn interval(r: f32, s: f32) -> f32 {
    s / F * (r.powf(1.0 / D) - 1.0)
}

pub mod update {
    use super::*;

    pub mod success {
        use super::*;

        pub fn stability(s: f32, d: f32, r: f32, g: Grade) -> f32 {
            let f = match g {
                Hard => W[15],
                Easy => W[16],
                _ => 1.0,
            };

            s * (W[8].exp() * (11.0 - d) * s.powf(-W[9]) * ((W[10] * (1.0 - r)).exp() - 1.0) * f)
        }
    }

    pub mod fail {
        use super::*;
        pub fn stability(s: f32, d: f32, r: f32) -> f32 {
            W[11] * d.powf(-W[12]) * ((s + 1.0).powf(W[13]) - 1.0) * (W[14] * (1.0 - r)).exp()
        }
    }

    pub fn difficulty(d: f32, g: Grade) -> f32 {
        W[7] * init::difficulty(Ok) + (1.0 - W[7]) * (d - W[6] * (g as i32 as f32 - 3.0))
    }

    pub mod shortterm {
        use super::*;
        pub fn stability(s: f32, g: Grade) -> f32 {
            s * (W[17] * (g as i32 as f32 - 3.0 + W[18])).exp()
        }
    }
}

pub mod init {
    use super::*;

    pub fn stability(g: Grade) -> f32 {
        W[g as i32 as usize]
    }

    /// the initial difficulty given a review grade
    pub fn difficulty(g: Grade) -> f32 {
        W[4] - (W[5] * (g as i32 as f32 - 1.0)).exp() + 1.0
    }
}
