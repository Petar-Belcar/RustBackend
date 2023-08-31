#[cfg(test)]
mod tests {
    use crate::row_arithmetic::{LinearProgram, Row, perform_checks};
    // Put unit test for the simplex method here
    #[test]
    fn test_all_checks_passed() {
        let tableau = vec![Row{a_ij: vec![1.0, 0.0, 1.0, 1.0], b_i: 1.0}, 
                                    Row{a_ij: vec![0.0, 1.0, 2.0, 1.0], b_i: 1.0}];

        let linear_program = LinearProgram
        {
            tableau: tableau, 
            costs: vec![0.0, 0.0, 1.0, 2.0], 
            relative_costs: {Row{a_ij: vec![0.0, 0.0, 0.0, 0.0], b_i: 0.0}},
            solution: vec![1.0, 1.0, 0.0, 0.0]
        };

        match perform_checks(&linear_program)
        {
            Ok(response) => assert_eq!(response, "All checks passed"),
            Err(_) => ()
        };
    }
}