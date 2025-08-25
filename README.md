# slow-scan-print

[![Crates.io](https://img.shields.io/crates/v/slow-scan-print)](https://crates.io/crates/slow-scan-print)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-2024-orange)](https://www.rust-lang.org/)

一个以固定时间间隔逐字符或逐行打印文本的命令行工具，其名称灵感来源于慢扫描电视（SSTV）。

## 功能特点

- 支持逐字符或逐行打印模式
- 可配置的打印延迟时间（支持全角字符、控制字符单独设置）
- 多文件输入支持（包括标准输入）
- 终端光标隐藏与恢复
- 国际化支持（中英文）
- 优雅处理 Ctrl+C 中断
- 轻量级、无依赖（仅标准库与少量稳定依赖）

## 安装

### 作为项目依赖

在依赖配置中禁用默认 features 即可排除 bin 所需的依赖

```toml
slow-scan-print = { version = "2.0.0", default-features = false }
```

### 从 Crates.io 安装

```bash
cargo install slow-scan-print
```

### 从源码构建

```bash
git clone https://github.com/Meow0x7E/slow-scan-print.git
cd slow-scan-print
cargo build --release
```

编译后的二进制文件位于 `target/release/slow-scan-print`。

## 使用方法

### 基本使用

```bash
# 从标准输入读取并逐字符打印
echo "Hello, World!" | slow-scan-print

# 从文件读取并逐行打印
slow-scan-print -l file.txt

# 同时读取多个文件
slow-scan-print file1.txt file2.txt
```

### 命令行选项

| 选项                   | 缩写 | 说明                                |
| ---------------------- | ---- | ----------------------------------- |
| `--delay`              | `-d` | 设置基础延迟时间（默认：20ms）      |
| `--full-width-delay`   | `-f` | 设置全角字符延迟（默认：2 × delay） |
| `--control-char-delay` | `-c` | 设置控制字符延迟（默认：0）         |
| `--tail-delay`         | `-t` | 是否在最后一个字符后也延迟          |
| `--line-mode`          | `-l` | 启用逐行模式                        |
| `--hide-cursor`        | `-i` | 隐藏终端光标                        |
| `--help`               | `-h` | 显示帮助信息                        |
| `--version`            | `-v` | 显示版本信息                        |

### 延迟时间格式

支持以下时间单位：

- `y`, `year`, `年` （年）
- `mon`, `month`, `月` （月）
- `w`, `week`, `周` （周）
- `d`, `day`, `日` （日）
- `h`, `hr`, `hour`, `小时` （小时）
- `m`, `min`, `minute`, `分钟` （分钟）
- `s`, `sec`, `second`, `秒` （秒）
- `ms`, `msec`, `millisecond`, `毫秒` （毫秒）
- `µs`, `µsec`, `microsecond`, `微秒` （微秒）
- `ns`, `nsec`, `nanosecond`, `纳秒` （纳秒）

支持简单算术表达式，例如：

```bash
slow-scan-print -d "100ms * 2" file.txt      # 200ms 延迟
slow-scan-print -d "1.5h + 30m" file.txt     # 2小时延迟
```

## 示例

### 逐字符打印中文文本

```bash
slow-scan-print -d 50ms --full-width-delay 100ms chinese.txt
```

### 逐行打印并隐藏光标

```bash
slow-scan-print -l -c log.txt
```

### 从管道读取并打印

```bash
cat story.txt | slow-scan-print -d 30ms
```

## 贡献

欢迎提交 Issue 和 Pull Request！  
请确保代码风格与现有代码一致，并通过 `cargo test` 和 `cargo clippy` 检查。

## 许可证

本项目基于 [MIT 许可证](LICENSE) 发布。

## 使用了deepseek

- 编写 README
- 编写翻译
- 编写注释
- 提供编写帮助
