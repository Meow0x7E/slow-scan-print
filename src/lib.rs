#![cfg_attr(feature = "unstable", feature(thread_sleep_until))]

use std::io::{self, Write};
#[cfg(not(feature = "unstable"))]
use std::thread::sleep;
use std::time::Duration;
#[cfg(feature = "unstable")]
use std::{thread::sleep_until, time::Instant};

use getset::{Getters, Setters};

/// 配置慢速扫描输出的参数
///
/// 用于控制字符输出时的延迟行为，支持根据不同字符类型设置不同的延迟时间
///
/// # 示例
/// ```
/// use std::time::Duration;
///
/// use slow_scan_print::SlowScanConfig;
///
/// let config = SlowScanConfig::default()
///     .set_base_delay(Duration::from_millis(30))
///     .set_full_width_delay(Duration::from_millis(60));
/// ```
///
/// ---
///
/// Configuration parameters for slow scan output
///
/// Controls delay behavior during character output, with support for different
/// delay times based on character type
#[derive(Debug, Clone, Copy, Getters, Setters)]
pub struct SlowScanConfig {
    /// 半角字符的基础延迟时间
    ///
    /// 适用于大多数拉丁字母、数字和符号
    ///
    /// ---
    ///
    /// Base delay for half-width characters
    ///
    /// Applies to most Latin letters, numbers and symbols
    #[getset(get = "pub", set = "pub")]
    base_delay: Duration,

    /// 全角字符的延迟时间
    ///
    /// 适用于中文字符、日文字符、韩文字符等全角字符
    ///
    /// ---
    ///
    /// Delay for full-width characters
    ///
    /// Applies to Chinese, Japanese, Korean and other full-width characters
    #[getset(get = "pub", set = "pub")]
    full_width_delay: Duration,

    /// 控制字符的延迟时间
    ///
    /// 适用于换行符(`\n`)、制表符(`\t`)等控制字符
    ///
    /// ---
    ///
    /// Delay for control characters
    ///
    /// Applies to control characters like newline(`\n`), tab(`\t`) etc.
    #[getset(get = "pub", set = "pub")]
    control_char_delay: Duration,

    /// 是否在输出最后一个字符后也添加延迟
    ///
    /// 如果设置为 `true`，即使在输出最后一个字符后也会应用延迟
    /// 如果设置为 `false`，最后一个字符后不添加延迟
    ///
    /// ---
    ///
    /// Whether to add delay after the last character output
    ///
    /// If set to `true`, delay will be applied even after the last character
    /// If set to `false`, no delay is added after the last character
    #[getset(get = "pub", set = "pub")]
    tail_delay: bool
}

impl SlowScanConfig {
    /// 根据预期的总持续时间和块数量自动计算并设置基础延迟
    ///
    /// 这个方法适用于需要精确控制整体输出时长的场景，通过预期的总时长和
    /// 块数量来自动计算每个块之间的基础延迟时间。
    ///
    /// # 参数
    /// - `expectation`: 期望的总输出持续时间
    /// - `chunk_count`: 输出的块数量（字符块或字节块）
    ///
    /// # 计算规则
    /// - 如果 `tail_delay` 为 `true`，延迟次数等于块数量
    /// - 如果 `tail_delay` 为 `false`，延迟次数等于 `块数量 - 1`
    /// - 基础延迟 = 期望总时长 / 延迟次数
    /// - 如果块数量为 0，基础延迟设置为 `Duration::ZERO`
    ///
    /// # 示例
    /// ```
    /// use std::time::Duration;
    ///
    /// use slow_scan_print::SlowScanConfig;
    ///
    /// // 希望在 1 秒内输出 10 个块
    /// let mut config = SlowScanConfig::default();
    /// config
    ///     .set_tail_delay(true)
    ///     .set_base_delay_from_expected_total_duration(
    ///         Duration::from_secs(1),
    ///         10
    ///     );
    ///
    /// assert_eq!(*config.base_delay(), Duration::from_millis(100));
    ///
    /// config
    ///     .set_tail_delay(false)
    ///     .set_base_delay_from_expected_total_duration(
    ///         Duration::from_secs(1),
    ///         11
    ///     );
    /// assert_eq!(*config.base_delay(), Duration::from_millis(100));
    /// ```
    ///
    /// # 注意
    /// - 这个方法不会修改 `full_width_delay` 和 `control_char_delay` 的设置
    /// - 实际总时长可能因系统调度和性能而有微小偏差
    /// - 如果 `chunk_count` 为 0 或 1（且 `tail_delay` 为 `false`），基础延迟会被设置为 0
    ///
    /// ---
    ///
    /// Automatically calculates and sets base delay based on expected total duration and chunk count
    ///
    /// This method is useful for scenarios requiring precise control over total output duration,
    /// automatically calculating the base delay between chunks based on expected total time
    /// and number of chunks.
    ///
    /// # Arguments
    /// - `expectation`: Expected total output duration
    /// - `chunk_count`: Number of output chunks (character chunks or byte chunks)
    ///
    /// # Calculation Rules
    /// - If `tail_delay` is `true`, number of delays equals chunk count
    /// - If `tail_delay` is `false`, number of delays equals `chunk_count - 1`
    /// - Base delay = Expected total duration / Number of delays
    /// - If chunk count is 0, base delay is set to `Duration::ZERO`
    ///
    /// # Example
    /// ```
    /// use std::time::Duration;
    ///
    /// use slow_scan_print::SlowScanConfig;
    ///
    /// // Want to output 10 chunks within 1 second
    /// let mut config = SlowScanConfig::default();
    /// config
    ///     .set_tail_delay(true)
    ///     .set_base_delay_from_expected_total_duration(
    ///         Duration::from_secs(1),
    ///         10
    ///     );
    ///
    /// assert_eq!(*config.base_delay(), Duration::from_millis(100));
    ///
    /// config
    ///     .set_tail_delay(false)
    ///     .set_base_delay_from_expected_total_duration(
    ///         Duration::from_secs(1),
    ///         11
    ///     );
    /// assert_eq!(*config.base_delay(), Duration::from_millis(100));
    /// ```
    ///
    /// # Notes
    /// - This method does not modify `full_width_delay` and `control_char_delay` settings
    /// - Actual total duration may have slight deviations due to system scheduling and performance
    /// - If `chunk_count` is 0 or 1 (and `tail_delay` is `false`), base delay will be set to 0
    pub fn set_base_delay_from_expected_total_duration(
        &mut self,
        expectation: Duration,
        chunk_count: u32
    ) -> &mut Self {
        let delay_count = if self.tail_delay {
            chunk_count
        } else {
            chunk_count.saturating_sub(1)
        };

        if delay_count > 0 {
            self.set_base_delay(expectation / delay_count)
        } else {
            self.set_base_delay(Duration::ZERO)
        }
    }
}

impl Default for SlowScanConfig {
    fn default() -> Self {
        Self {
            base_delay: Duration::from_millis(20),
            full_width_delay: Duration::from_millis(40),
            control_char_delay: Duration::ZERO,
            tail_delay: false
        }
    }
}

/// 提供缓慢扫描式写入功能的 trait，模拟逐字符输出效果
///
/// 适用于需要模拟打字机效果或逐字符显示的场景
///
/// # 延迟精度说明
/// - 默认情况下使用 `std::thread::sleep`，延迟精度受系统调度影响
/// - 启用 `unstable` 特性后使用 `std::thread::sleep_until`，提供更精准的延迟控制
///   避免因执行时间累积导致的延迟误差，同时不会带来明显的性能损失
///
/// ---
///
/// A trait for slow scanning write operations to simulate character-by-character output.
///
/// Useful for creating typewriter effects or progressive character display.
///
/// # Delay Precision Notes
/// - By default uses `std::thread::sleep` with precision affected by system scheduling
/// - When `unstable` feature is enabled, uses `std::thread::sleep_until` for more precise
///   delay control, avoiding cumulative timing errors from execution time, without
///   significant performance impact
pub trait SlowScanWrite {
    /// 以指定配置逐块写入数据
    ///
    /// # 参数
    /// - `iter`: 字节块迭代器，每个元素需实现 `AsRef<[u8]>`
    /// - `config`: 慢速扫描配置参数
    ///
    /// # 返回值
    /// - `Ok(())`: 所有数据成功写入
    /// - `Err(io::Error)`: 写入过程中发生 I/O 错误
    ///
    /// # 延迟精度
    /// - 默认实现使用 `std::thread::sleep`，延迟精度受系统调度影响
    /// - 启用 `unstable` 特性后使用 `std::thread::sleep_until`，提供更精准的延迟控制
    ///   避免因执行时间累积导致的延迟误差，同时不会带来明显的性能损失
    ///
    /// # 性能说明
    /// 基准测试表明，使用 `unstable` 特性不会带来明显的性能损失，同时提供更精确的定时控制
    ///
    /// # 示例
    /// ```
    /// use std::time::Duration;
    ///
    /// use slow_scan_print::{SlowScanConfig, SlowScanWrite};
    ///
    /// let mut config = SlowScanConfig::default();
    /// config.set_base_delay(Duration::from_millis(10));
    ///
    /// let mut writer = Vec::new();
    /// writer
    ///     .slow_scan_write_by_chunks(["Hello", " World!"].iter(), config)
    ///     .unwrap();
    /// ```
    ///
    /// ---
    ///
    /// Write data chunk-by-chunk with specified configuration.
    ///
    /// # Arguments
    /// - `iter`: Iterator of byte chunks where each item implements `AsRef<[u8]>`
    /// - `config`: Slow scan configuration parameters
    ///
    /// # Returns
    /// - `Ok(())`: All data written successfully
    /// - `Err(io::Error)`: I/O error occurred during writing
    ///
    /// # Delay Precision
    /// - Default implementation uses `std::thread::sleep` with precision affected by system scheduling
    /// - When `unstable` feature is enabled, uses `std::thread::sleep_until` for more precise
    ///   delay control, avoiding cumulative timing errors from execution time, without
    ///   significant performance impact
    ///
    /// # Performance Note
    /// Benchmarking shows that using the `unstable` feature does not incur significant
    /// performance penalty while providing more precise timing control
    ///
    /// # Example
    /// ```
    /// use std::time::Duration;
    ///
    /// use slow_scan_print::{SlowScanConfig, SlowScanWrite};
    ///
    /// let mut config = SlowScanConfig::default();
    /// config.set_base_delay(Duration::from_millis(10));
    ///
    /// let mut writer = Vec::new();
    /// writer
    ///     .slow_scan_write_by_chunks(["Hello", " World!"].iter(), config)
    ///     .unwrap();
    /// ```
    fn slow_scan_write_by_chunks<I>(
        &mut self,
        iter: I,
        config: SlowScanConfig
    ) -> Result<(), io::Error>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>;

    /// 根据 Unicode 字符宽度和配置进行延迟写入
    ///
    /// # 参数
    /// - `iter`: 字符迭代器
    /// - `config`: 慢速扫描配置参数
    ///
    /// # 返回值
    /// - `Ok(())`: 所有数据成功写入
    /// - `Err(io::Error)`: 写入过程中发生 I/O 错误
    ///
    /// # 注意
    /// - CJK 全角字符（如中文）会使用 `full_width_delay` 配置的延迟时间
    /// - 控制字符会使用 `control_char_delay` 配置的延迟时间
    /// - 半角字符会使用 `base_delay` 配置的延迟时间
    ///
    /// # 延迟精度
    /// - 默认实现使用 `std::thread::sleep`，延迟精度受系统调度影响
    /// - 启用 `unstable` 特性后使用 `std::thread::sleep_until`，提供更精准的延迟控制
    ///   避免因执行时间累积导致的延迟误差，同时不会带来明显的性能损失
    ///
    /// # 性能说明
    /// 基准测试表明，使用 `unstable` 特性不会带来明显的性能损失，同时提供更精确的定时控制
    ///
    /// ---
    ///
    /// Write with width-based delays using Unicode character widths and configuration.
    ///
    /// # Arguments
    /// - `iter`: Iterator of characters
    /// - `config`: Slow scan configuration parameters
    ///
    /// # Returns
    /// - `Ok(())`: All data written successfully
    /// - `Err(io::Error)`: I/O error occurred during writing
    ///
    /// # Notes
    /// - CJK full-width characters (e.g., Chinese) will use the `full_width_delay` configuration
    /// - Control characters will use the `control_char_delay` configuration
    /// - Half-width characters will use the `base_delay` configuration
    ///
    /// # Delay Precision
    /// - Default implementation uses `std::thread::sleep` with precision affected by system scheduling
    /// - When `unstable` feature is enabled, uses `std::thread::sleep_until` for more precise
    ///   delay control, avoiding cumulative timing errors from execution time, without
    ///   significant performance impact
    ///
    /// # Performance Note
    /// Benchmarking shows that using the `unstable` feature does not incur significant
    /// performance penalty while providing more precise timing control
    fn slow_scan_write_by_chars<I>(
        &mut self,
        iter: I,
        config: SlowScanConfig
    ) -> Result<(), io::Error>
    where
        I: Iterator<Item = char>;
}

impl<W: Write> SlowScanWrite for W {
    fn slow_scan_write_by_chunks<I>(
        &mut self,
        iter: I,
        config: SlowScanConfig
    ) -> Result<(), io::Error>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>
    {
        let mut iter = iter.peekable();
        #[cfg(feature = "unstable")]
        let mut now = Instant::now();

        while let Some(it) = iter.next() {
            self.write_all(it.as_ref())?;
            self.flush()?;

            if iter.peek().is_some() || config.tail_delay {
                #[cfg(not(feature = "unstable"))]
                sleep(config.base_delay);
                #[cfg(feature = "unstable")]
                {
                    now += config.base_delay;
                    sleep_until(now);
                }
            }
        }

        Ok(())
    }

    fn slow_scan_write_by_chars<I>(
        &mut self,
        iter: I,
        config: SlowScanConfig
    ) -> Result<(), io::Error>
    where
        I: Iterator<Item = char>
    {
        let mut iter = iter.peekable();
        let mut buf = [0; 4];
        #[cfg(feature = "unstable")]
        let mut now = Instant::now();

        while let Some(it) = iter.next() {
            self.write_all(it.encode_utf8(&mut buf).as_ref())?;
            self.flush()?;

            if iter.peek().is_some() || config.tail_delay {
                match unicode_width::UnicodeWidthChar::width_cjk(it) {
                    // 全宽字符（如中文字符）
                    Some(2) => {
                        #[cfg(not(feature = "unstable"))]
                        sleep(config.full_width_delay);
                        #[cfg(feature = "unstable")]
                        {
                            now += config.full_width_delay;
                            sleep_until(now);
                        }
                    }
                    // 控制字符（如 \n、\t 等）延迟
                    None => {
                        #[cfg(not(feature = "unstable"))]
                        sleep(config.control_char_delay);
                        #[cfg(feature = "unstable")]
                        {
                            now += config.control_char_delay;
                            sleep_until(now);
                        }
                    }
                    // 半宽字符（如英文字母、数字）
                    _ => {
                        #[cfg(not(feature = "unstable"))]
                        sleep(config.base_delay);
                        #[cfg(feature = "unstable")]
                        {
                            now += config.base_delay;
                            sleep_until(now);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
