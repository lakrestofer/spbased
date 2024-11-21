use super::*;
use std::ops::{Add, Div, Mul, Sub};

macro_rules! impl_grade_op {
    ($trait:ident, $op_fn:ident, $type:ty) => {
        impl $trait<$type> for Grade {
            type Output = $type;
            fn $op_fn(self, rhs: $type) -> Self::Output {
                Into::<$type>::into(self).$op_fn(rhs)
            }
        }
    };
}

impl From<Grade> for usize {
    fn from(g: Grade) -> Self {
        g as usize
    }
}

impl From<Grade> for f32 {
    fn from(g: Grade) -> Self {
        g as usize as f32
    }
}

// implement artithmetic for floats
impl_grade_op!(Add, add, f32);
impl_grade_op!(Mul, mul, f32);
impl_grade_op!(Sub, sub, f32);
impl_grade_op!(Div, div, f32);

// implement arithmetic for usize
impl_grade_op!(Add, add, usize);
impl_grade_op!(Mul, mul, usize);
impl_grade_op!(Sub, sub, usize);
impl_grade_op!(Div, div, usize);
