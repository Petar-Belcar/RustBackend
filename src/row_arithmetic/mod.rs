use std::fmt::Display;
// use serde::{Serialize, Deserialize};
use rocket::serde::{Deserialize, Serialize};
// use serde_json;


pub enum SimplexResult
{
    Unbound,
    Finished,
    IterationComplete,
    Error(String)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Row
{
    pub a_ij: Vec<f32>,
    pub b_i: f32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct LinearProgram
{
    pub tableau: Vec<Row>,
    pub costs: Vec<f32>,
    pub relative_costs: Row,
    pub solution: Vec<f32>
}

struct Position
{
    pub row: usize,
    pub column: usize,
}

impl Position
{
    pub fn new(new_row: usize, new_column: usize) -> Self
    {
        Position { row: new_row, column: new_column }
    }
}

#[allow(dead_code)]
impl Row
{
    // this may not be a needed function
    pub fn new(cost_changes: Vec<f32>, total_cost: f32) -> Self
    {
        Row{a_ij: cost_changes, b_i: -total_cost}
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

        if minuend.a_ij[column] == 0.0
        {
            return Err(format!("Cannot reduce row by column if the minuend has a 0 in it"));
        }

        let multiplier = self.determine_how_much_to_multiply_by(&minuend, column);

        let mut minuend_column: usize = 0;
        while minuend_column < self.a_ij.len()
        {
            if minuend_column != column 
            {
                self.a_ij[minuend_column] += multiplier * minuend.a_ij[minuend_column];
            }
            else 
            {
                self.a_ij[minuend_column] = 0.0;
            }
            minuend_column += 1;
        }

        self.b_i += multiplier * minuend.b_i;

        Ok(true)
    }

    fn determine_how_much_to_multiply_by(&self, minuend: &Row, column: usize) -> f32
    {
        -(self.a_ij[column] / minuend.a_ij[column])
    }

    pub fn reduce_row_till_column_one(&mut self, column: usize) -> Result<bool, String>
    {
        if column >= self.a_ij.len()
        {
            return Err(format!("The column which is to be set to 1 cannot be the last column or outside of the length of the row: [column = {}, row = {}]", column, self.a_ij.len()));
        }

        let multiplier = 1.0 / &self.a_ij[column];

        for number in &mut self.a_ij
        {
            *number = *number * multiplier;
        }

        self.a_ij[column] = 1.0;

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

#[allow(dead_code)]
impl LinearProgram
{
    // pub fn to_json(&self) -> Result<String, String>
    // {
    //     match serde_json::to_string(self)
    //     {
    //         Ok(json) => Ok(json),
    //         Err(error) => Err(format!("Error while serializing linear program: {}", error))
    //     }
    // }
    // // add logic here
    // pub fn new(json: &String) -> Result<LinearProgram, String>
    // {
    //     let mut linear_program : LinearProgram = match serde_json::from_str(&json)
    //     {
    //         Ok(program) => program,
    //         Err(error) => return Err(format!("Error while parsing json string as object: {}", error))
    //     };

    //     perform_checks(&linear_program)?;

    //     linear_program.relative_costs = linear_program.calculate_costs();

    //     Ok(linear_program)
    // }

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

    fn check_if_solution_is_feasible(&self) -> bool
    {
        self.solution.iter().filter(|x| **x < 0.0).count() == 0
    }
    
    fn check_if_the_first_m_are_basic(&self) -> bool
    {
        self.solution.iter().take(self.tableau.len()).count() == self.tableau.len() && 
        self.solution.iter().skip(self.tableau.len()).filter(|x| **x > 0.0).count() == 0
    }

    fn check_if_b_and_solutions_are_same(&self) -> bool
    {
        self.solution.iter().take(self.tableau.len())
            .zip(self.tableau.iter().map(|x| x.b_i))
            .filter(|(x, y)| *x == y).count() == self.tableau.len()
    }

    pub fn calculate_costs(&mut self) -> Row
    {
        Row::new(self.costs.iter().take(self.tableau.len()).map(|x| *x * 0.0)
            .chain(self.costs.iter().skip(self.tableau.len()).map(|x| -*x)).collect()
            , 0.0)      
    }

    pub fn find_lexicographically_lowest_row(&self, divider_column: usize) -> Result<usize, String>
    {
        let rows: Vec<Row> = self.tableau.iter().map(|x| x.clone()).collect();

        let mut current_lowest_row: usize = match find_first_row_with_positive_a(&rows, 0, divider_column)
        {
            Ok(number) => number,
            Err(_) => return Err(format!("There does not exist a row with positive elements in column {}", divider_column))
        };

        let mut current_comparing_row: usize = match find_first_row_with_positive_a(&rows, current_lowest_row + 1, divider_column)
        {
            Ok(number) => number,
            Err(_) => return Ok(current_lowest_row)
        };

        while current_comparing_row < self.tableau.len()
        {
            
            let lexicographic_comparison = compare_lexicographic_value_start(&self.tableau[current_lowest_row], &self.tableau[current_comparing_row], divider_column)?;

            if lexicographic_comparison
            {
                current_lowest_row = current_comparing_row;
            }
            current_comparing_row = 
            match find_first_row_with_positive_a(&rows, current_comparing_row + 1, divider_column)
            {
                Ok(number) => number,
                Err(_) => return Ok(current_lowest_row)
            };
        }

        Ok(current_lowest_row)
    }

    fn get_all_negative_cost_rows(&self) -> Vec<usize>
    {
        self.relative_costs.a_ij.iter().enumerate()
            .filter(|(_, y)| **y< 0.0)
            .map(|(x, _)| x).collect()
    }

    fn select_row_to_reduce_by(&self, negative_indices: &Vec<usize>) -> Result<(usize, usize), String>
    {
        for index in negative_indices
        {
            match self.find_lexicographically_lowest_row(*index)
            {
                Ok(row) => return Ok((row, *index)),
                Err(_) => ()
            };
        }

        Err(format!("No viable rows have been found"))
    }

    fn simplex_iteration(&mut self) -> SimplexResult
    {
        let negative_indices = self.get_all_negative_cost_rows();

        if negative_indices.len() == 0
        {
            return SimplexResult::Finished;
        }
        else 
        {
            let (reduction_row, column) = match self.select_row_to_reduce_by(&negative_indices)
            {
                Ok(row_result) => row_result,
                Err(_) => return SimplexResult::Unbound
            };

            match self.tableau[reduction_row].reduce_row_till_column_one(column)
            {
                Ok(_) => (),
                Err(error) => return SimplexResult::Error(error),
            };

            let cloned_reduction_row = self.tableau[reduction_row].clone();

            for row in self.tableau.iter_mut().enumerate()
                .filter(|(x, _)| *x != reduction_row).map(|(_, y)| y)
            {
                match row.reduce_row(&cloned_reduction_row, column)
                {
                    Ok(_) => (),
                    Err(error) => return SimplexResult::Error(error),
                };
            }

            match self.relative_costs.reduce_row(&cloned_reduction_row, column)
            {
                Ok(_) => (),
                Err(error) => return SimplexResult::Error(error),
            };

            SimplexResult::IterationComplete
        }
    }

    pub fn preform_simplex(&mut self) -> SimplexResult
    {
        loop 
        {
            match self.simplex_iteration()
            {
                SimplexResult::Finished => return SimplexResult::Finished,
                SimplexResult::Unbound => return SimplexResult::Unbound,
                SimplexResult::IterationComplete => (),
                SimplexResult::Error(error) => return SimplexResult::Error(error)
            } 
        }
    }

    pub fn set_solution(&mut self) -> Result<String, String>
    {
        for solution in &mut self.solution
        {
            *solution = 0.0;
        }

        for position in self.get_all_e_i()?
        {
            self.solution[position.column] = self.tableau[position.row].b_i;
        }
        
        Ok(format!("Solution set successfully"))
    }

    fn get_all_e_i(&self) -> Result<Vec<Position>, String>
    {
        let mut positions: Vec<Position> = Vec::new();

        for column_index in self.relative_costs.a_ij.iter()
            .enumerate().filter(|(_, x)| **x == 0.0).map(|(index, _)| index)
        {
            for (row_index, row) in self.tableau.iter().enumerate()
            {
                if row.a_ij[column_index] == 1.0
                {
                    if positions.iter().filter(|x| x.column == column_index).count() == 0
                    {
                        positions.push(Position::new(row_index, column_index));
                    }
                    else 
                    {
                        return Err(format!("A given column with relative cost 0 cannot have more than 1 element greater than 0")); 
                    }
                }
            }
        }

        Ok(positions)
    }

}

impl Display for LinearProgram
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tableau {:?}, Costs: {:?}, Relative costs: {}, Solution: {:?}", self.tableau, self.costs, self.relative_costs, self.solution)
    }
}

// TODO b needs to be checked first
fn compare_lexicographic_value_start(row_1: &Row, row_2: &Row, divider_column: usize) -> Result<bool, String>
{
    if row_1.b_i / row_1.a_ij[divider_column] == row_2.b_i / row_2.a_ij[divider_column]
    {
        compare_lexicographic_value(row_1, row_2, 0, divider_column)
    }
    else 
    {
        Ok(row_1.b_i / row_1.a_ij[divider_column] > row_2.b_i / row_2.a_ij[divider_column])
    }
}

fn compare_lexicographic_value(row_1: &Row, row_2: &Row, column: usize, divider_column: usize) -> Result<bool, String>
{
    if column >= row_1.a_ij.len()
    {
        Err(format!("Rows are linearly dependent"))
    }
    else 
    {
        if row_1.a_ij[column] / row_1.a_ij[divider_column] == row_2.a_ij[column] / row_2.a_ij[divider_column]
        {
            compare_lexicographic_value(row_1, row_2, column + 1, divider_column)
        }
        else 
        {
            Ok(row_1.a_ij[column] / row_1.a_ij[divider_column] > row_2.a_ij[column] / row_2.a_ij[divider_column])
        }
    }
}

fn find_first_row_with_positive_a(rows: &Vec<Row>, row: usize, divider_column: usize) -> Result<usize, String>
{
    let mut current_row: usize = row;
    while current_row < rows.len()
    {
        if !(rows[current_row].a_ij[divider_column] > 0.0)
        {
            current_row += 1;
        }
        else
        {
            return Ok(current_row);        
        }
    }
    
    Err(format!("The next column with positive number in column {} does not exist", row))
}

pub fn perform_checks(linear_program: &LinearProgram) -> Result<String, String>
{
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

    if !linear_program.check_if_solution_is_feasible()
    {
        return Err(format!("The passed solution is not feasible"));
    }

    if !linear_program.check_if_the_first_m_are_basic()
    {
        return Err(format!("The passed solution is not basic or non-degenerate"));
    }

    if !linear_program.check_if_b_and_solutions_are_same()
    {
        return Err(format!("Vector b and solution do not align"));
    }

    Ok(format!("All checks passed"))
}