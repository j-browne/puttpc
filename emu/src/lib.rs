pub mod v1;
pub mod v2;
pub use v2::*;

pub trait Machine: IntoIterator {
    /// The type of a single unit of input
    type Input;

    /// The type of a single unit of output
    type Output;

    /// Set the full input of the machine
    fn set_input(&mut self, input: &[Self::Input]);

    /// Perform one step of the machine
    fn step(&mut self) -> Option<Self::Output>;

    /// Run the machine until some halting condition is met
    fn run(&mut self) -> Vec<Self::Output>;

    /// Run the machine until some halting condition is met, with the input provided
    fn run_with_input(&mut self, input: &[Self::Input]) -> Vec<Self::Output> {
        self.set_input(input);
        self.run()
    }

    /// Return an iterator of the output, with the input provided
    fn into_iter_with_input(mut self, input: &[Self::Input]) -> <Self as IntoIterator>::IntoIter
    where
        Self: Sized,
    {
        self.set_input(input);
        self.into_iter()
    }
}
