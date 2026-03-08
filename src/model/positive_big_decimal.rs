use crate::model::my_error::MyError;
use crate::model::my_result::MyResult;
use rust_decimal::Decimal;
use std::ops::Deref;

pub const AMOUNT_PRECISION: u32 = 4;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct PositiveScale4Decimal(Decimal);

impl PositiveScale4Decimal {
    pub fn new(value: Decimal) -> MyResult<Self> {
        if value.is_sign_negative() {
            Err(MyError::NegativeAmount(value))
        } else if value.scale() > AMOUNT_PRECISION {
            Err(MyError::InvalidAmountPrecision(value))
        } else if value.is_zero() {
            Err(MyError::ZeroAmount(value))
        } else {
            Ok(PositiveScale4Decimal(value))
        }
    }
}

impl Deref for PositiveScale4Decimal {
    type Target = Decimal;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
