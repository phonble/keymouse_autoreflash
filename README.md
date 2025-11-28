# keymouse_autoreflash
模拟键盘鼠标操作，自动刷新桌面然后返回原窗口，欺骗后台认为用户处于活跃状态，且支持电脑锁屏,支持任意linux widnows，macos
1.程序编译运行
   cargo run -- 60 
   //表示每60分钟运行依次
2.# 构建发布版本
  cargo build --release

3. # 针对特定平台优化
  cargo build --release --target x86_64-unknown-linux-gnu
4. 运行
  ./deskshow.exe 120

