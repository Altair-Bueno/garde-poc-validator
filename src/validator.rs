pub trait Validator<T> {
    type Context;
    fn validate(&self, ctx: &Self::Context, value: &T) -> garde::Result;
}
