use std::io::Write;
use std::process::{Command, Stdio};

fn run_with_input(input: &str) -> String {
    let mut child = Command::new(env!("CARGO_BIN_EXE_fibonacci_stable"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Не удалось запустить программу");

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();

    let output = child.wait_with_output().unwrap();
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn given_valid_input_when_run_then_outputs_result() {
    // Given / When
    let stdout = run_with_input("10\n");

    // Then
    assert!(stdout.contains("fibonacci(10) = 55"), "stdout: {}", stdout);
    assert!(stdout.contains("--- Логи ---"), "stdout: {}", stdout);
}

#[test]
fn given_invalid_then_valid_input_when_run_then_recovers() {
    // Given / When
    let stdout = run_with_input("abc\n5\n");

    // Then
    assert!(
        stdout.contains("Ошибка"),
        "Должно быть сообщение об ошибке: {}",
        stdout
    );
    assert!(stdout.contains("fibonacci(5) = 5"), "stdout: {}", stdout);
}

#[test]
fn given_quit_when_run_then_exits_gracefully() {
    // Given / When
    let stdout = run_with_input("q\n");

    // Then
    assert!(stdout.contains("До свидания!"), "stdout: {}", stdout);
}

#[test]
fn given_overflow_then_valid_input_when_run_then_recovers() {
    // Given / When
    let stdout = run_with_input("200\n10\n");

    // Then
    assert!(
        stdout.contains("RU: Ошибка!"),
        "Должно быть RU сообщение: {}",
        stdout
    );
    assert!(
        stdout.contains("EN: Error!"),
        "Должно быть EN сообщение: {}",
        stdout
    );
    assert!(
        stdout.contains("fibonacci(10) = 55"),
        "Должен быть результат для 10: {}",
        stdout
    );
}

#[test]
fn given_overflow_then_quit_when_run_then_exits_gracefully() {
    // Given / When
    let stdout = run_with_input("200\nq\n");

    // Then
    assert!(
        stdout.contains("RU: Ошибка!"),
        "Должно быть RU сообщение: {}",
        stdout
    );
    assert!(
        stdout.contains("EN: Error!"),
        "Должно быть EN сообщение: {}",
        stdout
    );
    assert!(
        stdout.contains("До свидания!"),
        "Должен быть выход: {}",
        stdout
    );
}

#[test]
fn given_overflow_then_valid_input_when_run_then_logs_contain_both() {
    // Given / When
    let stdout = run_with_input("200\n10\n");

    // Then — логи должны содержать оба ввода
    assert!(
        stdout.contains("Ввод от пользователя: '200'"),
        "Логи должны содержать ввод 200: {}",
        stdout
    );
    assert!(
        stdout.contains("Ввод от пользователя: '10'"),
        "Логи должны содержать ввод 10: {}",
        stdout
    );
}

#[test]
fn given_invalid_input_when_run_then_shows_ru_and_en_error() {
    // Given / When
    let stdout = run_with_input("abc\n5\n");

    // Then
    assert!(
        stdout.contains("RU: Ошибка!"),
        "Должно быть RU сообщение: {}",
        stdout
    );
    assert!(
        stdout.contains("EN: Error!"),
        "Должно быть EN сообщение: {}",
        stdout
    );
    assert!(
        stdout.contains("fibonacci(5) = 5"),
        "Должен быть результат: {}",
        stdout
    );
}
