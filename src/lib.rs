use std::{fs::File, io::Read};
mod row_arithmetic;

pub fn run() -> Result<String, String>
{
    let path_to_row_file = String::from("example_row.json");
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

    let mut row = row_arithmetic::Row::new(&json_data);

    println!("{}", &row);

    row.reduce_row_till_column_one(1).expect("Failed to reduce row till the column is set to 1");

    Ok(format!("{:?}", row))
}