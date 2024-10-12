use std::sync::Arc;
use tokio::process::Command;
use tokio::sync::Mutex;
use tauri::State;

// 定义 JuliaProcessState 结构体，管理 Julia 进程状态
struct JuliaProcessState {
    process: Arc<Mutex<Option<tokio::process::Child>>>,
}

impl JuliaProcessState {
    // 定义 new 函数用于初始化 JuliaProcessState
    fn new() -> Self {
        JuliaProcessState {
            process: Arc::new(Mutex::new(None)),
        }
    }
}

// 异步命令，用于启动 Julia 服务
#[tauri::command]
async fn start_julia_service(state: State<'_, JuliaProcessState>) -> Result<String, String> {
    let mut julia_process = state.process.lock().await;

    if julia_process.is_none() {
        // 使用 Tokio 异步命令启动 Julia 进程
        match Command::new("julia")
            .arg("--project=../src-julia")
            .arg("../src-julia/julia-server.jl")
            .spawn()
        {
            Ok(child) => {
                *julia_process = Some(child);
                Ok("Julia 服务已启动".to_string())
            }
            Err(e) => Err(format!("启动 Julia 服务失败: {:?}", e)),
        }
    } else {
        Err("Julia 服务已经在运行中".to_string())
    }
}

// 异步命令，用于停止 Julia 服务
#[tauri::command]
async fn stop_julia_service(state: State<'_, JuliaProcessState>) -> Result<String, String> {
    let mut julia_process = state.process.lock().await;

    if let Some(mut child) = julia_process.take() {
        // 使用 Tokio 异步 kill 函数停止 Julia 进程
        match child.kill().await {
            Ok(_) => Ok("Julia 服务已停止".to_string()),
            Err(e) => Err(format!("停止 Julia 服务失败: {:?}", e)),
        }
    } else {
        Err("没有运行中的 Julia 服务".to_string())
    }
}

// 简单的问候命令
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 应用的主运行函数
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(JuliaProcessState::new()) // 使用 new() 函数初始化状态
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            start_julia_service,
            stop_julia_service
        ])
        .run(tauri::generate_context!())
        .expect("运行 Tauri 应用程序时出错");
}
