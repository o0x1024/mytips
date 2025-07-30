# ARM Linux AppImage 打包问题修复

## 问题描述

在 GitHub Actions 中进行 ARM Linux (aarch64) 交叉编译时，遇到 AppImage 打包失败的问题：

```
failed to bundle project: `Exec format error (os error 8)`
```

## 问题原因

`Exec format error (os error 8)` 错误是由于在 x86_64 架构的 GitHub Actions runner 上尝试执行 ARM64 架构的 AppImage 打包工具所致。AppImage 打包过程需要执行目标架构的二进制文件，但 x86_64 runner 无法直接执行 ARM64 二进制文件。

## 解决方案

### 方案一：跳过 ARM64 AppImage 打包（已采用）

修改 `.github/workflows/release.yml` 文件，对 ARM64 Linux 构建只生成 DEB 和 RPM 包，跳过 AppImage 打包：

```yaml
- name: Build the application
  if: matrix.platform != 'android'
  shell: bash
  run: |
    if [ "${{ matrix.platform }}" = "linux-aarch64" ]; then
      # ARM64 Linux 只构建 DEB 和 RPM，跳过 AppImage 避免交叉编译问题
      yarn tauri build --target ${{ matrix.rust_target }}
    else
      # 其他平台正常构建所有格式
      yarn tauri build --target ${{ matrix.rust_target }}
    fi
  env:
    TAURI_SIGNING_PRIVATE_KEY: ${{ secrets.TAURI_SIGNING_PRIVATE_KEY }}
    TAURI_SIGNING_PRIVATE_KEY_PASSWORD: ${{ secrets.TAURI_SIGNING_PRIVATE_KEY_PASSWORD }}
    # ARM64 Linux 只构建 DEB 和 RPM，跳过 AppImage
    TAURI_BUNDLE_TARGETS: ${{ matrix.platform == 'linux-aarch64' && 'deb,rpm' || 'all' }}
```

关键改动：
1. 使用 `TAURI_BUNDLE_TARGETS` 环境变量控制打包格式
2. ARM64 Linux 平台设置为 `deb,rpm`，其他平台保持 `all`
3. 移除了 `--bundles` 命令行参数，改用环境变量控制

### 方案二：使用 QEMU 支持（备选）

如果需要 ARM64 AppImage，可以安装 QEMU 支持：

```yaml
- name: Install QEMU for ARM64 emulation
  if: matrix.platform == 'linux-aarch64'
  run: |
    sudo apt-get update
    sudo apt-get install -y qemu-user-static
    sudo systemctl restart systemd-binfmt
```

### 方案三：使用原生 ARM64 Runner（理想方案）

使用原生 ARM64 GitHub Actions runner，但目前 GitHub 提供的 ARM64 runner 有限且成本较高。

## 验证结果

修复后，ARM64 Linux 构建应该能够成功生成：
- `mytips-{version}-arm64.deb`
- `mytips-{version}-arm64.rpm`
- 对应的签名文件 `.sig`

而不会尝试生成 AppImage 文件，从而避免 `Exec format error` 错误。

## 相关文件

- `.github/workflows/release.yml` - 主要修改文件
- `src-tauri/tauri.conf.json` - Tauri 配置文件
- `doc/arm-linux-build-fix.md` - 之前的 ARM Linux 构建修复文档

## 注意事项

1. ARM64 Linux 用户将无法获得 AppImage 格式的安装包，但可以使用 DEB 或 RPM 包
2. 如果未来需要 ARM64 AppImage，建议考虑使用原生 ARM64 runner 或 Docker 容器方案
3. 其他平台（x86_64 Linux、macOS、Windows）的 AppImage/DMG/NSIS 打包不受影响

## 更新时间

2024-12-19 - 初始修复方案实施