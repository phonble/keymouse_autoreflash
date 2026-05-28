# Mouse Activity Monitor & Auto Refresh Tool / 鼠标活动监控与自动刷新工具

![Version](https://img.shields.io/badge/version-v1.1.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey)

A cross-platform desktop automation tool that monitors mouse activity and automatically refreshes the desktop when idle. Keeps your system active and prevents screen lock.

跨平台桌面自动化工具，监控鼠标活动并在空闲时自动刷新桌面。保持系统活跃状态，防止屏幕锁定。

## ✨ Features / 功能特性

- 🖱️ **Simulate keyboard and mouse operations** - Automatically refresh desktop
  模拟键盘鼠标操作，自动刷新桌面
- 🔄 **Auto return to original window** - Seamless switching after operations
  操作后自动返回原窗口，无感切换
- ⏰ **Customizable execution interval** - Set your preferred refresh frequency
  可自定义执行间隔时间
- 🔒 **Screen lock detection** - Smart handling during locked state
  支持电脑锁屏检测
- 🌍 **Cross-platform support** - Works on Linux, Windows, macOS
  跨平台支持：Linux、Windows、macOS
- 🎯 **Background operation** - Keeps system active, prevents idle detection
  后台运行，欺骗系统认为用户处于活跃状态

## 📖 Basic Usage

```bash
# Use default parameters (30 min refresh, 1 min check)
keymouse_autoreflash.exe

# Custom parameters
keymouse_autoreflash.exe [refresh_interval] [check_interval]
```

## 🔧 Parameters

- **Refresh Interval**: How often to execute desktop refresh (in minutes), supports decimals
- **Check Interval**: How often to check mouse movement (in minutes), supports decimals
- **Constraints**: 
  - Both parameters must be greater than 0
  - Check interval < Refresh interval / 2

## 💡 Usage Examples

```bash
# 1. Use default values (recommended)
keymouse_autoreflash.exe
# Equivalent to: refresh every 30 minutes, check mouse every 1 minute

# 2. 60 min refresh, 2 min check
keymouse_autoreflash.exe 60 2

# 3. 15 min refresh, 30 sec check
keymouse_autoreflash.exe 15 0.5

# 4. 45 min refresh, 5 min check
keymouse_autoreflash.exe 45 5

# 5. View help
keymouse_autoreflash.exe --help
```

## ⚠️ Parameter Validation Examples

```bash
# ❌ Error: Check interval too large (equals half)
keymouse_autoreflash.exe 30 15
# Error: Check interval (15.00 min) must be less than half of refresh interval (15.00 min)

# ❌ Error: Check interval too large (exceeds half)
keymouse_autoreflash.exe 30 20
# Error: Check interval (20.00 min) must be less than half of refresh interval (15.00 min)

# ✅ Correct: Check interval less than half
keymouse_autoreflash.exe 30 14.9
# Validation passed

# ❌ Error: Negative value
keymouse_autoreflash.exe -10 1
# Error: Refresh interval must be greater than 0

# ❌ Error: Zero value
keymouse_autoreflash.exe 30 0
# Error: Check interval must be greater than 0

# ❌ Error: Non-numeric
keymouse_autoreflash.exe abc 1
# Error: Refresh interval 'abc' is not a valid number
```

## 🎯 How It Works

1. **Real-time detection** of mouse position changes
2. **Movement threshold**: Distance > 4 pixels considered as movement
3. **Timer reset**: Immediately resets inactivity timer when mouse movement detected
4. **Auto refresh**: Only executes desktop refresh when mouse remains stationary for the full refresh interval

### 🔄 Refresh Operation Sequence

When triggered, the program uses the **rdev library** to execute real system-level operations:

1. **Win+D** - Simulates keypress using `rdev::simulate()` to minimize all windows and show desktop
2. **Move mouse** - Moves cursor to screen center (960, 540)
3. **Left-click** - Simulates left button press and release on desktop
4. **Right-click** - Simulates mouse right button press and release to open desktop context menu
5. **Press E** - Simulates E key press and release to select "Refresh" option
6. **Alt+Tab** - Simulates Alt+Tab to switch back to original window
7. **Random position** - Moves mouse to a random position on screen (within 1920x1080 bounds)

> ⚠️ **Important**: 
> - These are **real system-level input events** implemented via `rdev` library, fully recognized by Windows as genuine user actions!
> - After refresh completes, mouse randomly stops at a screen position (with 50px margin)
> - If screen resolution is smaller than 1920x1080, program automatically adjusts to safe range

## 📊 Output Example

```
🖱️  Mouse Activity Monitor & Auto Refresh Tool
═══════════════════════════════════
⏱️  Refresh Interval: 30.00 minutes (1800 seconds)
🔍 Check Interval: 1.00 minute (60 seconds)
📏 Movement Threshold: 4 pixels
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
💡 Tip: Timer resets on mouse movement, only refreshes when continuously idle

🎯 Monitoring started...

⏳ [████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░] 20.0% (24.0 min remaining)
⏳ [███████████████░░░░░░░░░░░░░░░░░░░░░] 40.0% (18.0 min remaining)
🟢 Mouse activity detected - Timer reset
⏳ [██░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░] 5.0% (28.5 min remaining)
...
⏳ [████████████████████████████████████] 100.0% (0.0 min remaining)

🔴 Mouse has been stationary for 30.0 minutes!

🔄 Executing desktop refresh...
   Simulating Win+D to show desktop
   Simulating right-click menu refresh
   Returning to original window
✅ Refresh completed
```

## 🎨 Icon Legend

- 🟢 Green: Mouse movement detected, timer reset
- 🔴 Red: Refresh threshold reached, executing refresh
- ⏳ Hourglass: Mouse idle, showing remaining time
- 🔄 Cycle: Currently executing desktop refresh operation

## 📦 Installation & Compilation / 安装与编译

### Download Pre-built Binary / 下载预编译版本

**Recommended for most users / 推荐大多数用户使用**

Download the latest Windows executable from [GitHub Releases](https://github.com/phonble/keymouse_autoreflash/releases)

从 [GitHub Releases](https://github.com/phonble/keymouse_autoreflash/releases) 下载最新的 Windows 可执行文件

**Available releases / 可用版本:**
- v1.1.0 (Latest) - `keymouse_autoreflash.exe` with enhanced features
- v1.0.0 - `keymouse_autoreflash.exe` initial release

### Build from Source / 从源码编译

#### Prerequisites / 前置要求

- Rust toolchain (install from https://rustup.rs)
- For Windows cross-compilation on Linux:
  ```bash
  # Install mingw-w64 toolchain
  sudo apt-get install gcc-mingw-w64-x86-64
  
  # Add Windows target
  rustup target add x86_64-pc-windows-gnu
  ```

#### Build Steps / 编译步骤

```bash
# Clone repository / 克隆项目
git clone https://github.com/phonble/keymouse_autoreflash.git
cd keymouse_autoreflash

# Debug run (execute every 60 minutes) / 调试运行（每60分钟执行一次）
cargo run -- 60

# Build release version / 构建发布版本
cargo build --release

# For Windows cross-compilation (recommended) / Windows 交叉编译（推荐）
cargo build --release --target x86_64-pc-windows-gnu
```

#### Executable Location / 可执行文件位置

After building, find the executable:

- **Windows native build**: `target\release\keymouse_autoreflash.exe`
- **Linux/macOS native build**: `target/release/keymouse_autoreflash`
- **Cross-compile for Windows**: `target/x86_64-pc-windows-gnu/release/keymouse_autoreflash.exe`

#### Usage Examples / 使用示例

```bash
# Windows (from command prompt)
keymouse_autoreflash.exe          # Use default 30 min interval
keymouse_autoreflash.exe 60       # Execute every 60 minutes
keymouse_autoreflash.exe 30 1     # Refresh every 30 min, check every 1 min

# Linux/macOS
./keymouse_autoreflash            # Use default 30 min interval
./keymouse_autoreflash 60         # Execute every 60 minutes
./keymouse_autoreflash 30 1       # Refresh every 30 min, check every 1 min
```

#### Cross-compile for Windows (from Linux) / 从 Linux 交叉编译 Windows 版本

```bash
cargo build --release --target x86_64-pc-windows-gnu

# Executable location / 可执行文件位置
target/x86_64-pc-windows-gnu/release/keymouse_autoreflash.exe
```

#### Build Optimization (Optional) / 构建优化（可选）

For smaller binary size, add to `Cargo.toml`:

```toml
[profile.release]
lto = true
codegen-units = 1
strip = true
```

## 📝 Version History

### v1.1.0 (Current)
- ✅ Proper separation of source code and binary releases
- ✅ Added left-click before right-click in refresh sequence
- ✅ Improved README documentation with English translation
- ✅ Enhanced parameter validation with clear error messages
- ✅ Progress bar display with real-time updates
- ✅ Random mouse positioning after refresh completion
- ✅ Silent error handling for robustness

### v1.0.0
- Initial release
- Basic mouse monitoring functionality
- Desktop auto-refresh capability

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## 📄 License

This project is licensed under the MIT License.

## ⚙️ Technical Details

- **Language**: Rust 2024 Edition
- **Key Dependencies**: 
  - `rdev` v0.5 - Cross-platform input simulation
  - `tokio` v1 - Async runtime
- **Platform**: Windows (compiled for x86_64-pc-windows-msvc)
