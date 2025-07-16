#!/bin/bash

# 系统优化脚本 - 解决"Too many open files"问题
# 适用于 macOS 和 Linux 系统

echo "🔧 开始系统优化，解决文件描述符限制问题..."

# 检测操作系统
if [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
    echo "📱 检测到 macOS 系统"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="linux"
    echo "🐧 检测到 Linux 系统"
else
    echo "❌ 不支持的操作系统: $OSTYPE"
    exit 1
fi

# 显示当前限制
echo "📊 当前系统限制："
echo "   软限制: $(ulimit -Sn)"
echo "   硬限制: $(ulimit -Hn)"

# macOS 优化
if [[ "$OS" == "macos" ]]; then
    echo "🍎 正在优化 macOS 系统..."
    
    # 创建 launchd 配置文件以永久提高文件描述符限制
    sudo tee /Library/LaunchDaemons/limit.maxfiles.plist > /dev/null <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
        "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
  <dict>
    <key>Label</key>
    <string>limit.maxfiles</string>
    <key>ProgramArguments</key>
    <array>
      <string>launchctl</string>
      <string>limit</string>
      <string>maxfiles</string>
      <string>65536</string>
      <string>65536</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>ServiceIPC</key>
    <false/>
  </dict>
</plist>
EOF

    # 创建进程限制配置
    sudo tee /Library/LaunchDaemons/limit.maxproc.plist > /dev/null <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
        "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
  <dict>
    <key>Label</key>
    <string>limit.maxproc</string>
    <key>ProgramArguments</key>
    <array>
      <string>launchctl</string>
      <string>limit</string>
      <string>maxproc</string>
      <string>2048</string>
      <string>2048</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>ServiceIPC</key>
    <false/>
  </dict>
</plist>
EOF

    # 立即应用限制
    sudo launchctl load -w /Library/LaunchDaemons/limit.maxfiles.plist
    sudo launchctl load -w /Library/LaunchDaemons/limit.maxproc.plist
    
    # 立即设置当前会话的限制
    ulimit -n 65536
    
    echo "✅ macOS 优化完成"
    echo "⚠️  请重启终端或系统以使所有更改生效"

# Linux 优化
elif [[ "$OS" == "linux" ]]; then
    echo "🐧 正在优化 Linux 系统..."
    
    # 备份原始配置
    sudo cp /etc/security/limits.conf /etc/security/limits.conf.backup.$(date +%Y%m%d_%H%M%S)
    
    # 添加文件描述符限制配置
    sudo tee -a /etc/security/limits.conf > /dev/null <<EOF

# 解决 "Too many open files" 问题
# 由 mytips 优化脚本添加
*               soft    nofile          65536
*               hard    nofile          65536
root            soft    nofile          65536
root            hard    nofile          65536

# 进程数限制
*               soft    nproc           32768
*               hard    nproc           32768
root            soft    nproc           32768
root            hard    nproc           32768
EOF

    # 修改 systemd 限制（如果存在）
    if command -v systemctl >/dev/null 2>&1; then
        sudo mkdir -p /etc/systemd/system.conf.d
        sudo tee /etc/systemd/system.conf.d/limits.conf > /dev/null <<EOF
[Manager]
DefaultLimitNOFILE=65536
DefaultLimitNPROC=32768
EOF
        
        # 重新加载 systemd 配置
        sudo systemctl daemon-reload
        echo "✅ systemd 限制已更新"
    fi
    
    # 修改 PAM 限制
    if ! grep -q "pam_limits.so" /etc/pam.d/common-session 2>/dev/null; then
        if [ -f /etc/pam.d/common-session ]; then
            echo "session required pam_limits.so" | sudo tee -a /etc/pam.d/common-session
        fi
    fi
    
    # 立即设置当前会话的限制
    ulimit -n 65536
    
    echo "✅ Linux 优化完成"
    echo "⚠️  请重新登录或重启系统以使所有更改生效"
fi

# 创建 sqld 启动脚本，包含优化参数
echo "🚀 创建优化的 sqld 启动脚本..."

cat > start_sqld_optimized.sh <<'EOF'
#!/bin/bash

# 优化的 sqld 启动脚本
# 包含文件描述符和连接优化

# 设置环境变量
export RUST_LOG=info
export RUST_BACKTRACE=1

# 提高当前进程的文件描述符限制
ulimit -n 65536

# 创建数据库目录（如果不存在）
if [ ! -d "mytips.sqld" ]; then
    echo "📁 创建数据库目录..."
    mkdir -p mytips.sqld/dbs/default
    mkdir -p mytips.sqld/metastore
fi

# 清理可能存在的锁文件
echo "🧹 清理锁文件..."
find mytips.sqld -name "*.lock" -delete 2>/dev/null || true
find mytips.sqld -name "*-wal" -size 0 -delete 2>/dev/null || true

# 设置数据库文件权限
chmod -R 755 mytips.sqld

echo "🗄️  启动 sqld 服务器..."
echo "📊 文件描述符限制: $(ulimit -n)"
echo "🌐 监听地址: http://0.0.0.0:8888"
echo "💾 数据库路径: mytips.sqld"
echo ""

# 启动 sqld，添加优化参数
exec sqld \
    --http-listen-addr 0.0.0.0:8888 \
    --db-path=mytips.sqld \
    --max-connections=50 \
    --checkpoint-interval=3600 \
    --idle-timeout=300
EOF

chmod +x start_sqld_optimized.sh

# 创建系统监控脚本
cat > monitor_system.sh <<'EOF'
#!/bin/bash

# 系统监控脚本 - 监控文件描述符使用情况

echo "📊 系统资源监控"
echo "=================="

# 显示当前限制
echo "🔢 文件描述符限制:"
echo "   软限制: $(ulimit -Sn)"
echo "   硬限制: $(ulimit -Hn)"
echo ""

# 显示系统负载
echo "⚡ 系统负载:"
if command -v uptime >/dev/null 2>&1; then
    uptime
fi
echo ""

# 显示内存使用
echo "💾 内存使用:"
if command -v free >/dev/null 2>&1; then
    free -h
elif command -v vm_stat >/dev/null 2>&1; then
    vm_stat | head -5
fi
echo ""

# 显示打开的文件数量
echo "📁 打开的文件数量:"
if command -v lsof >/dev/null 2>&1; then
    echo "   总计: $(lsof | wc -l)"
    if pgrep sqld >/dev/null; then
        echo "   sqld 进程: $(lsof -p $(pgrep sqld) | wc -l)"
    fi
else
    echo "   lsof 命令不可用"
fi
echo ""

# 显示网络连接
echo "🌐 网络连接:"
if command -v netstat >/dev/null 2>&1; then
    echo "   活跃连接: $(netstat -an | grep ESTABLISHED | wc -l)"
    echo "   监听端口: $(netstat -tlnp 2>/dev/null | grep :8888 || echo "未找到 8888 端口")"
elif command -v ss >/dev/null 2>&1; then
    echo "   活跃连接: $(ss -t state established | wc -l)"
    echo "   监听端口: $(ss -tlnp | grep :8888 || echo "未找到 8888 端口")"
fi
echo ""

# 检查 sqld 进程状态
echo "🔍 sqld 进程状态:"
if pgrep sqld >/dev/null; then
    echo "   ✅ sqld 正在运行"
    echo "   PID: $(pgrep sqld)"
    if command -v ps >/dev/null 2>&1; then
        ps aux | grep "[s]qld" | head -1
    fi
else
    echo "   ❌ sqld 未运行"
fi

EOF

chmod +x monitor_system.sh

# 创建清理脚本
cat > cleanup_database.sh <<'EOF'
#!/bin/bash

# 数据库清理脚本 - 清理 WAL 文件和临时文件

echo "🧹 数据库清理工具"
echo "=================="

if [ ! -d "mytips.sqld" ]; then
    echo "❌ 数据库目录不存在"
    exit 1
fi

# 停止 sqld 进程
if pgrep sqld >/dev/null; then
    echo "⏹️  停止 sqld 进程..."
    pkill sqld
    sleep 2
fi

# 清理 WAL 文件
echo "🗑️  清理 WAL 文件..."
find mytips.sqld -name "*-wal" -exec ls -lh {} \;
find mytips.sqld -name "*-wal" -size +10M -delete
echo "   大型 WAL 文件已清理"

# 清理 SHM 文件
echo "🗑️  清理 SHM 文件..."
find mytips.sqld -name "*-shm" -delete

# 清理锁文件
echo "🗑️  清理锁文件..."
find mytips.sqld -name "*.lock" -delete

# 清理临时文件
echo "🗑️  清理临时文件..."
find mytips.sqld -name "tmp*" -delete
find mytips.sqld -name "*.tmp" -delete

# 显示清理后的大小
echo "📏 清理后的数据库大小:"
du -sh mytips.sqld/

echo "✅ 数据库清理完成"
echo "💡 现在可以重新启动 sqld"

EOF

chmod +x cleanup_database.sh

# 显示完成信息
echo ""
echo "🎉 系统优化完成！"
echo ""
echo "📋 可用的脚本："
echo "   ./start_sqld_optimized.sh  - 启动优化的 sqld 服务器"
echo "   ./monitor_system.sh        - 监控系统资源使用情况"
echo "   ./cleanup_database.sh      - 清理数据库文件"
echo ""
echo "🔧 优化内容："
echo "   ✅ 提高文件描述符限制至 65536"
echo "   ✅ 优化进程数限制"
echo "   ✅ 创建专用启动脚本"
echo "   ✅ 添加系统监控工具"
echo "   ✅ 提供数据库清理工具"
echo ""
echo "⚠️  重要提示："
echo "   1. 请重新启动终端或重新登录以使系统限制生效"
echo "   2. 使用 ./start_sqld_optimized.sh 启动数据库服务器"
echo "   3. 使用 ./monitor_system.sh 监控系统状态"
echo "   4. 如遇问题，使用 ./cleanup_database.sh 清理数据库"
echo ""
echo "🚀 现在可以重新启动 sqld 服务器进行测试！" 