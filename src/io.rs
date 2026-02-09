use std::io;

use crate::Logger;
use crate::errors::InputError;

/// Читает число из stdin в интерактивном цикле.
///
/// Повторяет запрос при некорректном вводе, выводя двуязычное сообщение об ошибке.
/// Все действия пользователя записываются в [`Logger`].
///
/// # Возвращает
///
/// * `Some(u128)` — корректное число
/// * `None` — пользователь ввёл `q` (выход) или произошла ошибка чтения stdin
pub fn read_input(logger: &mut Logger) -> Option<u128> {
    println!("Введите число для вычисления Фибоначчи:");

    loop {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(e) => {
                logger.log(&format!("Ошибка чтения stdin: {}", e));
                eprintln!("Ошибка чтения ввода: {}", e);
                return None;
            }
        }

        let trimmed = input.trim();

        if trimmed.eq_ignore_ascii_case("q") {
            return None;
        }

        logger.log(&format!("Ввод от пользователя: '{}'", trimmed));

        match parse_input(&input) {
            Ok(n) => return Some(n),
            Err(e) => {
                logger.log(&format!("Некорректный ввод: '{}'. Ошибка: {}", trimmed, e));
                println!("{}\nПопробуйте снова или введите 'q' для выхода:", e);
            }
        }
    }
}

/// Парсит строку пользовательского ввода в `u128`.
///
/// Выполняет последовательную валидацию: пустой ввод, множественные значения,
/// отрицательные числа, дробные числа, нечисловые символы, переполнение.
///
/// # Аргументы
///
/// * `input` — строка ввода (допускаются пробелы по краям)
///
/// # Примеры
///
/// ```
/// use fibonacci_stable::parse_input;
///
/// assert_eq!(parse_input("42"), Ok(42));
/// assert!(parse_input("abc").is_err());
/// assert!(parse_input("-5").is_err());
/// ```
pub fn parse_input(input: &str) -> Result<u128, InputError> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Err(InputError::EmptyInput);
    }

    if trimmed.contains(' ') {
        return Err(InputError::MultipleValues);
    }

    if trimmed.starts_with('-') {
        return Err(InputError::NegativeNumber);
    }

    if trimmed.contains('.') {
        return Err(InputError::FloatNumber);
    }

    if !trimmed.chars().all(|c| c.is_ascii_digit()) {
        return Err(InputError::InvalidCharacters);
    }

    trimmed
        .parse::<u128>()
        .map_err(|_| InputError::NumberTooLarge)
}

/// Выводит результат вычисления в формате `fibonacci(n) = result`.
pub fn print_result(n: u128, result: u128) {
    println!("fibonacci({}) = {}", n, result);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::InputError;

    #[test]
    fn given_valid_number_when_parse_input_then_returns_ok() {
        // Given
        let input = "10";

        // When
        let result = parse_input(input);

        // Then
        assert_eq!(result, Ok(10));
    }

    #[test]
    fn given_letters_when_parse_input_then_returns_invalid_characters() {
        // Given
        let input = "abc";

        // When
        let result = parse_input(input);

        // Then
        assert_eq!(result, Err(InputError::InvalidCharacters));
    }

    #[test]
    fn given_negative_number_when_parse_input_then_returns_negative_number() {
        // Given
        let input = "-5";

        // When
        let result = parse_input(input);

        // Then
        assert_eq!(result, Err(InputError::NegativeNumber));
    }

    #[test]
    fn given_float_when_parse_input_then_returns_float_number() {
        // Given
        let input = "3.14";

        // When
        let result = parse_input(input);

        // Then
        assert_eq!(result, Err(InputError::FloatNumber));
    }

    #[test]
    fn given_empty_string_when_parse_input_then_returns_empty_input() {
        // Given
        let input = "";

        // When
        let result = parse_input(input);

        // Then
        assert_eq!(result, Err(InputError::EmptyInput));
    }

    #[test]
    fn given_multiple_numbers_when_parse_input_then_returns_multiple_values() {
        // Given
        let input = "10 20";

        // When
        let result = parse_input(input);

        // Then
        assert_eq!(result, Err(InputError::MultipleValues));
    }

    #[test]
    fn given_huge_number_when_parse_input_then_returns_number_too_large() {
        // Given
        let input = "999999999999999999999999999999999999999999";

        // When
        let result = parse_input(input);

        // Then
        assert_eq!(result, Err(InputError::NumberTooLarge));
    }

    #[test]
    fn given_mixed_letters_and_digits_when_parse_input_then_returns_invalid_characters() {
        // Given
        let input = "12abc";

        // When
        let result = parse_input(input);

        // Then
        assert_eq!(result, Err(InputError::InvalidCharacters));
    }

    #[test]
    fn given_special_chars_when_parse_input_then_returns_invalid_characters() {
        // Given
        let input = "@#$%";

        // When
        let result = parse_input(input);

        // Then
        assert_eq!(result, Err(InputError::InvalidCharacters));
    }

    #[test]
    fn given_whitespace_around_number_when_parse_input_then_returns_ok() {
        // Given
        let input = "  42  ";

        // When
        let result = parse_input(input);

        // Then
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn given_zero_when_parse_input_then_returns_ok() {
        // Given
        let input = "0";

        // When
        let result = parse_input(input);

        // Then
        assert_eq!(result, Ok(0));
    }

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn given_any_u128_when_parse_its_string_then_returns_same_value(n: u128) {
            // Given
            let input = n.to_string();

            // When
            let result = parse_input(&input);

            // Then
            prop_assert_eq!(result, Ok(n));
        }
    }
}
