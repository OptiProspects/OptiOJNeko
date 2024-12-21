use crate::judge::JudgeConfig;
use anyhow::Result;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

#[cfg(target_os = "windows")]
use winapi::um::{
    processthreadsapi::OpenProcess,
    psapi::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS},
    winnt::PROCESS_QUERY_INFORMATION,
};

#[cfg(target_os = "linux")]
use std::fs::read_to_string;

pub struct Runner {
    config: JudgeConfig,
}

impl Runner {
    pub fn new(config: JudgeConfig) -> Self {
        Self { config }
    }

    pub async fn compile(&self) -> Result<()> {
        match self.config.language.as_str() {
            "python" => {
                fs::write("solution.py", &self.config.source_code)?;
                // 检查 Python 是否可用
                let python_cmd = if cfg!(windows) { "python" } else { "python3" };
                let status = Command::new(python_cmd)
                    .arg("-c")
                    .arg("print('test')")
                    .output()?;

                if !status.status.success() {
                    return Err(anyhow::anyhow!("Python interpreter not found"));
                }
            }
            "cpp" => {
                let (source_file, exec_file) = ("solution.cpp", "solution");

                fs::write(source_file, &self.config.source_code)?;

                let status = Command::new("g++")
                    .arg(source_file)
                    .arg("-o")
                    .arg(exec_file)
                    .output()?;

                if !status.status.success() {
                    return Err(anyhow::anyhow!(
                        "Compilation error: {}",
                        String::from_utf8_lossy(&status.stderr)
                    ));
                }

                // 清理源代码文件
                fs::remove_file(source_file).ok();
            }
            "c" => {
                let (source_file, exec_file) = ("solution.c", "solution");

                fs::write(source_file, &self.config.source_code)?;

                let status = Command::new("gcc")
                    .arg(source_file)
                    .arg("-o")
                    .arg(exec_file)
                    .output()?;

                if !status.status.success() {
                    return Err(anyhow::anyhow!(
                        "Compilation error: {}",
                        String::from_utf8_lossy(&status.stderr)
                    ));
                }

                // 清理源代码文件
                fs::remove_file(source_file).ok();
            }
            "java" => {
                fs::write("Main.java", &self.config.source_code)?;

                let status = Command::new("javac").arg("Main.java").output()?;

                if !status.status.success() {
                    return Err(anyhow::anyhow!(
                        "Compilation error: {}",
                        String::from_utf8_lossy(&status.stderr)
                    ));
                }

                // 清理源代码文件
                fs::remove_file("Main.java").ok();
            }
            "go" => {
                let (source_file, exec_file) = ("solution.go", "solution");

                fs::write(source_file, &self.config.source_code)?;

                let status = Command::new("go")
                    .arg("build")
                    .arg("-o")
                    .arg(exec_file)
                    .arg(source_file)
                    .output()?;

                if !status.status.success() {
                    return Err(anyhow::anyhow!(
                        "Compilation error: {}",
                        String::from_utf8_lossy(&status.stderr)
                    ));
                }

                // 清理源代码文件
                fs::remove_file(source_file).ok();
            }
            _ => return Err(anyhow::anyhow!("Unsupported language")),
        }
        Ok(())
    }

    #[cfg(target_os = "linux")]
    fn get_memory_usage(pid: u32) -> Result<u64> {
        let status_file = format!("/proc/{}/status", pid);
        let content = read_to_string(status_file)?;

        for line in content.lines() {
            if line.starts_with("VmRSS:") {
                let memory = line
                    .split_whitespace()
                    .nth(1)
                    .ok_or_else(|| anyhow::anyhow!("Failed to parse memory usage"))?
                    .parse::<u64>()?;
                // 转换 KB 到字节
                return Ok(memory * 1024);
            }
        }

        Err(anyhow::anyhow!("Could not find memory usage"))
    }

    #[cfg(target_os = "windows")]
    fn get_memory_usage(pid: u32) -> Result<u64> {
        unsafe {
            let handle = OpenProcess(PROCESS_QUERY_INFORMATION, 0, pid);
            if handle.is_null() {
                return Err(anyhow::anyhow!("Failed to get process handle"));
            }

            let mut memory_counters = PROCESS_MEMORY_COUNTERS {
                cb: std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
                PageFaultCount: 0,
                PeakWorkingSetSize: 0,
                WorkingSetSize: 0,
                QuotaPeakPagedPoolUsage: 0,
                QuotaPagedPoolUsage: 0,
                QuotaPeakNonPagedPoolUsage: 0,
                QuotaNonPagedPoolUsage: 0,
                PagefileUsage: 0,
                PeakPagefileUsage: 0,
            };

            if GetProcessMemoryInfo(
                handle,
                &mut memory_counters,
                std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
            ) == 0
            {
                return Err(anyhow::anyhow!("Failed to get process memory info"));
            }

            // 使用 WorkingSetSize，它表示物理内存使用量
            Ok(memory_counters.WorkingSetSize as u64)
        }
    }

    pub async fn run(&self, input: &str) -> Result<(String, Duration, u64)> {
        let start = Instant::now();

        let mut command = match self.config.language.as_str() {
            "cpp" | "c" => Command::new("./solution"),
            "python" => {
                let mut cmd = if cfg!(windows) {
                    Command::new("python")
                } else {
                    Command::new("python3")
                };
                cmd.arg("solution.py");
                cmd
            }
            "java" => {
                let mut cmd = Command::new("java");
                cmd.arg("Main");
                cmd
            }
            "go" => Command::new("./solution"),
            _ => return Err(anyhow::anyhow!("Unsupported language")),
        };

        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let pid = child.id();

        // 写入输入后立即关闭 stdin
        {
            let mut stdin = child
                .stdin
                .take()
                .ok_or_else(|| anyhow::anyhow!("Failed to open stdin"))?;
            stdin.write_all(input.as_bytes())?;
            // stdin 在这里会自动关闭
        }

        // 设置超时检查
        let timeout = tokio::time::sleep(self.config.time_limit);
        tokio::pin!(timeout);

        let should_stop = Arc::new(AtomicBool::new(false));
        let should_stop_clone = should_stop.clone();
        let memory_usage = Arc::new(AtomicU64::new(0));
        let memory_usage_clone = memory_usage.clone();

        let monitoring = thread::spawn(move || {
            let mut max_memory: u64 = 0;
            while !should_stop_clone.load(Ordering::SeqCst) {
                if let Ok(current_memory) = Self::get_memory_usage(pid) {
                    max_memory = max_memory.max(current_memory);
                    memory_usage_clone.store(max_memory, Ordering::SeqCst);
                }
                thread::sleep(Duration::from_millis(1));
            }
        });

        // 使用 tokio 的 spawn_blocking 来等待子进程
        let output = tokio::task::spawn_blocking(move || child.wait_with_output());

        tokio::select! {
            result = output => {
                should_stop.store(true, Ordering::SeqCst);
                monitoring.join().ok();

                let output = result??;
                let duration = start.elapsed();
                let max_memory = memory_usage.load(Ordering::SeqCst);

                if !output.status.success() {
                    return Err(anyhow::anyhow!("Runtime error"));
                }

                Ok((String::from_utf8(output.stdout)?, duration, max_memory))
            }
            _ = timeout => {
                should_stop.store(true, Ordering::SeqCst);
                monitoring.join().ok();
                Err(anyhow::anyhow!("Time limit exceeded"))
            }
        }
    }
}

impl Drop for Runner {
    fn drop(&mut self) {
        // 清理所有可能的临时文件
        let files = match self.config.language.as_str() {
            "cpp" | "c" => vec!["solution", "solution.exe"],
            "python" => vec!["solution.py", "__pycache__"],
            "java" => vec!["Main.class", "Main.java"],
            "go" => vec!["solution", "solution.exe"],
            _ => vec![],
        };

        for file in files {
            if Path::new(file).exists() {
                if file == "__pycache__" {
                    fs::remove_dir_all(file).ok();
                } else {
                    fs::remove_file(file).ok();
                }
            }
        }
    }
}
