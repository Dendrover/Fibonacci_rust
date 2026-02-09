/// Простой логгер с таймстемпами.
///
/// Записывает сообщения в формате `[YYYY-MM-DD HH:MM:SS.mmm] сообщение`.
/// Используется для отслеживания действий пользователя и этапов вычисления.
///
/// # Пример
///
/// ```
/// use fibonacci_stable::Logger;
///
/// let mut logger = Logger::new();
/// logger.log("Программа запущена");
/// assert_eq!(logger.entries().len(), 1);
/// ```
pub struct Logger {
    entries: Vec<String>,
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

impl Logger {
    /// Создаёт новый пустой логгер.
    pub fn new() -> Self {
        Logger {
            entries: Vec::new(),
        }
    }

    /// Записывает сообщение с текущим таймстемпом.
    pub fn log(&mut self, message: &str) {
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        self.entries.push(format!("[{}] {}", now, message));
    }

    /// Возвращает все записи лога.
    pub fn entries(&self) -> &[String] {
        &self.entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_new_logger_when_created_then_has_no_entries() {
        // Given / When
        let logger = Logger::new();

        // Then
        assert!(logger.entries().is_empty());
    }

    #[test]
    fn given_logger_when_log_message_then_entry_contains_message() {
        // Given
        let mut logger = Logger::new();

        // When
        logger.log("тестовое сообщение");

        // Then
        assert_eq!(logger.entries().len(), 1);
        assert!(logger.entries()[0].contains("тестовое сообщение"));
    }

    #[test]
    fn given_logger_when_log_message_then_entry_contains_timestamp() {
        // Given
        let mut logger = Logger::new();

        // When
        logger.log("test");

        // Then
        let entry = &logger.entries()[0];
        assert!(
            entry.starts_with("[20"),
            "Запись должна начинаться с таймстемпа: {}",
            entry
        );
    }

    // 1. Несколько записей — порядок сохранён
    #[test]
    fn given_logger_when_log_multiple_then_entries_in_order() {
        // Given
        let mut logger = Logger::new();

        // When
        logger.log("первое");
        logger.log("второе");
        logger.log("третье");

        // Then
        assert_eq!(logger.entries().len(), 3);
        assert!(logger.entries()[0].contains("первое"));
        assert!(logger.entries()[1].contains("второе"));
        assert!(logger.entries()[2].contains("третье"));
    }

    // 4. Формат записи соответствует паттерну [YYYY-MM-DD HH:MM:SS.mmm] сообщение
    #[test]
    fn given_logger_when_log_then_entry_matches_format() {
        // Given
        let mut logger = Logger::new();
        let re = regex::Regex::new(r"^\[\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\.\d{3}\] .+$").unwrap();

        // When
        logger.log("hello");

        // Then
        let entry = &logger.entries()[0];
        assert!(
            re.is_match(entry),
            "Запись не соответствует формату: {}",
            entry
        );
    }

    use proptest::prelude::*;

    proptest! {
        // 5. Любое сообщение — после log(s) запись содержит s
        #[test]
        fn given_logger_when_log_any_string_then_entry_contains_it(s in "\\PC{1,100}") {
            // Given
            let mut logger = Logger::new();

            // When
            logger.log(&s);

            // Then
            prop_assert!(logger.entries()[0].contains(&s));
        }

        // 6. После N вызовов log(), entries().len() == N
        #[test]
        fn given_logger_when_log_n_times_then_has_n_entries(n in 1usize..50) {
            // Given
            let mut logger = Logger::new();

            // When
            for i in 0..n {
                logger.log(&format!("msg {}", i));
            }

            // Then
            prop_assert_eq!(logger.entries().len(), n);
        }
    }
}
