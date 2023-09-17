use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::row_arithmetic;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LinearSystem
{
    pub equations: Vec<row_arithmetic::Row>,
    pub solution: Option<HashMap<(usize, usize), f32>> // The first is row, second is column
}

pub enum RowColumnResult
{
    Found((usize, usize)),
    NoSolution,
    NotFound
}

impl LinearSystem
{
    pub fn new(equations: Vec<row_arithmetic::Row>) -> Self
    {
        LinearSystem {equations, solution: Some(HashMap::new())}
    }

    pub fn solve_system_of_linear_equations(&self) -> Vec<f32>
    {
        todo!();
    }

    pub fn determine_column_and_row(&self) -> RowColumnResult
    {
        let solution_set = match &self.solution
        {
            Some(set) => set,
            None => return RowColumnResult::NoSolution
        };


        for (row_index, row) in self.equations.iter().enumerate()
            .filter(|(index, _)| !solution_set.iter().any(|(key, _)| key.0 == *index))
        {
            match row.a_ij.iter().enumerate()
                .filter(|(index, value)| 
                    !solution_set.iter().any(|(key, _)| key.1 == *index) && **value > 0.0
                ).nth(0)
            {
                None => (),
                Some((column_index, _)) => return RowColumnResult::Found((row_index, column_index))
            }
        }

        RowColumnResult::NotFound
    }
}