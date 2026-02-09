use fibonacci_stable::{Logger, fibonacci, parse_input};

#[test]
fn given_valid_input_when_full_workflow_then_logger_tracks_everything() {
    // Given
    let mut logger = Logger::new();
    let input = "10";

    // When
    let n = parse_input(input).unwrap();
    logger.log(&format!("Ввод: {}", n));
    let result = fibonacci(n).unwrap();
    logger.log(&format!("Результат: {}", result));

    // Then
    assert_eq!(n, 10);
    assert_eq!(result, 55);
    assert_eq!(logger.entries().len(), 2);
    assert!(logger.entries()[0].contains("Ввод: 10"));
    assert!(logger.entries()[1].contains("Результат: 55"));
}

#[test]
fn given_invalid_then_valid_input_when_parse_then_second_succeeds() {
    // Given
    let inputs = vec!["abc", "-5", "3.14", "7"];

    // When / Then
    let last_result = inputs.iter().find_map(|input| match parse_input(input) {
        Ok(n) => Some(n),
        Err(_) => None,
    });

    assert_eq!(last_result, Some(7));
    assert_eq!(fibonacci(last_result.unwrap()).unwrap(), 13);
}

#[test]
fn given_zero_input_when_full_workflow_then_returns_zero() {
    // Given
    let mut logger = Logger::new();

    // When
    let n = parse_input("0").unwrap();
    logger.log(&format!("Ввод: {}", n));
    let result = fibonacci(n).unwrap();
    logger.log(&format!("Результат: {}", result));

    // Then
    assert_eq!(result, 0);
    assert!(logger.entries()[1].contains("Результат: 0"));
}

use proptest::prelude::*;

proptest! {
    #[test]
    fn given_any_string_when_parse_input_then_never_panics(s in "\\PC{0,200}") {
        // Given / When
        let result = parse_input(&s);

        // Then — не паникует, всегда Ok или Err
        prop_assert!(result.is_ok() || result.is_err());
    }
}
