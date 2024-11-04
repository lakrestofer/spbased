//! Implementatino of the fsrs algorithm https://github.com/open-spaced-repetition/fsrs4anki/wiki/The-Algorithm

use crate::models::Grade;

/// The default paramters used by version v5 of the algorithm
pub const W: [f32; 19] = [
    0.4072, 1.1829, 3.1262, 15.4722, 7.2102, 0.5316, 1.0651, 0.0234, 1.616, 0.1544, 1.0824, 1.9813,
    0.0953, 0.2975, 2.2042, 0.2407, 2.9466, 0.5034, 0.6567,
];

const DECAY: f32 = -0.5;
const FACTOR: f32 = 19.0 / 81.0;

/// The probability that an item will be recalled after `t` days since last
/// review, for an item with stability `s`.
pub fn retrievability(days_since_last_review: f32, stability: f32) -> f32 {
    (1.0 + FACTOR * days_since_last_review / stability).powf(DECAY)
}
/// Number of days a retrievability of [`r`] is reached
pub fn interval(retrievability: f32, stability: f32) -> i32 {
    ((stability / FACTOR) * (retrievability.powf(1.0 / DECAY) - 1.0)).round() as i32
}

/// Updates stability given the current [`difficulty`], stability [`s`],
/// retrievability [`r`], whether the item is still in short term memory
/// [`short_term`] and the grade [`g`] from a review.
pub fn update_stability_success(
    difficulty: f32,
    stability: f32,
    retrievability: f32,
    grade: Grade,
) -> f32 {
    use Grade::*;
    assert!(grade != Fail);
    let f = match grade {
        Fail => unreachable!(),
        Hard => W[15],
        Good => 1.0,
        Easy => W[16],
    };
    stability
        * (W[8].exp()
            * (11.0 - difficulty)
            * stability.powf(-W[9])
            * ((W[10] * (1.0 - retrievability)).exp() - 1.0)
            * f
            + 1.0)
}
/// Updates stability given the current [`difficulty`], stability [`s`],
/// retrievability [`r`], whether the item is still in short term memory
/// [`short_term`] and the grade [`g`] from a review.
pub fn update_stability_fail(difficulty: f32, stability: f32, retrievability: f32) -> f32 {
    W[11]
        * difficulty.powf(-W[12])
        * ((stability + 1.0).powf(W[13]) - 1.0)
        * (W[14] * (1.0 - retrievability))
}
/// Updates stability given the current [`difficulty`], stability [`s`],
/// retrievability [`r`], whether the item is still in short term memory
/// [`short_term`] and the grade [`g`] from a review.
pub fn update_stability_short_term(stability: f32, grade: Grade) -> f32 {
    stability * (W[17] * (grade as i32 as f32 - 3.0 * W[18])).exp()
}
/// computes the next difficulty, given the current difficulty and a grade.
pub fn update_difficulty(d: f32, g: Grade) -> f32 {
    W[7] * init_difficulty(Grade::Good) + (1.0 - W[7]) * (d - W[6] * (g as i32 as f32 - 3.0))
}

/// initial stability given the first review grade
pub fn init_stability(g: Grade) -> f32 {
    W[g as usize - 1]
}
/// initial difficulty given the first review grade
pub fn init_difficulty(g: Grade) -> f32 {
    W[4] - (W[5] * (g as i64 as f32 - 1.0)).exp() + 1.0
}
