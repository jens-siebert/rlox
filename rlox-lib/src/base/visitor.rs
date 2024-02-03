pub trait Visitor<I, R, E> {
    fn visit(&self, input: &I) -> Result<R, E>;
}
