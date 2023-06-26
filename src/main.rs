mod row_arithmetic;

fn main() {
    let mut subtrahend = [5.0, 3.5, 9.0, 4.0];
    let minuend = [1.0, 1.0, 18.0, 2.0];

    row_arithmetic::subtract_rows_setting_column_to_zero(&mut subtrahend, &minuend, 2);

    println!("{:?}", subtrahend);
}
