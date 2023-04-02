#[cfg(test)]
mod tests {
    use crate::negative_clean;
    use crate::split_string;
    use crate::Calculator;

    #[test]
    fn calculation_plus() {
        let mut calculator = Calculator::new();
        test_calculation(10.0, "5 + 5", &mut calculator);
        test_calculation(10.0, "5+5", &mut calculator);
        test_calculation(29.0, "5+5+19", &mut calculator);
    }
    #[test]
    fn calculation_minus() {
        let mut calculator = Calculator::new();
        test_calculation(0.0, "5 - 5", &mut calculator);
        test_calculation(0.0, "5-5", &mut calculator);
        test_calculation(-5.0, "5 - 10", &mut calculator);
        test_calculation(5.0, "-5 - -10", &mut calculator);
    }
    #[test]
    fn calculation_chain() {
        let mut calculator = Calculator::new();
        test_calculation(20.0, "(5 + 5)*2", &mut calculator);
        test_calculation(15.0, "5 + 5 * 2", &mut calculator);
    }
    #[test]
    fn calculation_negative_cleaned() {
        let mut calculator = Calculator::new();
        test_calculation(-10.0, "-5 + -5", &mut calculator);
        test_calculation(10.0, "----5 + 5", &mut calculator);
    }
    #[test]
    fn calculation_specials() {
        let mut calculator = Calculator::new();
        test_calculation(1.0, "log_5(5)", &mut calculator);
    }

    // function for the actual test
    fn test_calculation(expected_result: f64, test_string: &str, calc_ref: &mut Calculator) {
        calc_ref.entry = String::from(test_string);

        calc_ref.tokens = split_string(&calc_ref.entry);
        negative_clean(&mut calc_ref.tokens);
        calc_ref.parse_tokens();
        calc_ref.next();
        let maybe_result = calc_ref.handle_expression();
        if maybe_result.is_err() {
            return;
        }
        assert_eq!(expected_result, maybe_result.ok().unwrap());
    }
}
