//! Provides slow scanning writing capability for writers
//!
//! This module extends the standard [`Write`] trait through the [`SlowScanWrite`] trait,
//! adding chunk-by-chunk writing with delays, useful for simulating typing effects,
//! network scanning scenarios, etc.

use std::{
    io::{self, Write},
    thread,
    time::Duration
};

/// Trait adding slow scanning writing capability to writers
///
/// Types implementing this trait can write data chunk by chunk with specified delays
/// between writes, suitable for scenarios requiring progressive writing effects.
///
/// # Examples
/// ```
/// use std::{io, time::Duration};
/// use slow_scan_print::SlowScanWrite;
///
/// let mut writer = io::stdout();
/// let chunks = vec!["Hello", " ", "world", "!"];
///
/// // Write chunks with 100ms intervals
/// writer.slow_scan_write(chunks.into_iter(), Duration::from_millis(100))
///     .expect("Write successful");
/// ```
pub trait SlowScanWrite {
    /// Writes iterator contents with specified delays
    ///
    /// # Parameters
    /// - `iter`: An iterator yielding items convertible to byte slices ([`AsRef<[u8]>`])
    /// - `delay`: Pause duration after each write
    ///
    /// # Returns
    /// - `Ok(())`: All content written successfully
    /// - `Err(e)`: Returns IO error if encountered, immediately aborting the process
    ///
    /// # Notes
    /// - Uses [`write_all`] to ensure full write of each chunk
    /// - Calls [`flush`] after each write for immediate effect
    /// - Uses [`thread::sleep`] for delays, blocking the current thread
    ///
    /// # Examples
    /// Simulate typing effect:
    /// ```
    /// # use std::{io, time::Duration};
    /// # use slow_scan_print::SlowScanWrite;
    /// #
    /// let mut buffer = Vec::new();
    /// let words = ["Type", " ", "slowly", " ", "letter", " ", "by", " ", "letter"];
    ///
    /// buffer.slow_scan_write(words.into_iter(), Duration::from_millis(50))
    ///     .unwrap();
    ///
    /// assert_eq!(String::from_utf8(buffer).unwrap(), "Type slowly letter by letter");
    /// ```
    fn slow_scan_write<I>(&mut self, iter: I, delay: Duration) -> Result<(), io::Error>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>;
}

impl<W: Write> SlowScanWrite for W {
    fn slow_scan_write<I>(&mut self, iter: I, delay: Duration) -> Result<(), io::Error>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>
    {
        for item in iter {
            self.write_all(item.as_ref())?;
            self.flush()?; // 确保每次写入立即生效
            thread::sleep(delay);
        }

        Ok(())
    }
}
