use crate::errors::ComputationError;

/// Вычисляет число Фибоначчи для заданного индекса `x`.
///
/// Использует итеративный алгоритм на основе [`Iterator::try_fold`] с [`u128::checked_add`],
/// что гарантирует отсутствие паник при переполнении.
///
/// # Аргументы
///
/// * `x` — индекс числа Фибоначчи (0-based: fib(0)=0, fib(1)=1, fib(2)=1, ...)
///
/// # Возвращает
///
/// * `Ok(u128)` — число Фибоначчи, если оно помещается в `u128` (до fib(186) включительно)
/// * `Err(ComputationError::Overflow(x))` — если результат превышает `u128::MAX`
///
/// # Примеры
///
/// ```
/// use fibonacci_stable::fibonacci;
///
/// assert_eq!(fibonacci(0).unwrap(), 0);
/// assert_eq!(fibonacci(10).unwrap(), 55);
/// assert!(fibonacci(187).is_err());
/// ```
pub fn fibonacci(x: u128) -> Result<u128, ComputationError> {
    if x == 0 {
        return Ok(0);
    }
    (0..x - 1)
        .try_fold((0u128, 1u128), |(a, b), _| {
            b.checked_add(a).map(|sum| (b, sum))
        })
        .map(|(_, b)| b)
        .ok_or(ComputationError::Overflow(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_0_when_fibonacci_then_returns_0() {
        // Given
        let input = 0;

        // When
        let result = fibonacci(input).unwrap();

        // Then
        assert_eq!(result, 0);
    }

    #[test]
    fn given_1_when_fibonacci_then_returns_1() {
        // Given
        let input = 1;

        // When
        let result = fibonacci(input).unwrap();

        // Then
        assert_eq!(result, 1);
    }

    #[test]
    fn given_2_when_fibonacci_then_returns_1() {
        // Given
        let input = 2;

        // When
        let result = fibonacci(input).unwrap();

        // Then
        assert_eq!(result, 1);
    }

    #[test]
    fn given_3_when_fibonacci_then_returns_2() {
        // Given
        let input = 3;

        // When
        let result = fibonacci(input).unwrap();

        // Then
        assert_eq!(result, 2);
    }

    #[test]
    fn given_100_when_fibonacci_then_returns_354224848179261915075() {
        // Given
        let input = 100;

        // When
        let result = fibonacci(input).unwrap();

        // Then
        assert_eq!(result, 354224848179261915075);
    }

    #[test]
    fn given_consecutive_inputs_when_fibonacci_then_each_is_sum_of_two_previous() {
        // Given
        let n = 30;

        // When
        let results: Vec<u128> = (0..=n).map(|i| fibonacci(i).unwrap()).collect();

        // Then
        for i in 2..=n as usize {
            assert_eq!(
                results[i],
                results[i - 1] + results[i - 2],
                "fibonacci({}) должен быть равен fibonacci({}) + fibonacci({})",
                i,
                i - 1,
                i - 2
            );
        }
    }

    #[test]
    fn given_n_when_cassini_identity_then_holds() {
        // Given: тождество Кассини: fib(n-1) * fib(n+1) - fib(n)² = (-1)^n
        // ⚠ Не увеличивать диапазон! При n > 93 произведение fib(n)² переполняет i128,
        // а `as i128` молча обрежет значение — тест будет проходить с неверными данными.
        for n in 1u128..=30 {
            // When
            let fib_prev = fibonacci(n - 1).unwrap() as i128;
            let fib_curr = fibonacci(n).unwrap() as i128;
            let fib_next = fibonacci(n + 1).unwrap() as i128;

            let result = fib_prev * fib_next - fib_curr * fib_curr;
            let expected = if n % 2 == 0 { 1 } else { -1 };

            // Then
            assert_eq!(
                result, expected,
                "Тождество Кассини не выполняется для n={}",
                n
            );
        }
    }

    #[test]
    fn given_186_when_fibonacci_then_returns_ok() {
        // Given: fib(186) — последнее число Фибоначчи, помещающееся в u128
        let input = 186;

        // When
        let result = fibonacci(input);

        // Then: граничное значение — должен быть Ok
        assert!(
            result.is_ok(),
            "fibonacci(186) должен вернуть Ok, т.к. fib(186) помещается в u128"
        );
    }

    #[test]
    fn given_large_input_when_fibonacci_then_overflows() {
        // Given: значение, при котором u128 переполнится
        let input = 187;

        // When
        let result = fibonacci(input);

        // Then: должен быть Err, а не паника
        assert!(result.is_err());
    }

    // Тест перенесён из safe_fibonacci.rs
    #[test]
    fn given_large_input_when_safe_fibonacci_then_returns_error() {
        // Given
        let input = 187;

        // When
        let result = fibonacci(input);

        // Then
        assert!(result.is_err());
    }

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn fib_sum_property(n in 2u128..80) {
            // Given
            let fib_n = fibonacci(n).unwrap();
            let fib_prev = fibonacci(n - 1).unwrap();
            let fib_prev2 = fibonacci(n - 2).unwrap();

            // When
            let sum = fib_prev + fib_prev2;

            // Then
            prop_assert_eq!(fib_n, sum);
        }

        #[test]
        fn fib_cassini_property(n in 1u128..30) {
            // ⚠ Не увеличивать диапазон! При n > 93 произведение fib(n)² переполняет i128,
            // а `as i128` молча обрежет значение — тест будет проходить с неверными данными.
            // Given
            let prev = fibonacci(n - 1).unwrap() as i128;
            let curr = fibonacci(n).unwrap() as i128;
            let next = fibonacci(n + 1).unwrap() as i128;

            // When
            let result = prev * next - curr * curr;
            let expected = if n % 2 == 0 { 1i128 } else { -1i128 };

            // Then
            prop_assert_eq!(result, expected);
        }
    }
}
