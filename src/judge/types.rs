use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudgeResult {
    pub status: JudgeStatus,
    pub time_used: Duration,
    pub memory_used: u64,
    pub error_message: Option<String>,
    pub test_case_results: Vec<TestCaseResult>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JudgeStatus {
    Accepted,
    WrongAnswer,
    TimeLimitExceeded,
    MemoryLimitExceeded,
    RuntimeError,
    CompilationError,
    SystemError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub input: String,
    pub expected_output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCaseResult {
    pub status: JudgeStatus,
    pub time_used: Duration,
    pub memory_used: u64,
    pub actual_output: String,
    pub test_case_id: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudgeConfig {
    pub time_limit: Duration, // 时间限制
    pub memory_limit: u64,    // 内存限制(bytes)
    pub language: String,     // 编程语言
    pub source_code: String,  // 源代码
}
