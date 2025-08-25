use std::process::exit;
use std::time::Duration;

use clap::{Arg, ArgAction, Command};
use rust_i18n::t;
use slow_scan_print::SlowScanConfig;

/// 命令行参数解析结果
///
/// 用于存储从命令行解析得到的各种配置选项和参数
///
/// # 示例
/// ```
/// use std::time::Duration;
///
/// use slow_scan_print::Args;
///
/// let args = Args {
///     delay: Duration::from_millis(30),
///     line_mode: false,
///     hide_cursor: true,
///     files: vec!["example.txt".to_string()]
/// };
/// ```
///
/// ---
///
/// Command line arguments parsing result
///
/// Used to store various configuration options and parameters parsed from command line
#[derive(Debug, Clone)]
pub(crate) struct Args {
    pub slow_scan_config: SlowScanConfig,
    /// 是否启用行模式
    ///
    /// 如果为 `true`，则按行而不是按字符进行延迟输出
    ///
    /// ---
    ///
    /// Whether line mode is enabled
    ///
    /// If `true`, output will be delayed by line instead of by character
    pub line_mode: bool,
    /// 是否隐藏光标
    ///
    /// 如果为 `true`，将在输出过程中隐藏终端光标
    ///
    /// ---
    ///
    /// Whether to hide cursor
    ///
    /// If `true`, terminal cursor will be hidden during output
    pub hide_cursor: bool,
    /// 要处理的文件列表
    ///
    /// 支持多个文件输入，特殊值 "-" 表示从标准输入读取
    ///
    /// ---
    ///
    /// List of files to process
    ///
    /// Supports multiple file inputs, special value "-" indicates reading from standard input
    pub files: Vec<String>
}

impl Args {
    /// 从命令行参数创建新的 `Args` 实例
    ///
    /// 使用 `clap` 库解析命令行参数并转换为结构化数据
    ///
    /// # 返回值
    /// 返回解析后的 `Args` 实例
    ///
    /// # Panics
    /// - 当无法解析延迟时间字符串时会 panic
    /// - 当遇到意外的内部错误时会 panic
    ///
    /// ---
    ///
    /// Create a new `Args` instance from command line arguments
    ///
    /// Uses `clap` library to parse command line arguments and convert to structured data
    ///
    /// # Returns
    /// Returns parsed `Args` instance
    ///
    /// # Panics
    /// - Panics when delay time string cannot be parsed
    /// - Panics when unexpected internal errors occur
    pub fn new() -> Self {
        let args = [
            Arg::new("delay")
                .short('d')
                .long("delay")
                .value_name("TIME")
                .action(ArgAction::Set)
                .default_value("20ms")
                .help(t!("clap.delay.help").to_string())
                .long_help(t!("clap.delay.long_help").to_string()),
            Arg::new("full-width-delay")
                .short('f')
                .long("full-width-delay")
                .value_name("TIME")
                .action(ArgAction::Set)
                .help(t!("clap.full_width_delay").to_string()),
            Arg::new("control-char-delay")
                .short('c')
                .long("control-char-delay")
                .value_name("TIME")
                .action(ArgAction::Set)
                .help(t!("clap.control_char_delay").to_string()),
            Arg::new("tail-delay")
                .short('t')
                .long("tail-delay")
                .action(ArgAction::SetTrue)
                .help(t!("clap.tail_delay").to_string()),
            Arg::new("line-mode")
                .short('l')
                .long("line-mode")
                .action(ArgAction::SetTrue)
                .help(t!("clap.line_mode").to_string()),
            Arg::new("hide-cursor")
                .short('i') // "ignore cursor" 可能不是很准，但大概是就行
                .long("hide-cursor")
                .action(ArgAction::SetTrue)
                .help(t!("clap.hide_cursor").to_string()),
            Arg::new("files")
                .action(ArgAction::Append)
                .default_value("-")
                .help(t!("clap.files").to_string()),
            Arg::new("version")
                .short('v')
                .long("version")
                .action(ArgAction::Version)
                .help(t!("clap.version").to_string()),
            Arg::new("help")
                .short('h')
                .long("help")
                .action(ArgAction::Help)
                .help(t!("clap.help").to_string())
                .long_help(t!("clap.long_help").to_string())
        ];

        let matches = Command::new(env!("CARGO_PKG_NAME"))
            .disable_version_flag(true)
            .disable_help_flag(true)
            .version(env!("CARGO_PKG_VERSION"))
            .about(t!("clap.about").to_string())
            .author(env!("CARGO_PKG_AUTHORS"))
            .args(&args)
            .get_matches();

        let unreachable_msg = t!("error.unreachable");

        let delay = matches.get_one::<String>("delay").map_or_else(
            || unreachable!("{}", unreachable_msg),
            |it| {
                duration_str::parse_std(it).unwrap_or_else(|_| {
                    eprintln!("{}", t!("error.convert_string_to_duration"));
                    exit(1)
                })
            }
        );

        let full_width_delay =
            matches.get_one::<String>("full-width-delay").map_or_else(
                || delay * 2,
                |it| {
                    duration_str::parse_std(it).unwrap_or_else(|_| {
                        eprintln!("{}", t!("error.convert_string_to_duration"));
                        exit(1)
                    })
                }
            );

        let control_char_delay =
            matches.get_one::<String>("control-char-delay").map_or_else(
                || Duration::ZERO,
                |it| {
                    duration_str::parse_std(it).unwrap_or_else(|_| {
                        eprintln!("{}", t!("error.convert_string_to_duration"));
                        exit(1)
                    })
                }
            );

        let tail_delay = *matches
            .get_one::<bool>("tail-delay")
            .unwrap_or_else(|| unreachable!("{}", unreachable_msg));

        let slow_scan_config = *SlowScanConfig::default()
            .set_base_delay(delay)
            .set_full_width_delay(full_width_delay)
            .set_control_char_delay(control_char_delay)
            .set_tail_delay(tail_delay);

        let line_mode = *matches
            .get_one::<bool>("line-mode")
            .unwrap_or_else(|| unreachable!("{}", unreachable_msg));

        let hide_cursor = *matches
            .get_one::<bool>("hide-cursor")
            .unwrap_or_else(|| unreachable!("{}", unreachable_msg));

        let files = matches
            .get_many::<String>("files")
            .unwrap_or_else(|| unreachable!("{}", unreachable_msg))
            .map(|it| it.to_owned())
            .collect::<Vec<String>>();

        Self {
            slow_scan_config,
            line_mode,
            hide_cursor,
            files
        }
    }
}
