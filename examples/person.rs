use garde::Validate;
use garde_poc_validator::Validator;
use lazy_regex::lazy_regex;
use lazy_regex::Lazy;
use regex::Regex;

// Regex validation at compile time, but any other regex would work fine
pub static USERNAME_REGEX: Lazy<Regex> = lazy_regex!("^ab+$"i);

#[derive(Debug)]
pub struct Person {
    // #[garde(USERNAME_REGEX)]
    name: String,
    // #[garde(18..)]
    age: usize,
}

// !!! Example code generated by the macro
impl Validate for Person {
    type Context = ();

    fn validate(&self, ctx: &Self::Context) -> Result<(), garde::Errors> {
        let mut v = vec![];
        if let Err(error) = (USERNAME_REGEX).validate(ctx, &self.name) {
            v.push(error);
        }
        if let Err(error) = (18..).validate(ctx, &self.age) {
            v.push(error);
        }
        if v.is_empty() {
            Ok(())
        } else {
            Err(garde::Errors::Simple(v))
        }
    }
}
// !!! END Example code generated by the macro

fn main() {
    let person = Person {
        name: "Marty".into(),
        age: 10,
    };
    person.validate(&()).expect("Person validation failed")
}
