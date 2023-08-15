use std::fmt::Display;

use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Row
{
    pub a_ij: Vec<f32>,
    pub b_i: f32
}

pub struct LinearProgram
{
    pub tableau: Vec<Row>,
    pub costs: Row
}

impl Row
{
    pub fn new(json: &String) -> Self
    {
        serde_json::from_str(&json).expect("Unable to convert from json string to struct")
    }

    pub fn reduce_row(&mut self, minuend: &Row, column: usize) -> Result<bool, String>
    {
        if self.a_ij.len() != minuend.a_ij.len()
        {
            return Err(format!("Rows cannot be reduced if they are not the same length: subtrahend length = {}, minuend length = {}", self.a_ij.len(), minuend.a_ij.len()));
        }

        if column >= self.a_ij.len()
        {
            return Err(format!("Column cannot be outside of row: row length = {}, column = {}", self.a_ij.len(), column));
        }

        let multiplier = self.determine_how_much_to_multiply_by(minuend, column);

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
    pub fn new(json: String) -> Self
    {
        todo!();
    }
}