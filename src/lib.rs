use std::{
    fmt::Display,
    io::{self, Write},
    thread,
    time::Duration
};

pub trait SlowScanWrite {
    fn slow_scan_write<I>(&mut self, iter: I, delay: Duration) -> Result<(), io::Error>
    where
        I: IntoIterator,
        I::Item: Display;
}

impl<W: Write> SlowScanWrite for W {
    fn slow_scan_write<I>(&mut self, iter: I, delay: Duration) -> Result<(), io::Error>
    where
        I: IntoIterator,
        I::Item: Display
    {
        for item in iter {
            self.write_all(item.to_string().as_bytes())?;
            self.flush()?;
            thread::sleep(delay);
        }

        // 返回成功，如果所有项都成功写入
        Ok(())
    }
}
