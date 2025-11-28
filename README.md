# KeyMouse AutoReflash

一个跨平台的键盘鼠标模拟工具，用于自动保持系统活跃状态。

## 功能特性

- 🖱️ 模拟键盘鼠标操作，自动刷新桌面
- 🔄 操作后自动返回原窗口，无感切换
- ⏰ 可自定义执行间隔时间
- 🔒 支持电脑锁屏检测
- 🌍 跨平台支持：Linux、Windows、macOS
- 🎯 后台运行，欺骗系统认为用户处于活跃状态

## 安装与编译

### 从源码编译

```bash
# 克隆项目
git clone <repository-url>
cd keymouse_autoreflash

# 调试运行（每60分钟执行一次）
cargo run -- 60

# 构建发布版本
cargo build --release
