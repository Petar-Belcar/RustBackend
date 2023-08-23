mod row_arithmetic;

pub fn run(json_string: &String) -> Result<String, String>
{
    let mut linear_program = row_arithmetic::LinearProgram::new(json_string)?;

    match linear_program.preform_simplex()
    {
        Ok(_) => (),
        Err(error) => return Err(error)
    };

    linear_program.set_solution()?;

    Ok(format!("{}", linear_program.to_json()?))
}