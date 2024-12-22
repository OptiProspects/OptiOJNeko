use crate::grpc::judge_grpc_service::judge_grpc_service_server::JudgeGrpcService;
use crate::grpc::judge_grpc_service::{
    SubmitRequest, SubmitResponse, TestCase as GrpcTestCase, TestCaseResult as GrpcTestCaseResult,
};
use opti_neko::judge::{Judge, JudgeConfig, TestCase, TestCaseResult};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};
use tracing::{error, info};

pub struct JudgeGrpcServiceImpl {
    judge: Arc<Mutex<Judge>>,
}

impl Default for JudgeGrpcServiceImpl {
    fn default() -> Self {
        info!("创建新的 JudgeGrpcServiceImpl 实例");
        let default_config = JudgeConfig {
            time_limit: Duration::from_secs(1),
            memory_limit: 256 * 1024 * 1024,
            language: String::new(),
            source_code: String::new(),
        };

        Self {
            judge: Arc::new(Mutex::new(Judge::new(default_config))),
        }
    }
}

#[tonic::async_trait]
impl JudgeGrpcService for JudgeGrpcServiceImpl {
    async fn submit(
        &self,
        request: Request<SubmitRequest>,
    ) -> Result<Response<SubmitResponse>, Status> {
        let req = request.into_inner();

        if req.language.is_empty() {
            error!("编程语言不能为空");
            return Err(Status::invalid_argument("编程语言不能为空"));
        }

        if req.source_code.is_empty() {
            error!("源代码不能为空");
            return Err(Status::invalid_argument("源代码不能为空"));
        }

        if req.time_limit <= 0 {
            error!("时间限制必须大于0");
            return Err(Status::invalid_argument("时间限制必须大于0"));
        }

        if req.memory_limit == 0 {
            error!("内存限制必须大于0");
            return Err(Status::invalid_argument("内存限制必须大于0"));
        }

        if req.test_cases.is_empty() {
            error!("测试点不能为空");
            return Err(Status::invalid_argument("测试点不能为空"));
        }

        info!(
            language = %req.language,
            time_limit = %req.time_limit,
            memory_limit = %req.memory_limit,
            test_cases_count = %req.test_cases.len(),
            "收到新的提交请求"
        );

        let judge_config = JudgeConfig {
            time_limit: Duration::from_millis(req.time_limit as u64),
            memory_limit: (req.memory_limit as u64) * 1024 * 1024, // 转换 MB 到字节
            language: req.language,
            source_code: req.source_code,
        };

        let mut judge = self.judge.lock().await;
        *judge = Judge::new(judge_config);

        let test_cases: Vec<TestCase> = req
            .test_cases
            .into_iter()
            .map(|tc: GrpcTestCase| TestCase {
                input: tc.input,
                expected_output: tc.expected_output,
            })
            .collect();

        info!("开始执行判题");
        let result = match judge.judge_all(&test_cases).await {
            Ok(r) => r,
            Err(e) => {
                error!("判题执行失败: {}", e);
                return Err(Status::internal(e.to_string()));
            }
        };

        info!(
            status = ?result.status,
            time_used = ?result.time_used,
            memory_used = %result.memory_used,
            "判题完成"
        );

        let test_case_results: Vec<GrpcTestCaseResult> = result
            .test_case_results
            .into_iter()
            .map(|tcr: TestCaseResult| GrpcTestCaseResult {
                status: tcr.status as i32,
                time_used: tcr.time_used.as_millis() as f64,
                memory_used: (tcr.memory_used as f64 / 1024.0 * 100.0).round() / 100.0,
                actual_output: tcr.actual_output,
                test_case_id: tcr.test_case_id as i32,
            })
            .collect();

        let response = SubmitResponse {
            status: result.status as i32,
            time_used: result.time_used.as_millis() as f64,
            memory_used: (result.memory_used as f64 / 1024.0 * 100.0).round() / 100.0,
            error_message: result.error_message.unwrap_or_default(),
            test_case_results,
        };

        Ok(Response::new(response))
    }
}
