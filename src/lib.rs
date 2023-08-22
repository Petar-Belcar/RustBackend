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
        Ok(_) => (),
        Err(_) => println!("File on location {} was not able to be read", path_to_row_file)
    }

    let mut linear_program = row_arithmetic::LinearProgram::new(&json_data)?;
    println!("{}", linear_program);

    match linear_program.preform_simplex()
    {
        Ok(result) => println!("{}", result),
        Err(error) => return Err(error)
    };

    linear_program.set_solution()?;

    Ok(format!("{}", linear_program.to_json()?))
}