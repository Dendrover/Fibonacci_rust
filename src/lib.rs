//! # fibonacci_stable
//!
//! Калькулятор чисел Фибоначчи на Rust с безопасной обработкой переполнения,
//! двуязычными сообщениями об ошибках (RU/EN), логированием и полным покрытием тестами.
//!
//! ## Основные возможности
//!
//! - Безопасное вычисление через [`fibonacci()`] — `try_fold` + `checked_add`, без паник
//! - Валидация ввода через [`parse_input()`] — обработка всех видов некорректного ввода
//! - Двуязычные ошибки ([`InputError`], [`ComputationError`]) — RU + EN
//! - Логирование через [`Logger`] с таймстемпами
//!
//! ## Пример
//!
//! ```
//! use fibonacci_stable::fibonacci;
//!
//! let result = fibonacci(10).unwrap();
//! assert_eq!(result, 55);
//! ```

pub mod errors;
pub mod fibonacci;
pub mod io;
pub mod logger;

pub use errors::{ComputationError, InputError};
pub use fibonacci::fibonacci;
pub use io::{parse_input, print_result, read_input};
pub use logger::Logger;
