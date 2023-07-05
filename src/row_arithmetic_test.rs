#[cfg(test)]
mod tests {
    use crate::row_arithmetic;

    #[test]
    fn subtraction_correct_input_test()
    {
        let mut subtrahend: [f32; 4] = [1.0, 0.0, 0.0, 0.0];
        let minuend: [f32; 4] = [1.0, 0.0, 0.0, 0.0];
        let column = 0;

        let arithmetic_result = match row_arithmetic::subtract_rows_setting_column_to_zero(&mut subtrahend, &minuend, column)
        {
            Ok(_) => true,
            Err(_) => false
        };

        let mut all_elements_zero = true;
        for element in subtrahend
        {
            if element != 0.0
            {
                all_elements_zero = false;
            }
        }

        assert!(all_elements_zero && arithmetic_result);
    }

    #[test]
    fn subtraction_column_too_large()
    {
        let mut subtrahend: [f32; 4] = [1.0, 0.0, 0.0, 0.0];
        let minuend: [f32; 4] = [1.0, 0.0, 0.0, 0.0];
        let column = 4;

        let arithmetic_result = match row_arithmetic::subtract_rows_setting_column_to_zero(&mut subtrahend, &minuend, column)
        {
            Ok(_) => true,
            Err(_) => false
        };

        assert!(!arithmetic_result);
    }

    #[test]
    fn subtraction_arrays_not_same_size()
    {
        let mut subtrahend: [f32; 4] = [1.0, 0.0, 0.0, 0.0];
        let minuend: [f32; 3] = [1.0, 0.0, 0.0];
        let column = 0;

        let arithmetic_result = match row_arithmetic::subtract_rows_setting_column_to_zero(&mut subtrahend, &minuend, column)
        {
            Ok(_) => true,
            Err(_) => false
        };

        assert!(!arithmetic_result);
    }

    #[test]
    fn reduction_correct_input()
    {
        let mut row = [3.0, 2.0, 2.0, 1.0, 5.0];
        let column = 2;

        let arithmetic_result = match row_arithmetic::reduce_row_till_column_zero(&mut row, column)
        {
            Ok(_) => true,
            Err(_) => false,
        };

        assert!(arithmetic_result);
    }

    #[test]
    fn reduction_column_too_large()
    {
        let mut row = [3.0, 2.0, 2.0, 1.0, 5.0];
        let column = 4;

        let arithmetic_result = match row_arithmetic::reduce_row_till_column_zero(&mut row, column)
        {
            Ok(_) => true,
            Err(_) => false,
        };

        assert!(!arithmetic_result);
    }

}