use crate::Validator;

pub struct Contains<S>(pub S);

impl<T: AsRef<str>, S: AsRef<str>> Validator<T> for Contains<S> {
    type Context = ();

    fn validate(&self, _: &Self::Context, value: &T) -> garde::Result {
        if value.as_ref().contains(self.0.as_ref()) {
            Ok(())
        } else {
            Err(garde::Error {
                message: "Substring missing".into(),
            })
        }
    }
}
