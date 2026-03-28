//! logging.rs  로그 설정 및 로그 출력 모듈
//!
//! 로그 설정을 세팅하고, 일반 동작/탐지/설정 관련 로그를 출력한다.

/// 로그 레벨 정의
#[derive(Debug)]
pub enum LogLevel {
    Debug,
    Info,
    //Warn,
    //Error,
    //Crit,
}

/// 로그 설정 값을 세팅한다.
///
/// # Arguments
/// - 추가 예정
///
/// # Returns
/// - 추가 예정
pub fn set_log() {
    println!("로그 설정");
}

/// 로그 레벨 켜져있는지 확인한다.
pub fn should_log(_level: LogLevel) -> bool {
    true  // TODO: config 연동
}

/// 문자열을 받아 로그로 출력한다.
pub fn message_log(level: LogLevel, msg: String) {
    // TODO: 설정 기반 필터링
    println!("[{:?}] {}", level, msg);
}

/// 로깅 매크로
#[macro_export]
macro_rules! print_log {
    ($level:expr, $($arg:tt)*) => {
        if crate::log::should_log($level) {
            crate::log::message_log($level, format!($($arg)*));
        }
    };
}