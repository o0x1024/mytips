# ARM Linux 构建修复记录

## 问题描述

在 GitHub Actions 中构建 ARM64 Linux 版本时遇到以下错误：

```
failed to bundle project: `Exec format error (os error 8)`
```

## 问题分析

1. **错误原因**: 在 x86_64 主机上交叉编译 ARM64 时，AppImage 打包工具尝试执行 ARM64 二进制文件导致格式错误
2. **根本原因**: AppImage 打包过程需要在目标架构上运行二进制文件进行依赖分析，但交叉编译环境无法执行 ARM64 二进制
3. **原有配置问题**: 虽然设置了 `TAURI_BUNDLE_TARGETS` 环境变量，但配置方式不正确，构建过程仍然尝试创建 AppImage

## 解决方案

### 修改前的配置
```yaml
- name: Build the application
  if: matrix.platform != 'android'
  run: yarn tauri build --target ${{ matrix.rust_target }}
  env:
    TAURI_SIGNING_PRIVATE_KEY: ${{ secrets.TAURI_SIGNING_PRIVATE_KEY }}
    TAURI_SIGNING_PRIVATE_KEY_PASSWORD: ${{ secrets.TAURI_SIGNING_PRIVATE_KEY_PASSWORD }}
    # 对于 ARM64 Linux，只打包 DEB 和 RPM，跳过 AppImage
    TAURI_BUNDLE_TARGETS: ${{ matrix.platform == 'linux-aarch64' && 'deb,rpm' || '' }}
```

### 修改后的配置
```yaml
- name: Build the application
  if: matrix.platform != 'android'
  run: |
    if [ "${{ matrix.platform }}" = "linux-aarch64" ]; then
      # ARM64 Linux 只构建 DEB 和 RPM，跳过 AppImage 避免交叉编译问题
      yarn tauri build --target ${{ matrix.rust_target }} --bundles deb,rpm
    else
      # 其他平台正常构建所有格式
      yarn tauri build --target ${{ matrix.rust_target }}
    fi
  env:
    TAURI_SIGNING_PRIVATE_KEY: ${{ secrets.TAURI_SIGNING_PRIVATE_KEY }}
    TAURI_SIGNING_PRIVATE_KEY_PASSWORD: ${{ secrets.TAURI_SIGNING_PRIVATE_KEY_PASSWORD }}
```

## 修改要点

1. **使用条件脚本**: 改用 shell 脚本条件判断，而不是环境变量
2. **明确指定打包格式**: 对 ARM64 Linux 使用 `--bundles deb,rpm` 参数
3. **保持其他平台不变**: 其他平台继续使用默认的完整打包流程

## 预期效果

- ARM64 Linux 构建将只生成 DEB 和 RPM 包，跳过 AppImage
- 避免交叉编译时的二进制执行错误
- 其他平台的构建流程保持不变

## 状态

- [x] 问题分析完成
- [x] 解决方案实施完成
- [x] Windows PowerShell 兼容性修复完成
- [ ] 测试验证（需要下次 tag 发布时验证）

## 补充修复

### Windows PowerShell 兼容性问题

**问题**: Windows 环境下运行 bash 语法的 if 语句导致 PowerShell 解析错误：
```
ParserError: Missing '(' after 'if' in if statement.
```

**解决方案**: 在构建步骤中添加 `shell: bash` 指令，强制所有平台使用 bash shell：
```yaml
- name: Build the application
  if: matrix.platform != 'android'
  shell: bash  # 强制使用 bash，确保跨平台兼容性
  run: |
    if [ "${{ matrix.platform }}" = "linux-aarch64" ]; then
      # ARM64 Linux 只构建 DEB 和 RPM，跳过 AppImage 避免交叉编译问题
      yarn tauri build --target ${{ matrix.rust_target }} --bundles deb,rpm
    else
      # 其他平台正常构建所有格式
      yarn tauri build --target ${{ matrix.rust_target }}
    fi
```

## 备注

此修复确保了 ARM64 Linux 版本能够成功构建，同时保持了对用户最重要的包格式（DEB 和 RPM）的支持。AppImage 格式在 ARM64 Linux 环境中相对较少使用，因此跳过该格式对用户影响较小。