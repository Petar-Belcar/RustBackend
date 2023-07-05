mod row_arithmetic;
mod row_arithmetic_test;

fn main() {
    let mut subtrahend = [5.0, 3.5, 9.0, 4.0];
    let minuend = [1.0, 1.0, 18.0, 2.0];

    match row_arithmetic::subtract_rows_setting_column_to_zero(&mut subtrahend, &minuend, 2)
    {
        Ok(_) => println!("{:?}", subtrahend),
        Err(error_text) => println!("{}", error_text)
    };

    match row_arithmetic::reduce_row_till_column_zero(&mut subtrahend, 0)
    {
        Ok(_) => println!("{:?}", subtrahend),
        Err(error_text) => println!("{}", error_text),
    };
}
