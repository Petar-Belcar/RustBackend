use std::{fmt::Display, thread::current};

use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Row
{
    pub a_ij: Vec<f32>,
    pub b_i: f32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LinearProgram
{
    pub tableau: Vec<Row>,
    pub costs: Vec<f32>,
    pub relative_costs: Row,
    pub solution: Vec<f32>
}

impl Row
{
    // this may not be a needed function
    pub fn new(json: &String) -> Self
    {
        serde_json::from_str(&json).expect("Unable to convert from json string to Row struct")
    }

    pub fn reduce_row(&mut self, minuend: Row, column: usize) -> Result<bool, String>
    {
        if self.a_ij.len() != minuend.a_ij.len()
        {
            return Err(format!("Rows cannot be reduced if they are not the same length: subtrahend length = {}, minuend length = {}", self.a_ij.len(), minuend.a_ij.len()));
        }

        if column >= self.a_ij.len()
        {
            return Err(format!("Column cannot be outside of row: row length = {}, column = {}", self.a_ij.len(), column));
        }

        let multiplier = self.determine_how_much_to_multiply_by(&minuend, column);

        let mut minuend_column = 0;
        for subtrahend_column in &mut self.a_ij
        {
            *subtrahend_column = *subtrahend_column + multiplier * minuend.a_ij[minuend_column as usize];
            minuend_column += 1;
        }

        Ok(true)
    }

    fn determine_how_much_to_multiply_by(&self, minuend: &Row, column: usize) -> f32
    {
        -(self.a_ij[column as usize] / minuend.a_ij[column as usize])
    }

    pub fn reduce_row_till_column_one(&mut self, column: u32) -> Result<bool, String>
    {
        if column as usize >= self.a_ij.len() - 1
        {
            return Err(format!("The column which is to be set to 1 cannot be the last column or outside of the length of the row: [column = {}, row = {}]", column, self.a_ij.len()));
        }

        let multiplier = 1.0 / &self.a_ij[column as usize];

        for number in &mut self.a_ij
        {
            *number = *number * multiplier;
        }

        self.b_i = self.b_i * multiplier;

        Ok(true)
    }
}

impl Display for Row
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Row: {:?}, Constant: {}", self.a_ij, self.b_i)
    }
}

impl LinearProgram
{
    // add logic here
    pub fn new(json: &String) -> Result<LinearProgram, String>
    {
        let mut linear_program : LinearProgram = match serde_json::from_str(&json)
        {
            Ok(program) => program,
            Err(error) => return Err(format!("Error while parsing json string as object: {}", error))
        };

        if !linear_program.check_row_length()?
        {
            return Err(format!("Json passed did not have the same length of rows"));
        }

        if !linear_program.check_if_rows_is_equal_or_greater_than_columns()
        {
            return Err(format!("The passed linear program has more columns than rows"))
        }

        if !linear_program.check_if_matrix_starts_with_identity()
        {
            return Err(format!("The passed linear problem does not start with an identity"));
        }

        if !linear_program.check_if_solution_is_fesable()
        {
            return Err(format!("The passed solution is not feasible"));
        }

        if !linear_program.check_if_the_first_m_are_basic()
        {
            return Err(format!("The passed solution is not basic or non-degenerate"));
        }

        if !linear_program.check_if_b_and_solutions_are_same()
        {
            return Err(format!("Vector b and solution do not aligne"));
        }

        println!("{}", linear_program);

        Ok(linear_program)
    }

    fn check_row_length (&self) -> Result<bool, String>
    {
        let mut first_row_length: Option<usize> = None;
        let mut same_length = true;
        for length in self.tableau.iter()
                                                        .map(|row| row.a_ij.len())
        {
            match first_row_length
            {
                None => first_row_length = Some(length),
                Some(row_length) => 
                {
                    if row_length != length
                    {
                        same_length = false;
                    }
                }
            }
        };

        match first_row_length 
        {
            None => return Err(format!("First row does no have length")),
            Some(row_length) =>
            {
                if row_length != self.costs.len() || row_length != self.relative_costs.a_ij.len() || row_length != self.solution.len()
                {
                    same_length = false
                }
            }
        }

        Ok(same_length)
    }

    fn check_if_rows_is_equal_or_greater_than_columns(&self) -> bool
    {
        self.tableau.len() <= self.tableau[0].a_ij.len()
    }

    fn check_if_matrix_starts_with_identity(&self) -> bool
    {
        let mut starts_with_identity = true;

        let mut current_row: usize = 0;
        let max_row = self.tableau.len();

        for row in &self.tableau
        {
            let mut current_column: usize = 0;
            for column in &row.a_ij
            {
                if current_column < max_row
                {
                    if current_column == current_row && *column != 1.0
                    {
                        starts_with_identity = false;
                    }
                    if current_column != current_row && *column != 0.0
                    {
                        starts_with_identity = false;
                    }
                }
                current_column += 1;
            }
            current_row += 1;
        }

        starts_with_identity
    }

    fn check_if_solution_is_fesable(&self) -> bool
    {
        self.solution.iter().filter(|x| **x < 0.0).count() == 0
    }
    
    fn check_if_the_first_m_are_basic(&self) -> bool
    {
        self.solution.iter().take(self.tableau.len()).filter(|x| **x > 0.0).count() == self.tableau.len() && 
        self.solution.iter().skip(self.tableau.len()).filter(|x| **x > 0.0).count() == 0
    }

    fn check_if_b_and_solutions_are_same(&self) -> bool
    {
        self.solution.iter().take(self.tableau.len()).filter(|x| **x > 0.0)
            .zip(self.tableau.iter().map(|x| x.b_i))
            .filter(|(x, y)| *x == y).count() == self.tableau.len()
    }

    fn calculate_costs(&self) -> Result<String, String>
    {
        
        todo!();
    }
}

impl Display for LinearProgram
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tableau {:?}, Costs: {:?}, Relative costs: {}, Solution: {:?}", self.tableau, self.costs, self.relative_costs, self.solution)
    }
}