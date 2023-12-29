const PARAMS: [f64; 17] = [
    0.4, 0.6, 2.4, 5.8, 4.93, 0.94, 0.86, 0.01, 1.49, 0.14, 0.94, 2.18, 0.05, 0.34, 1.26, 0.29,
    2.61,
];
// initial stabilities
const AGAIN_STABILITY: f64 = PARAMS[0];
const HARD_STABILITY: f64 = PARAMS[1];
const PASS_STABILITY: f64 = PARAMS[2];
const EASY_STABILITY: f64 = PARAMS[3];
// initial difficulty weights
const INIT_DIFFICULTY_WEIGHT_1: f64 = PARAMS[4];
const INIT_DIFFICULTY_WEIGHT_2: f64 = PARAMS[5];
// update difficulty weights
const UPDATE_DIFFICULTY_WEIGHT_1: f64 = PARAMS[6];
const UPDATE_DIFFICULTY_WEIGHT_2: f64 = PARAMS[7];
// update stability weights
const UPDATE_STABILITY_WEIGHT_1: f64 = PARAMS[8];
const UPDATE_STABILITY_WEIGHT_2: f64 = PARAMS[9];
const UPDATE_STABILITY_WEIGHT_3: f64 = PARAMS[10];
// update stability weight on fail
const UPDATE_STABILITY_FAIL_1: f64 = PARAMS[11];
const UPDATE_STABILITY_FAIL_2: f64 = PARAMS[12];
const UPDATE_STABILITY_FAIL_3: f64 = PARAMS[13];
const UPDATE_STABILITY_FAIL_4: f64 = PARAMS[14];
// update stability weight on success
const UPDATE_STABILITY_SUCCESS_1: f64 = PARAMS[15];
const UPDATE_STABILITY_SUCCESS_2: f64 = PARAMS[16];

/// Enum representing possible review results
#[derive(Clone, Copy)]
pub enum Grade {
    Again = 1,
    Hard = 2,
    Pass = 3,
    Easy = 4,
}

pub struct ConversionError;

impl TryFrom<i32> for Grade {
    type Error = ConversionError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Grade::Again),
            2 => Ok(Grade::Hard),
            3 => Ok(Grade::Pass),
            4 => Ok(Grade::Easy),
            _ => Err(ConversionError),
        }
    }
}

/// given the first review event, what stability
/// (number of days untill recall probability reaches 90%)
/// do we estimate it to have?
pub fn stability_init(g: Grade) -> f64 {
    match g {
        Grade::Again => AGAIN_STABILITY,
        Grade::Hard => HARD_STABILITY,
        Grade::Pass => PASS_STABILITY,
        Grade::Easy => EASY_STABILITY,
    }
}

/// given the first review event, what difficulty do we estimate it to have?
pub fn difficulty_init(g: Grade) -> f64 {
    INIT_DIFFICULTY_WEIGHT_1 - (g as u8 as f64) * INIT_DIFFICULTY_WEIGHT_2
}

/// given a review event, how should we update a review items difficulty?
pub fn update_difficulty(d: f64, g: Grade) -> f64 {
    UPDATE_DIFFICULTY_WEIGHT_2 * difficulty_init(Grade::Pass)
        + (1.0 - UPDATE_DIFFICULTY_WEIGHT_2)
            * (d - UPDATE_DIFFICULTY_WEIGHT_1 * (g as u8 as f64 - 3.0))
}

/// the probability that review item will be recalled after n_days
pub fn recall(n_days: f64, stability: f64) -> f64 {
    1.0 / (1.0 + (n_days / (9.0 * stability)))
}

/// the number of days until the probability of recalling the
/// review_item reaches r
pub fn interval(recall_probability: f64, stability: f64) -> f64 {
    9.0 * stability * (1.0 / recall_probability - 1.0)
}

/// given a reviev even were we did recall the item, how should we update its stability
fn update_stability_on_recall(difficulty: f64, stability: f64, grade: Grade, n_days: f64) -> f64 {
    let recall = recall(n_days, stability);
    let grade_factor = match grade {
        Grade::Hard => UPDATE_STABILITY_SUCCESS_1,
        Grade::Easy => UPDATE_STABILITY_SUCCESS_2,
        _ => 1.0,
    };
    let stability_scaling = (UPDATE_STABILITY_WEIGHT_1.exp() * (11.0 - difficulty))
        * stability.powf(-UPDATE_STABILITY_WEIGHT_2)
        * ((UPDATE_STABILITY_WEIGHT_3 * (1.0 - recall)).exp() - 1.0)
        * grade_factor;
    stability * stability_scaling
}

/// given a reviev even were we did not recall the item, how should we update its stability
fn update_stability_on_forget(difficulty: f64, stability: f64, n_days: f64) -> f64 {
    let r = recall(n_days, stability);
    UPDATE_STABILITY_FAIL_1
        * difficulty.powf(-UPDATE_STABILITY_FAIL_2)
        * ((stability + 1.0).powf(UPDATE_STABILITY_FAIL_3) - 1.0)
        * (UPDATE_STABILITY_FAIL_4 * (1.0 - r)).exp()
}

/// given a review event, how should we update a review items stability?
pub fn update_stability(difficulty: f64, stability: f64, grade: Grade, n_days: f64) -> f64 {
    match grade {
        Grade::Again => update_stability_on_forget(difficulty, stability, n_days),
        g => update_stability_on_recall(difficulty, stability, g, n_days),
    }
}
