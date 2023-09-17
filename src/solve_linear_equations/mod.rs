use crate::row_arithmetic;

pub struct LinearSystem
{
    equations: Vec<row_arithmetic::Row>,
    solution: HashMap<(usize, usize), f32>
}

impl LinearSystem
{
    pub fn new(equations: Vec<row_arithmetic::Row>) -> Self
    {
        LinearSystem {equations: equations, solution: HashMap::new()}
    }

    pub fn solve_system_of_linear_equations(&self) -> Vec<f32>
    {
        todo!();
    }

    fn determine_column_and_row(&self) -> (usize, usize)
    {

        todo!();
    }
}