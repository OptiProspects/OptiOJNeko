use opti_neko::{Judge, JudgeConfig, JudgeStatus, TestCase};
use std::time::Duration;

#[tokio::test]
async fn test_accepted_submission() {
    let config = JudgeConfig {
        time_limit: Duration::from_secs(1),
        memory_limit: 256 * 1024 * 1024, // 256MB
        language: "cpp".to_string(),
        source_code: r#"
            #include <iostream>
            using namespace std;
            int main() {
                int a, b;
                cin >> a >> b;
                cout << a + b << endl;
                return 0;
            }
        "#
        .to_string(),
    };

    let test_case = TestCase {
        input: "1 2\n".to_string(),
        expected_output: "3\n".to_string(),
    };

    let judge = Judge::new(config);
    let result = judge.judge(&test_case).await.unwrap();
    assert_eq!(result.status, JudgeStatus::Accepted);
}

#[tokio::test]
async fn test_wrong_answer() {
    let config = JudgeConfig {
        time_limit: Duration::from_secs(1),
        memory_limit: 256 * 1024 * 1024,
        language: "cpp".to_string(),
        source_code: r#"
            #include <iostream>
            using namespace std;
            int main() {
                int a, b;
                cin >> a >> b;
                cout << a - b << endl;  // 错误的实现
                return 0;
            }
        "#
        .to_string(),
    };

    let test_case = TestCase {
        input: "1 2\n".to_string(),
        expected_output: "3\n".to_string(),
    };

    let judge = Judge::new(config);
    let result = judge.judge(&test_case).await.unwrap();
    assert_eq!(result.status, JudgeStatus::WrongAnswer);
}

#[tokio::test]
async fn test_compilation_error() {
    let config = JudgeConfig {
        time_limit: Duration::from_secs(1),
        memory_limit: 256 * 1024 * 1024,
        language: "cpp".to_string(),
        source_code: r#"
            #include <iostream>
            using namespace std;
            int main() {
                cout << "Hello World!" // 缺少分号
                return 0;
            }
        "#
        .to_string(),
    };

    let test_case = TestCase {
        input: "".to_string(),
        expected_output: "Hello World!\n".to_string(),
    };

    let judge = Judge::new(config);
    let result = judge.judge(&test_case).await.unwrap();
    assert_eq!(result.status, JudgeStatus::CompilationError);
}

#[tokio::test]
async fn test_multiple_test_cases() {
    let config = JudgeConfig {
        time_limit: Duration::from_secs(1),
        memory_limit: 256 * 1024 * 1024,
        language: "cpp".to_string(),
        source_code: r#"
            #include <iostream>
            using namespace std;
            int main() {
                int a, b;
                cin >> a >> b;
                cout << a + b << endl;
                return 0;
            }
        "#
        .to_string(),
    };

    let test_cases = vec![
        TestCase {
            input: "1 2\n".to_string(),
            expected_output: "3\n".to_string(),
        },
        TestCase {
            input: "5 7\n".to_string(),
            expected_output: "12\n".to_string(),
        },
        TestCase {
            input: "0 0\n".to_string(),
            expected_output: "0\n".to_string(),
        },
    ];

    let judge = Judge::new(config);
    let result = judge.judge_all(&test_cases).await.unwrap();

    assert_eq!(result.status, JudgeStatus::Accepted);
    assert_eq!(result.test_case_results.len(), 3);

    for test_result in &result.test_case_results {
        assert_eq!(test_result.status, JudgeStatus::Accepted);
    }
}

#[tokio::test]
async fn test_partial_correct() {
    let config = JudgeConfig {
        time_limit: Duration::from_secs(1),
        memory_limit: 256 * 1024 * 1024,
        language: "cpp".to_string(),
        source_code: r#"
            #include <iostream>
            using namespace std;
            int main() {
                int a, b;
                cin >> a >> b;
                if (a == 1) cout << a + b << endl;
                else cout << a - b << endl;  // 只有第一个测试点正确
                return 0;
            }
        "#
        .to_string(),
    };

    let test_cases = vec![
        TestCase {
            input: "1 2\n".to_string(),
            expected_output: "3\n".to_string(),
        },
        TestCase {
            input: "5 3\n".to_string(),
            expected_output: "8\n".to_string(),
        },
    ];

    let judge = Judge::new(config);
    let result = judge.judge_all(&test_cases).await.unwrap();

    assert_eq!(result.status, JudgeStatus::WrongAnswer);
    assert_eq!(result.test_case_results[0].status, JudgeStatus::Accepted);
    assert_eq!(result.test_case_results[1].status, JudgeStatus::WrongAnswer);
}

#[tokio::test]
async fn test_python_submission() {
    let config = JudgeConfig {
        time_limit: Duration::from_secs(1),
        memory_limit: 256 * 1024 * 1024,
        language: "python".to_string(),
        source_code: r#"
a, b = map(int, input().split())
print(a + b)
        "#
        .to_string(),
    };

    let test_cases = vec![
        TestCase {
            input: "1 2\n".to_string(),
            expected_output: "3\n".to_string(),
        },
        TestCase {
            input: "100 200\n".to_string(),
            expected_output: "300\n".to_string(),
        },
    ];

    let judge = Judge::new(config);
    let result = judge.judge_all(&test_cases).await.unwrap();
    assert_eq!(result.status, JudgeStatus::Accepted);
}

#[tokio::test]
async fn test_go_submission() {
    let config = JudgeConfig {
        time_limit: Duration::from_secs(1),
        memory_limit: 256 * 1024 * 1024,
        language: "go".to_string(),
        source_code: r#"
package main

import "fmt"

func main() {
    var a, b int
    fmt.Scanf("%d %d", &a, &b)
    fmt.Println(a + b)
}
        "#
        .to_string(),
    };

    let test_cases = vec![
        TestCase {
            input: "1 2\n".to_string(),
            expected_output: "3\n".to_string(),
        },
        TestCase {
            input: "100 200\n".to_string(),
            expected_output: "300\n".to_string(),
        },
    ];

    let judge = Judge::new(config);
    let result = judge.judge_all(&test_cases).await.unwrap();
    assert_eq!(result.status, JudgeStatus::Accepted);
}
