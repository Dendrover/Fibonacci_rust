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
pub mod logger;
pub mod io;

pub use errors::{InputError, ComputationError};
pub use fibonacci::fibonacci;
pub use logger::Logger;
pub use io::{read_input, parse_input, print_result};
