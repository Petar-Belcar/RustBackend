#[cfg(test)]
mod tests {
    use crate::row_arithmetic::{LinearProgram, Row, perform_checks, self};
    use crate::LinearProgramResponse;

    fn simplex_procedure(linear_program: &mut row_arithmetic::LinearProgram) -> LinearProgramResponse
    {
        match perform_checks(&linear_program)
        {
            Ok(_) => (),
            Err(error) => return LinearProgramResponse::Error(error)
        };

        linear_program.relative_costs = linear_program.calculate_costs();

        match linear_program.preform_simplex()
        {
            row_arithmetic::SimplexResult::Finished => (),
            row_arithmetic::SimplexResult::Unbound => return LinearProgramResponse::Unbound(format!("Problem is unbound and the optimal solution is infinity")),
            row_arithmetic::SimplexResult::IterationComplete => return LinearProgramResponse::Error(format!("Iteration complete, you should never get this though")),
            row_arithmetic::SimplexResult::Error(error) => return LinearProgramResponse::Error(error)
        };

        match linear_program.set_solution()
        {
            Ok(_) => (),
            Err(error) => return LinearProgramResponse::Error(error)
        }

        let response_row = row_arithmetic::Row{a_ij: linear_program.solution.clone(), b_i: linear_program.relative_costs.b_i};

        LinearProgramResponse::LinearProgram(response_row)
    }

    // Put unit test for the simplex method here
    #[test]
    fn test_all_checks_passed_has_optimal_solution() 
    {
        let tableau = vec![Row{a_ij: vec![1.0, 0.0, 1.0, 1.0], b_i: 1.0}, 
                                    Row{a_ij: vec![0.0, 1.0, 2.0, 1.0], b_i: 1.0}];

        let mut linear_program = LinearProgram
        {
            tableau: tableau, 
            costs: vec![0.0, 0.0, 1.0, 2.0], 
            relative_costs: {Row{a_ij: vec![0.0, 0.0, 0.0, 0.0], b_i: 0.0}},
            solution: vec![1.0, 1.0, 0.0, 0.0]
        };

        match simplex_procedure(&mut linear_program)
        {
            LinearProgramResponse::LinearProgram(row) => assert_eq!(row.b_i, 2.0),
            LinearProgramResponse::Unbound(_) => assert!(false),
            LinearProgramResponse::Error(_) => assert!(false)
        };
    }

    #[test]
    fn test_all_checks_passed_unbound()
    {
        let tableau = vec![Row{a_ij: vec![1.0, 0.0, 0.0, 1.0], b_i: 1.0}, 
                                    Row{a_ij: vec![0.0, 1.0, 0.0, 1.0], b_i: 1.0}];

        let mut linear_program = LinearProgram
        {
            tableau: tableau, 
            costs: vec![0.0, 0.0, 1.0, 2.0], 
            relative_costs: {Row{a_ij: vec![0.0, 0.0, 0.0, 0.0], b_i: 0.0}},
            solution: vec![1.0, 1.0, 0.0, 0.0]
        };

        match simplex_procedure(&mut linear_program)
        {
            LinearProgramResponse::LinearProgram(_) => assert!(false),
            LinearProgramResponse::Unbound(_) => assert!(true),
            LinearProgramResponse::Error(_) => assert!(false)
        };
    }

    #[test]
    fn test_mismatched_row_length()
    {
        let tableau = vec![Row{a_ij: vec![1.0, 0.0, 0.0, 1.0], b_i: 1.0}, 
                                    Row{a_ij: vec![0.0, 1.0, 0.0], b_i: 1.0}];

        let mut linear_program = LinearProgram
        {
            tableau: tableau, 
            costs: vec![0.0, 0.0, 1.0, 2.0], 
            relative_costs: {Row{a_ij: vec![0.0, 0.0, 0.0, 0.0], b_i: 0.0}},
            solution: vec![1.0, 1.0, 0.0, 0.0]
        };

        match simplex_procedure(&mut linear_program)
        {
            LinearProgramResponse::LinearProgram(_) => assert!(false),
            LinearProgramResponse::Unbound(_) => assert!(false),
            LinearProgramResponse::Error(message) => assert_eq!("Json passed did not have the same length of rows", message)
        };
    }

    #[test]
    fn test_number_of_rows_greater_than_columns()
    {
        let tableau = vec![Row{a_ij: vec![1.0], b_i: 1.0}, 
                                    Row{a_ij: vec![0.0], b_i: 1.0}];

        let mut linear_program = LinearProgram
        {
            tableau: tableau, 
            costs: vec![0.0], 
            relative_costs: {Row{a_ij: vec![0.0], b_i: 0.0}},
            solution: vec![1.0]
        };

        match simplex_procedure(&mut linear_program)
        {
            LinearProgramResponse::LinearProgram(_) => assert!(false),
            LinearProgramResponse::Unbound(_) => assert!(false),
            LinearProgramResponse::Error(message) => assert_eq!("The passed linear program has more columns than rows", message)
        };
    }

    #[test]
    fn test_check_if_matrix_starts_with_identity()
    {
        let tableau = vec![Row{a_ij: vec![1.1, 0.0, 1.0, 1.0], b_i: 1.0}, 
                                    Row{a_ij: vec![0.0, 1.0, 2.0, 1.0], b_i: 1.0}];

        let mut linear_program = LinearProgram
        {
            tableau: tableau, 
            costs: vec![0.0, 0.0, 1.0, 2.0], 
            relative_costs: {Row{a_ij: vec![0.0, 0.0, 0.0, 0.0], b_i: 0.0}},
            solution: vec![1.0, 1.0, 0.0, 0.0]
        };

        match simplex_procedure(&mut linear_program)
        {
            LinearProgramResponse::LinearProgram(_) => assert!(false),
            LinearProgramResponse::Unbound(_) => assert!(false),
            LinearProgramResponse::Error(message) => assert_eq!("The passed linear problem does not start with an identity", message)
        };
    }

    #[test]
    fn test_check_if_solution_is_feasible()
    {
        let tableau = vec![Row{a_ij: vec![1.0, 0.0, 1.0, 1.0], b_i: 1.0}, 
                                    Row{a_ij: vec![0.0, 1.0, 2.0, 1.0], b_i: 1.0}];

        let mut linear_program = LinearProgram
        {
            tableau: tableau, 
            costs: vec![0.0, 0.0, 1.0, 2.0], 
            relative_costs: {Row{a_ij: vec![0.0, 0.0, 0.0, 0.0], b_i: 0.0}},
            solution: vec![1.0, 1.0, 0.0, -1.0]
        };

        match simplex_procedure(&mut linear_program)
        {
            LinearProgramResponse::LinearProgram(_) => assert!(false),
            LinearProgramResponse::Unbound(_) => assert!(false),
            LinearProgramResponse::Error(message) => assert_eq!("The passed solution is not feasible", message)
        };
    }

    #[test]
    fn test_check_if_the_first_m_are_basic()
    {
        let tableau = vec![Row{a_ij: vec![1.0, 0.0, 1.0, 1.0], b_i: 1.0}, 
                                    Row{a_ij: vec![0.0, 1.0, 2.0, 1.0], b_i: 1.0}];

        let mut linear_program = LinearProgram
        {
            tableau: tableau, 
            costs: vec![0.0, 0.0, 1.0, 2.0], 
            relative_costs: {Row{a_ij: vec![0.0, 0.0, 0.0, 0.0], b_i: 0.0}},
            solution: vec![1.0, 1.0, 0.0, 1.0]
        };

        match simplex_procedure(&mut linear_program)
        {
            LinearProgramResponse::LinearProgram(_) => assert!(false),
            LinearProgramResponse::Unbound(_) => assert!(false),
            LinearProgramResponse::Error(message) => assert_eq!("The passed solution is not basic or non-degenerate", message)
        };
    }

    #[test]
    fn test_check_if_b_and_solutions_are_same()
    {
        let tableau = vec![Row{a_ij: vec![1.0, 0.0, 1.0, 1.0], b_i: 1.0}, 
                                    Row{a_ij: vec![0.0, 1.0, 2.0, 1.0], b_i: 1.0}];

        let mut linear_program = LinearProgram
        {
            tableau: tableau, 
            costs: vec![0.0, 0.0, 1.0, 2.0], 
            relative_costs: {Row{a_ij: vec![0.0, 0.0, 0.0, 0.0], b_i: 0.0}},
            solution: vec![1.0, 2.0, 0.0, 0.0]
        };

        match simplex_procedure(&mut linear_program)
        {
            LinearProgramResponse::LinearProgram(_) => assert!(false),
            LinearProgramResponse::Unbound(_) => assert!(false),
            LinearProgramResponse::Error(message) => assert_eq!("Vector b and solution do not align", message)
        };
    }
}