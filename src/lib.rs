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
///     .with_base_delay(Duration::from_millis(30))
///     .with_full_width_delay(Duration::from_millis(60));
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
/// ---
///
/// A trait for slow scanning write operations to simulate character-by-character output.
///
/// Useful for creating typewriter effects or progressive character display.
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
    /// # 示例
    /// ```ignore
    /// use std::time::Duration;
    /// use slow_scan_print::{SlowScanConfig, SlowScanWrite};
    ///
    /// let config = SlowScanConfig::default()
    ///     .set_base_delay(Duration::from_millis(50));
    /// let mut writer = Vec::new();
    /// writer.slow_scan_write_by_chunks(["Hello", " World!"].iter(), config).unwrap();
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
    /// # Example
    /// ```ignore
    /// use std::time::Duration;
    /// use slow_scan_print::{SlowScanConfig, SlowScanWrite};
    ///
    /// let config = SlowScanConfig::default()
    ///     .with_base_delay(Duration::from_millis(50));
    /// let mut writer = Vec::new();
    /// writer.slow_scan_write_by_chunks(["Hello", " World!"].iter(), config).unwrap();
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

        while let Some(it) = iter.next() {
            self.write_all(it.as_ref())?;
            self.flush()?;

            if iter.peek().is_some() || config.tail_delay {
                #[cfg(not(feature = "unstable"))]
                sleep(config.base_delay);
                #[cfg(feature = "unstable")]
                sleep_until(Instant::now() + config.base_delay);
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
                        sleep_until(Instant::now() + config.full_width_delay)
                    }
                    // 控制字符（如 \n、\t 等）延迟
                    None => {
                        #[cfg(not(feature = "unstable"))]
                        sleep(config.control_char_delay);
                        #[cfg(feature = "unstable")]
                        sleep_until(Instant::now() + config.control_char_delay)
                    }
                    // 半宽字符（如英文字母、数字）
                    _ => {
                        #[cfg(not(feature = "unstable"))]
                        sleep(config.base_delay);
                        #[cfg(feature = "unstable")]
                        sleep_until(Instant::now() + config.base_delay)
                    }
                }
            }
        }

        Ok(())
    }
}
