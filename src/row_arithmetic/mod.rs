pub fn subtract_rows_setting_column_to_zero<'a>(subtrahend: &'a mut [f32], minuend: &'a [f32], column: u32) -> Result<bool, String>
{
    if subtrahend.len() != minuend.len()
    {
        return Err(format!("Rows must be of the same length: [subtrahend = {}, minuend = {}]", subtrahend.len(), minuend.len()))
    }

    let subtrahend_len: u32 = subtrahend.len().try_into().unwrap();

    if column >= subtrahend_len - 1
    {
        return Err(format!("The column which is to be set to 0 cannot be the last column or outside of the length of the subtrahend: [column = {}, subtrahend = {}]", column, subtrahend_len))
    }

    let multiplier = determine_how_much_to_multiply_by(subtrahend, minuend, column);

    let mut minuend_column = 0;
    for subtrahend_column in subtrahend
    {
        *subtrahend_column = *subtrahend_column + multiplier * minuend[minuend_column as usize];
        minuend_column += 1;
    }

    Ok(true)
}

fn determine_how_much_to_multiply_by<'a>(subtrahend: &'a mut [f32], minuend: &'a [f32], column: u32) -> f32
{
    -(&subtrahend[column as usize] / &minuend[column as usize])
}