#[cfg(test)]
mod tests {
    use crate::row_arithmetic;

    #[test]
    fn correct_input_test()
    {
        let mut subtrahend: [f32; 4] = [1.0, 0.0, 0.0, 0.0];
        let minuend: [f32; 4] = [1.0, 0.0, 0.0, 0.0];
        let column = 0;

        row_arithmetic::subtract_rows_setting_column_to_zero(&mut subtrahend, &minuend, column);

        let mut all_elements_zero = true;
        for element in subtrahend
        {
            if element != 0.0
            {
                all_elements_zero = false;
            }
        }

        assert!(all_elements_zero);
    }

    #[test]
    #[should_panic]
    fn column_too_large() // Set tests to expect to panic - although maybe we should panic somewhere else
    {
        let mut subtrahend: [f32; 4] = [1.0, 0.0, 0.0, 0.0];
        let minuend: [f32; 4] = [1.0, 0.0, 0.0, 0.0];
        let column = 4;

        row_arithmetic::subtract_rows_setting_column_to_zero(&mut subtrahend, &minuend, column);

        let mut all_elements_zero = true;
        for element in subtrahend
        {
            if element != 0.0
            {
                all_elements_zero = false;
            }
        }

        assert!(all_elements_zero);
    }

    #[test]
    #[should_panic]
    fn arrays_not_same_size()
    {
        let mut subtrahend: [f32; 4] = [1.0, 0.0, 0.0, 0.0];
        let minuend: [f32; 3] = [1.0, 0.0, 0.0];
        let column = 0;

        row_arithmetic::subtract_rows_setting_column_to_zero(&mut subtrahend, &minuend, column);

        let mut all_elements_zero = true;
        for element in subtrahend
        {
            if element != 0.0
            {
                all_elements_zero = false;
            }
        }

        assert!(all_elements_zero);
    }

}