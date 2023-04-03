use std::ops::RangeFrom;
use std::ops::RangeInclusive;
use std::ops::RangeToInclusive;

use crate::Validator;

impl<Value> Validator<Value> for RangeInclusive<Value>
where
    Value: PartialOrd,
{
    type Context = ();

    fn validate(&self, _: &Self::Context, value: &Value) -> garde::Result {
        if self.contains(value) {
            Ok(())
        } else {
            Err(garde::Error {
                message: "Out of bounds".into(),
            })
        }
    }
}

impl<Value> Validator<Value> for RangeToInclusive<Value>
where
    Value: PartialOrd,
{
    type Context = ();

    fn validate(&self, _: &Self::Context, value: &Value) -> garde::Result {
        if self.contains(value) {
            Ok(())
        } else {
            Err(garde::Error {
                message: "Out of bounds".into(),
            })
        }
    }
}

impl<Value> Validator<Value> for RangeFrom<Value>
where
    Value: PartialOrd,
{
    type Context = ();

    fn validate(&self, _: &Self::Context, value: &Value) -> garde::Result {
        if self.contains(value) {
            Ok(())
        } else {
            Err(garde::Error {
                message: "Out of bounds".into(),
            })
        }
    }
}
