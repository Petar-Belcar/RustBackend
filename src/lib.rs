use std::{fs::File, io::Read};
mod row_arithmetic;

pub fn run() -> Result<String, String>
{
    let path_to_row_file = String::from("example_linearProgram.json");
    let mut example_row = match File::open(&path_to_row_file)
    {
        Ok(file) => file,
        Err(error_text) => return Err(format!("{}", error_text))
    };

    let mut json_data = String::new();
    match example_row.read_to_string(&mut json_data)
    {
        Ok(_) => println!("File on location {} read and stored\n{}", path_to_row_file, json_data),
        Err(_) => println!("File on location {} was not able to be read", path_to_row_file)
    }

    let mut linear_program = row_arithmetic::LinearProgram::new(&json_data)?;
    println!("{}", linear_program);

    let cloned_row : row_arithmetic::Row = linear_program.tableau[0].clone();
    match linear_program.tableau[2].reduce_row(cloned_row, 4)
    {
        Ok(_) => println!("Row successfully reduced"),
        Err(error_message) => return Err(error_message),
    };

    let divider_column = 5;
    match linear_program.find_lexicographically_lowest_row(divider_column)
    {
        Ok(row) => println!("Lexicologicly lowest row with divider row {} is: {}", divider_column, row),
        Err(error) => println!("{}", error)
    };

    Ok(format!("{}", linear_program))
}