use fibonacci_stable::{read_input, print_result, Logger, fibonacci, InputError};

fn main() {
    let mut logger = Logger::new();

    logger.log("Программа запущена");

    loop {
        let n = match read_input(&mut logger) {
            Some(n) => n,
            None => {
                logger.log("Пользователь вышел из программы");
                println!("До свидания!");
                break;
            }
        };

        logger.log(&format!("Старт вычисления fibonacci({})", n));

        match fibonacci(n) {
            Ok(result) => {
                logger.log(&format!("Конец вычисления: fibonacci({}) = {}", n, result));
                print_result(n, result);
                logger.log("Результат выведен в терминал");
                break;
            }
            Err(_) => {
                let error = InputError::ComputationOverflow(n);
                logger.log(&format!("{}", error));
                println!("{}", error);
                println!("Попробуйте снова или введите 'q' для выхода:");
            }
        }
    }

    println!("\n--- Логи ---");
    for entry in logger.entries() {
        println!("{}", entry);
    }
}
