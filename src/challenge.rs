pub trait Challenge {
    type Input;
    type Output;

    fn name(&self) -> &'static str;

    fn parse(&self, input: &[u8]) -> Self::Input;

    fn solve_part_1(&self, input: &Self::Input) -> Self::Output;

    fn solve_part_2(&self, input: &Self::Input) -> Self::Output;
}
