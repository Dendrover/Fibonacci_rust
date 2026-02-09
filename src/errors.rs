use std::fmt;

/// Ошибка вычисления числа Фибоначчи.
///
/// Возникает, когда результат не помещается в `u128`.
#[derive(Debug, PartialEq)]
pub enum ComputationError {
    /// Переполнение при вычислении для заданного входного значения.
    Overflow(u128),
}

impl fmt::Display for ComputationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ComputationError::Overflow(x) = self;
        write!(f, "Переполнение при вычислении для входа {}", x)
    }
}

/// Ошибка валидации пользовательского ввода.
///
/// Каждый вариант выводится на двух языках (RU + EN) через [`fmt::Display`].
#[derive(Debug, PartialEq)]
pub enum InputError {
    /// Пользователь ничего не ввёл.
    EmptyInput,
    /// Введено отрицательное число.
    NegativeNumber,
    /// Введено дробное число.
    FloatNumber,
    /// Ввод содержит нечисловые символы.
    InvalidCharacters,
    /// Введено более одного значения.
    MultipleValues,
    /// Число не помещается в `u128`.
    NumberTooLarge,
    /// Переполнение при вычислении Фибоначчи для данного числа.
    ComputationOverflow(u128),
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (ru, en) = match self {
            InputError::EmptyInput => (
                "Пустой ввод".to_string(),
                "Empty input".to_string(),
            ),
            InputError::NegativeNumber => (
                "Отрицательные числа не поддерживаются".to_string(),
                "Negative numbers are not supported".to_string(),
            ),
            InputError::FloatNumber => (
                "Дробные числа не поддерживаются".to_string(),
                "Floating point numbers are not supported".to_string(),
            ),
            InputError::InvalidCharacters => (
                "Ввод содержит нечисловые символы".to_string(),
                "Input contains non-numeric characters".to_string(),
            ),
            InputError::MultipleValues => (
                "Введите только одно число".to_string(),
                "Enter only one number".to_string(),
            ),
            InputError::NumberTooLarge => (
                "Число слишком большое".to_string(),
                "Number is too large".to_string(),
            ),
            InputError::ComputationOverflow(x) => (
                format!("Переполнение при вычислении fibonacci({})", x),
                format!("Overflow when computing fibonacci({})", x),
            ),
        };
        write!(f, "RU: Ошибка! {}\nEN: Error! {}", ru, en)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_empty_input_when_display_then_shows_ru_and_en() {
        // Given
        let error = InputError::EmptyInput;

        // When
        let message = format!("{}", error);

        // Then
        assert!(message.contains("RU: Ошибка! Пустой ввод"));
        assert!(message.contains("EN: Error! Empty input"));
    }

    #[test]
    fn given_negative_number_when_display_then_shows_ru_and_en() {
        // Given
        let error = InputError::NegativeNumber;

        // When
        let message = format!("{}", error);

        // Then
        assert!(message.contains("RU: Ошибка! Отрицательные числа не поддерживаются"));
        assert!(message.contains("EN: Error! Negative numbers are not supported"));
    }

    #[test]
    fn given_float_number_when_display_then_shows_ru_and_en() {
        // Given
        let error = InputError::FloatNumber;

        // When
        let message = format!("{}", error);

        // Then
        assert!(message.contains("RU: Ошибка! Дробные числа не поддерживаются"));
        assert!(message.contains("EN: Error! Floating point numbers are not supported"));
    }

    #[test]
    fn given_invalid_characters_when_display_then_shows_ru_and_en() {
        // Given
        let error = InputError::InvalidCharacters;

        // When
        let message = format!("{}", error);

        // Then
        assert!(message.contains("RU: Ошибка! Ввод содержит нечисловые символы"));
        assert!(message.contains("EN: Error! Input contains non-numeric characters"));
    }

    #[test]
    fn given_multiple_values_when_display_then_shows_ru_and_en() {
        // Given
        let error = InputError::MultipleValues;

        // When
        let message = format!("{}", error);

        // Then
        assert!(message.contains("RU: Ошибка! Введите только одно число"));
        assert!(message.contains("EN: Error! Enter only one number"));
    }

    #[test]
    fn given_too_large_number_when_display_then_shows_ru_and_en() {
        // Given
        let error = InputError::NumberTooLarge;

        // When
        let message = format!("{}", error);

        // Then
        assert!(message.contains("RU: Ошибка! Число слишком большое"));
        assert!(message.contains("EN: Error! Number is too large"));
    }

    #[test]
    fn given_overflow_when_display_then_shows_ru_and_en() {
        // Given
        let error = InputError::ComputationOverflow(187);

        // When
        let message = format!("{}", error);

        // Then
        assert!(message.contains("RU: Ошибка! Переполнение при вычислении fibonacci(187)"));
        assert!(message.contains("EN: Error! Overflow when computing fibonacci(187)"));
    }

    #[test]
    fn given_computation_overflow_when_display_then_shows_message() {
        let err = ComputationError::Overflow(187);
        assert_eq!(
            format!("{}", err),
            "Переполнение при вычислении для входа 187"
        );
    }
}
