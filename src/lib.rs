mod row_arithmetic;

pub fn run(linear_program: &mut row_arithmetic::LinearProgram) -> Result<&mut row_arithmetic::LinearProgram, String>
{
    linear_program.relative_costs = linear_program.calculate_costs();

    match linear_program.preform_simplex()
    {
        Ok(_) => (),
        Err(error) => return Err(error)
    };

    match linear_program.set_solution()
    {
        Ok(_) => (),
        Err(error) => return Err(error)
    }

    Ok(linear_program)
}