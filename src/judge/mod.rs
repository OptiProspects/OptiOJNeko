mod checker;
mod runner;
mod types;

use anyhow::Result;
use checker::Checker;
use runner::Runner;
use std::time::Duration;
pub use types::*;

pub struct Judge {
    runner: Runner,
    checker: Checker,
}

impl Judge {
    pub fn new(config: JudgeConfig) -> Self {
        let runner = Runner::new(config.clone());
        let checker = Checker::new();
        Self { runner, checker }
    }

    pub async fn judge(&self, test_case: &TestCase) -> Result<JudgeResult> {
        self.judge_all(&[test_case.clone()]).await
    }

    pub async fn judge_all(&self, test_cases: &[TestCase]) -> Result<JudgeResult> {
        println!("开始判题...");

        if let Err(e) = self.runner.compile().await {
            println!("编译错误: {}", e);
            return Ok(JudgeResult {
                status: JudgeStatus::CompilationError,
                time_used: Default::default(),
                memory_used: 0,
                error_message: Some(e.to_string()),
                test_case_results: vec![],
            });
        }
        println!("编译成功!");

        let mut test_case_results = Vec::new();
        let mut max_time = Duration::default();
        let mut max_memory = 0u64;
        let mut final_status = JudgeStatus::Accepted;

        for (i, test_case) in test_cases.iter().enumerate() {
            println!("\n测试点 #{}", i + 1);
            match self.runner.run(&test_case.input).await {
                Ok((output, time_used, memory_used)) => {
                    println!("运行时间: {:?}", time_used);
                    println!("内存使用: {} bytes", memory_used);
                    println!("程序输出: {}", output.trim());
                    println!("期望输出: {}", test_case.expected_output.trim());

                    let status = self.checker.check(test_case, &output);
                    println!("判题结果: {:?}", status);

                    max_time = max_time.max(time_used);
                    max_memory = max_memory.max(memory_used);

                    if status != JudgeStatus::Accepted {
                        final_status = status.clone();
                    }

                    test_case_results.push(TestCaseResult {
                        status,
                        time_used,
                        memory_used,
                        actual_output: output,
                        test_case_id: i,
                    });
                }
                Err(e) => {
                    println!("运行错误: {}", e);
                    test_case_results.push(TestCaseResult {
                        status: JudgeStatus::RuntimeError,
                        time_used: Duration::default(),
                        memory_used: 0,
                        actual_output: e.to_string(),
                        test_case_id: i,
                    });
                    final_status = JudgeStatus::RuntimeError;
                }
            }
        }

        println!("\n判题完成!");
        println!("最终状态: {:?}", final_status);
        println!("最大运行时间: {:?}", max_time);
        println!("最大内存使用: {} bytes", max_memory);

        Ok(JudgeResult {
            status: final_status,
            time_used: max_time,
            memory_used: max_memory,
            error_message: None,
            test_case_results,
        })
    }
}
