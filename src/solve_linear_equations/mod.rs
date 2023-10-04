use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::row_arithmetic;

#[derive(Debug, Serialize, Deserialize)]
pub struct LinearSystem
{
    pub equations: Vec<row_arithmetic::Row>,
}

pub enum RowColumnResult
{
    Found((usize, usize)),
    NotFound
}

impl LinearSystem
{
    pub fn new(equations: Vec<row_arithmetic::Row>) -> Self
    {
        LinearSystem {equations}
    }

    // Assume here that the number of equations is lesser than or equal to the number of variables
    pub fn solve_system_of_linear_equations(&mut self) -> Result<bool, String>
    {
        let mut solution_map: HashMap<(usize, usize), f32> = HashMap::new();
        let mut iteration_count: usize = 0;

        while solution_map.len() != self.equations.len() && iteration_count < self.equations[0].a_ij.len()
        {
            iteration_count += 1;
            match self.determine_column_and_row(&solution_map)
            {
                RowColumnResult::Found((row, column)) =>
                {
                    match self.reduce_rows(column, row)
                    {
                        Ok(_) => (),
                        Err(error) => return Err(error)
                    };

                    solution_map.insert((row, column), self.equations[row].b_i);
                },
                RowColumnResult::NotFound => return Err(format!("Linear set of equations cannot be turned into reduced row echelon form")),

            }
        }

        Ok(true)
    }

    pub fn determine_column_and_row(&self, solution_map: &HashMap<(usize, usize), f32>) -> RowColumnResult
    {
        for (row_index, row) in self.equations.iter().enumerate()
            .filter(|(index, _)| !solution_map.iter().any(|(key, _)| key.0 == *index))
        {
            match row.a_ij.iter().enumerate()
                .filter(|(index, value)| 
                    !solution_map.iter().any(|(key, _)| key.1 == *index) && **value != 0.0
                ).nth(0)
            {
                None => (),
                Some((column_index, _)) => return RowColumnResult::Found((row_index, column_index))
            }
        }

        RowColumnResult::NotFound
    }

    fn reduce_rows(&mut self, column: usize, row: usize) -> Result<bool, String>
    {
        match self.equations[row].reduce_row_till_column_one(column)
        {
            Ok(_) => (),
            Err(error) => return Err(error),
        };

        let equation_to_reduce_by = self.equations[row].clone();

        for equation in self.equations.iter_mut().enumerate()
            .filter(|(index, _)| *index != row).map(|(_, row)| row)
        {
            match equation.reduce_row(&equation_to_reduce_by, column)
            {
                Ok(_) => (),
                Err(error) => return Err(error)
            };
        }

        Ok(true)
    }
}