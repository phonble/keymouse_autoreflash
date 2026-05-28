use std::thread;
use std::time::{Duration, Instant, SystemTime};
use std::env;
use std::sync::{Arc, Mutex};
use rdev::{simulate, Button, EventType, Key, SimulateError, listen};

struct MouseState {
    x: f64,
    y: f64,
}

fn get_mouse_position(state: &Arc<Mutex<MouseState>>) -> (f64, f64) {
    let state = state.lock().unwrap();
    (state.x, state.y)
}

fn is_moved(prev_pos: (f64, f64), curr_pos: (f64, f64), threshold: f64) -> bool {
    let dx = curr_pos.0 - prev_pos.0;
    let dy = curr_pos.1 - prev_pos.1;
    let distance_squared = dx * dx + dy * dy;
    distance_squared > threshold * threshold
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

/// 模拟按键组合（如 Win+D）
fn send_key_combo(keys: &[Key]) -> Result<(), SimulateError> {
    // 按下所有键
    for &key in keys {
        send_keyevent(key, true)?;
    }

    // 释放所有键（逆序）
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

/// 模拟鼠标左键点击
fn send_mouse_left_click() -> Result<(), SimulateError> {
    simulate(&EventType::ButtonPress(Button::Left))?;
    thread::sleep(Duration::from_millis(50));
    simulate(&EventType::ButtonRelease(Button::Left))?;
    thread::sleep(Duration::from_millis(50));
    Ok(())
}

fn simulate_refresh() {
    perform_refresh_sequence();
}

/// 执行完整的刷新序列
fn perform_refresh_sequence() {
    // Win+D 回到桌面
    let _ = send_key_combo(&[Key::MetaLeft, Key::KeyD]);
    thread::sleep(Duration::from_secs(1));
    
    // 移动鼠标到屏幕中央
    let _ = send_mouse_move(960.0, 540.0);
    thread::sleep(Duration::from_millis(300));
    
    // 左键单击
    let _ = send_mouse_left_click();
    thread::sleep(Duration::from_millis(200));
    
    // 右键点击
    let _ = send_mouse_click(Button::Right);
    thread::sleep(Duration::from_millis(500));
    
    // 按 E 键执行刷新
    let _ = send_keyevent(Key::KeyE, true);
    let _ = send_keyevent(Key::KeyE, false);
    thread::sleep(Duration::from_secs(1));
    
    // Alt+Tab 切换回原窗口
    let _ = send_key_combo(&[Key::Alt, Key::Tab]);
    thread::sleep(Duration::from_secs(1));
    
    // 随机移动鼠标到新位置
    let random_x = generate_random_position(1920.0);
    let random_y = generate_random_position(1080.0);
    let _ = send_mouse_move(random_x, random_y);
}

/// 生成随机位置（带边界检查）
fn generate_random_position(max: f64) -> f64 {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    
    // 使用时间戳生成伪随机数
    let random = (timestamp % 10000) as f64 / 10000.0;
    
    // 确保在安全范围内（留出边距）
    let margin = 50.0;  // 边缘留50像素边距
    let safe_max = max - margin * 2.0;
    
    if safe_max > 0.0 {
        margin + random * safe_max
    } else {
        // 如果屏幕太小，返回中心位置
        max / 2.0
    }
}

fn print_help() {
    println!("🖱️  鼠标活动监控与自动刷新器");
    println!("============================\n");
    println!("用法: mouse_position_printer.exe [刷新间隔] [检查间隔]\n");
    println!("参数:");
    println!("  刷新间隔    - 多久执行一次刷新操作（分钟），默认: 30");
    println!("  检查间隔    - 多久检查一次鼠标移动（分钟），默认: 1");
    println!("              必须 < 刷新间隔的 1/2\n");
    println!("示例:");
    println!("  mouse_position_printer.exe           # 使用默认值 (30分钟刷新, 1分钟检查)");
    println!("  mouse_position_printer.exe 60 2      # 60分钟刷新, 2分钟检查");
    println!("  mouse_position_printer.exe 15 0.5    # 15分钟刷新, 0.5分钟(30秒)检查");
    println!("  mouse_position_printer.exe --help    # 显示此帮助信息\n");
    println!("规则:");
    println!("  • 鼠标移动时重置计时器");
    println!("  • 只有持续不动达到刷新间隔才执行刷新");
    println!("  • 移动判断阈值: 4像素\n");
}

fn parse_args() -> Result<(f64, f64), String> {
    let args: Vec<String> = env::args().collect();
    
    // 默认值
    let default_refresh_interval = 30.0;  // 30分钟
    let default_check_interval = 1.0;     // 1分钟
    
    if args.len() == 1 {
        // 没有参数，使用默认值
        return Ok((default_refresh_interval, default_check_interval));
    }
    
    if args.len() == 2 && (args[1] == "--help" || args[1] == "-h" || args[1] == "help") {
        print_help();
        std::process::exit(0);
    }
    
    if args.len() != 3 {
        return Err(format!("❌ 错误: 参数数量不正确\n\n请使用 --help 查看使用方法"));
    }
    
    let refresh_interval: f64 = args[1].parse().map_err(|_| {
        format!("❌ 错误: 刷新间隔 '{}' 不是有效的数字", args[1])
    })?;
    
    let check_interval: f64 = args[2].parse().map_err(|_| {
        format!("❌ 错误: 检查间隔 '{}' 不是有效的数字", args[2])
    })?;
    
    // 验证参数合理性
    if refresh_interval <= 0.0 {
        return Err("❌ 错误: 刷新间隔必须大于0".to_string());
    }
    
    if check_interval <= 0.0 {
        return Err("❌ 错误: 检查间隔必须大于0".to_string());
    }
    
    // 检查间隔必须小于刷新间隔的一半
    let max_check_interval = refresh_interval / 2.0;
    if check_interval >= max_check_interval {
        return Err(format!(
            "❌ 错误: 检查间隔 ({:.2}分钟) 必须小于刷新间隔的一半 ({:.2}分钟)",
            check_interval, max_check_interval
        ));
    }
    
    Ok((refresh_interval, check_interval))
}

fn create_progress_bar(progress: f64, width: usize) -> String {
    let filled = (progress * width as f64) as usize;
    let empty = width - filled;
    let filled_str = "█".repeat(filled);
    let empty_str = "░".repeat(empty);
    format!("{}{}", filled_str, empty_str)
}

fn main() {
    // 解析命令行参数
    let (refresh_interval_min, check_interval_min) = match parse_args() {
        Ok(values) => values,
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    };
    
    // 转换为 Duration
    let refresh_interval = Duration::from_secs_f64(refresh_interval_min * 60.0);
    let check_interval = Duration::from_secs_f64(check_interval_min * 60.0);
    
    println!("🖱️  鼠标活动监控与自动刷新器");
    println!("═══════════════════════════════════");
    println!("⏱️  刷新间隔: {:.2} 分钟 ({:.0} 秒)", refresh_interval_min, refresh_interval.as_secs_f64());
    println!("🔍 检查间隔: {:.2} 分钟 ({:.0} 秒)", check_interval_min, check_interval.as_secs_f64());
    println!("📏 移动阈值: 4 像素");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("💡 提示: 鼠标移动时重置计时器，持续不动才刷新\n");
    
    let movement_threshold = 4.0;  // 移动判断阈值（像素）
    
    // 启动鼠标监听器
    let mouse_state = Arc::new(Mutex::new(MouseState { x: 0.0, y: 0.0 }));
    let mouse_state_listener = Arc::clone(&mouse_state);
    
    thread::spawn(move || {
        if let Err(error) = listen(move |event| {
            if let EventType::MouseMove { x, y } = event.event_type {
                let mut state = mouse_state_listener.lock().unwrap();
                state.x = x;
                state.y = y;
            }
        }) {
            eprintln!("❌ 监听错误: {:?}", error);
        }
    });
    
    // 等待一下让监听器启动
    thread::sleep(Duration::from_millis(100));
    
    let mut last_position = get_mouse_position(&mouse_state);
    let mut last_movement_time = Instant::now();
    let progress_width = 40;  // 进度条宽度
    let mut refresh_count = 0u32;  // 刷新次数计数器
    let mut locked_attempts = 0u32;  // 锁屏期间尝试次数
    
    println!("🎯 开始监控...\n");
    
    loop {
        thread::sleep(check_interval);
        
        let current_position = get_mouse_position(&mouse_state);
        let moved = is_moved(last_position, current_position, movement_threshold);
        
        if moved {
            // 鼠标移动了，重置计时器
            last_movement_time = Instant::now();
            
            // 立即更新进度条显示为0%
            let progress_bar = create_progress_bar(0.0, progress_width);
            print!("\r⏳ [{}] 0.0% (剩余 {:.1} 分钟)", progress_bar, refresh_interval.as_secs_f64() / 60.0);
            use std::io::{self, Write};
            io::stdout().flush().unwrap();
        } else {
            // 鼠标未移动，显示进度条
            let elapsed = last_movement_time.elapsed();
            let elapsed_secs = elapsed.as_secs_f64();
            let total_secs = refresh_interval.as_secs_f64();
            let progress = (elapsed_secs / total_secs).min(1.0);
            
            if elapsed >= refresh_interval {
                // 达到阈值，先更新进度条到100%
                let progress_bar = create_progress_bar(1.0, progress_width);
                print!("\r⏳ [{}] 100.0% (剩余 0.0 分钟)", progress_bar);
                use std::io::{self, Write};
                io::stdout().flush().unwrap();
                
                // 执行刷新
                refresh_count += 1;
                println!("\n\n🔄 第 {} 次刷新操作", refresh_count);
                simulate_refresh();
                
                // 检查是否成功（简单判断：如果locked_attempts增加，说明可能在锁屏期间）
                if locked_attempts > 0 {
                    println!("   ⚠️  锁屏期间第 {} 次尝试", locked_attempts);
                    locked_attempts = 0;  // 重置
                }
                
                // 刷新后重置计时器
                last_movement_time = Instant::now();
            } else {
                // 显示进度条
                let progress_bar = create_progress_bar(progress, progress_width);
                let remaining = total_secs - elapsed_secs;
                let remaining_min = remaining / 60.0;
                
                print!("\r⏳ [{}] {:.1}% (剩余 {:.1} 分钟)", 
                       progress_bar, progress * 100.0, remaining_min);
                use std::io::{self, Write};
                io::stdout().flush().unwrap();
            }
        }
        
        last_position = current_position;
    }
}
