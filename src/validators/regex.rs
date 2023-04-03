use regex::Regex;

use crate::Validator;

impl<T> Validator<T> for Regex
where
    T: AsRef<str>,
{
    type Context = ();

    fn validate(&self, _: &Self::Context, value: &T) -> garde::Result {
        if self.is_match(value.as_ref()) {
            Ok(())
        } else {
            Err(garde::Error {
                message: "Fail to match regex".into(),
            })
        }
    }
}
