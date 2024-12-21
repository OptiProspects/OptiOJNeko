use crate::judge::types::{JudgeStatus, TestCase};

pub struct Checker;

impl Checker {
    pub fn new() -> Self {
        Self
    }

    pub fn check(&self, test_case: &TestCase, actual_output: &str) -> JudgeStatus {
        let expected = test_case.expected_output.trim();
        let actual = actual_output.trim();

        if expected == actual {
            JudgeStatus::Accepted
        } else {
            JudgeStatus::WrongAnswer
        }
    }
}
