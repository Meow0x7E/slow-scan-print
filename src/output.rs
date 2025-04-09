use std::io::{self, Write};
use std::thread;
use std::time::Duration;

/// 提供缓慢扫描式写入功能的 trait，模拟逐字符输出效果
///
/// 适用于需要模拟打字机效果或逐字符显示的场景
///
/// ---
///
/// Provides slow scanning write operations to simulate character-by-character output
///
/// Useful for creating typewriter effects or progressive character display
pub trait SlowScanWrite {
    /// 以指定延迟逐块写入数据
    ///
    /// # 参数
    /// - `iter`: 字节块迭代器，每个元素需实现 `AsRef<[u8]>`
    /// - `delay`: 每个数据块写入后的延迟时间（最后一个块之后不延迟）
    ///
    /// # 返回值
    /// - `Ok(())`: 所有数据成功写入
    /// - `Err(io::Error)`: 写入过程中发生 I/O 错误
    ///
    /// # 示例
    /// ```ignore
    /// use std::time::Duration;
    /// let mut writer = Vec::new();
    /// writer.slow_scan_write(["Hello", " World!"].iter(), Duration::from_millis(50)).unwrap();
    /// ```
    ///
    /// ---
    ///
    /// Write data chunk-by-chunk with specified delay
    ///
    /// # Arguments
    /// - `iter`: Iterator of byte chunks where each item implements `AsRef<[u8]>`
    /// - `delay`: Delay after each chunk (not applied after last chunk)
    ///
    /// # Returns
    /// - `Ok(())`: All data written successfully
    /// - `Err(io::Error)`: I/O error occurred during writing
    ///
    /// # Example
    /// ```ignore
    /// use std::time::Duration;
    /// let mut writer = Vec::new();
    /// writer.slow_scan_write(["Hello", " World!"].iter(), Duration::from_millis(50)).unwrap();
    /// ```
    fn slow_scan_write<I>(
        &mut self,
        iter: I,
        delay: Duration
    ) -> Result<(), io::Error>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>;

    /// 根据 Unicode 字符宽度进行延迟写入（需要启用 "unicode-width" 特性）
    ///
    /// # 参数
    /// - `iter`: 字符迭代器
    /// - `halt_width_delay`: 半角字符的基础延迟时间，全角字符使用双倍延迟
    ///
    /// # 注意
    /// - 依赖 `unicode-width` crate 的字符宽度计算
    /// - CJK 全角字符（如中文）会触发双倍延迟
    ///
    /// ---
    ///
    /// Write with width-based delays using Unicode character widths (requires "unicode-width" feature)
    ///
    /// # Arguments
    /// - `iter`: Iterator of characters
    /// - `halt_width_delay`: Base delay for half-width characters, full-width characters use double delay
    ///
    /// # Notes
    /// - Depends on `unicode-width` crate for width calculation
    /// - CJK full-width characters (e.g., Chinese) will trigger double delay
    fn slow_scan_write_use_chars<I>(
        &mut self,
        iter: I,
        halt_width_delay: Duration
    ) -> Result<(), io::Error>
    where
        I: Iterator<Item = char>;
}

impl<W: Write> SlowScanWrite for W {
    fn slow_scan_write<I>(
        &mut self,
        mut iter: I,
        delay: Duration
    ) -> Result<(), io::Error>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>
    {
        let mut current = match iter.next() {
            Some(c) => c,
            None => return Ok(())
        };

        loop {
            let next = iter.next();

            self.write_all(current.as_ref())?;
            self.flush()?;

            if next.is_some() {
                thread::sleep(delay)
            }

            current = match next {
                Some(c) => c,
                None => return Ok(())
            };
        }
    }

    fn slow_scan_write_use_chars<I>(
        &mut self,
        mut iter: I,
        halt_width_delay: Duration
    ) -> Result<(), io::Error>
    where
        I: Iterator<Item = char>
    {
        let full_width_delay = halt_width_delay * 2;
        let mut current = match iter.next() {
            Some(c) => c,
            None => return Ok(())
        };

        loop {
            let next = iter.next();

            let mut buf = [0; 4];
            current.encode_utf8(&mut buf);
            self.write_all(&buf)?;
            self.flush()?;

            if next.is_some() {
                match unicode_width::UnicodeWidthChar::width_cjk(current) {
                    Some(2) => thread::sleep(full_width_delay),
                    None => {}
                    _ => thread::sleep(halt_width_delay)
                }
            }

            current = match next {
                Some(c) => c,
                None => return Ok(())
            };
        }
    }
}
