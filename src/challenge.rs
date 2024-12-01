pub trait Challenge {
    type Input;
    type Output;

    fn name(&self) -> &'static str;

    fn parse(&self, input: &[u8]) -> Self::Input;

    fn solve(&self, input: &Self::Input) -> Self::Output;
}
