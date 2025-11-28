use clap::Parser;
use rdev::{simulate, Button, EventType, Key, SimulateError};
use std::thread;
use std::time::Duration;
use tokio::time;

/// 桌面自动刷新程序
/// 直接运行 `deskshow.exe 10` 即可设置10分钟刷新一次
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 刷新间隔时间（单位：分钟），默认15分钟
    #[arg(default_value_t = 15)]
    interval: u64,
}

/// 模拟键盘按键
fn send_keyevent(key: Key, press: bool) -> Result<(), SimulateError> {
    let event_type = if press {
        EventType::KeyPress(key)
    } else {
        EventType::KeyRelease(key)
    };
    simulate(&event_type)?;
    thread::sleep(Duration::from_millis(50));
    Ok(())
}

/// 模拟按键组合（如 Win+D, Alt+Tab）
fn send_key_combo(keys: &[Key]) -> Result<(), SimulateError> {
    // 按下所有键
    for &key in keys {
        send_keyevent(key, true)?;
    }

    // 释放所有键（逆序，符合常规操作）
    for &key in keys.iter().rev() {
        send_keyevent(key, false)?;
    }

    Ok(())
}

/// 模拟鼠标点击
fn send_mouse_click(button: Button) -> Result<(), SimulateError> {
    simulate(&EventType::ButtonPress(button))?;
    thread::sleep(Duration::from_millis(50));
    simulate(&EventType::ButtonRelease(button))?;
    thread::sleep(Duration::from_millis(50));
    Ok(())
}

/// 模拟鼠标移动
fn send_mouse_move(x: f64, y: f64) -> Result<(), SimulateError> {
    simulate(&EventType::MouseMove { x, y })?;
    thread::sleep(Duration::from_millis(10));
    Ok(())
}

/// 获取屏幕中心坐标（根据常见分辨率调整）
fn get_screen_center() -> (f64, f64) {
    // 常见的1080p屏幕中心，你可以根据实际分辨率调整
    (960.0, 540.0)
}

/// 执行完整的刷新序列
async fn perform_refresh_sequence() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 开始执行刷新序列...");

    // 1. 使用 Win + D 回到桌面
    println!("📋 模拟 Win + D 回到桌面...");
    send_key_combo(&[Key::MetaLeft, Key::KeyD])?;
    time::sleep(Duration::from_secs(2)).await;

    // 2. 移动鼠标到屏幕中央
    let (center_x, center_y) = get_screen_center();
    println!("🖱️ 移动鼠标到屏幕中央: ({}, {})", center_x, center_y);
    send_mouse_move(center_x, center_y)?;
    time::sleep(Duration::from_millis(500)).await;

    // 3. 执行右键刷新
    println!("🔄 执行右键刷新...");
    send_mouse_click(Button::Right)?;
    time::sleep(Duration::from_millis(800)).await;

    // 4. 按 R 键选择刷新
    println!("⌨️ 按 R 键选择刷新...");
    send_keyevent(Key::KeyR, true)?;
    send_keyevent(Key::KeyR, false)?;

    time::sleep(Duration::from_secs(1)).await;

    // 5. 使用 Alt + Tab 切换回原窗口
    println!("🔄 使用 Alt + Tab 切换回原窗口...");
    send_key_combo(&[Key::Alt, Key::Tab])?;
    time::sleep(Duration::from_secs(1)).await;

    println!("✅ 刷新序列执行完成");
    Ok(())
}

#[tokio::main]
async fn main() {
    // 解析命令行参数
    let args = Args::parse();
    let interval_secs = args.interval * 60;

    println!("🎯 桌面自动刷新程序已启动！");
    println!("⏰ 将每隔 {} 分钟执行一次刷新操作", args.interval);
    println!("🛑 按 Ctrl+C 退出程序\n");

    // 初始等待，避免程序启动后立即执行
    time::sleep(Duration::from_secs(5)).await;

    // 处理 Ctrl+C 信号
    let ctrl_c = tokio::signal::ctrl_c();
    tokio::pin!(ctrl_c);

    let mut attempt_count = 0;

    loop {
        tokio::select! {
            _ = &mut ctrl_c => {
                println!("\n🛑 程序被用户中断，退出...");
                break;
            }
            _ = async {
                attempt_count += 1;
                println!("\n📊 第 {} 次执行刷新操作", attempt_count);

                match perform_refresh_sequence().await {
                    Ok(_) => {
                        println!("⏳ 操作完成，等待 {} 分钟...", args.interval);
                    }
                    Err(e) => {
                        eprintln!("❌ 执行过程中出现错误: {}", e);
                        eprintln!("⚠️ 将继续在 {} 分钟后重试...", args.interval);
                    }
                }

                time::sleep(Duration::from_secs(interval_secs)).await;
            } => {}
        }
    }
}
