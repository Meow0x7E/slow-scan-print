use std::{
    fmt::Display,
    io::{self, Write},
    thread,
    time::Duration
};

/// 以慢速扫描的方式写入数据。
///
/// 该函数接收一个迭代器、一个时间间隔（`Duration`）和一个可写入的引用（`&mut W`），
/// 并逐个从迭代器中取出元素，将它们转换为字符串并写入到提供的写入器（`writer`）中。
/// 在每次写入后，都会刷新写入器并暂停指定的时间间隔（`dur`）。
///
/// # 参数
///
/// - `iter`: 要写入的元素的迭代器。
/// - `dur`: 每次写入后暂停的时间间隔。
/// - `writer`: 要写入的目标的可写引用。
///
/// # 类型参数
///
/// - `I`: 实现`IntoIterator`特质的类型，其元素可以转换为字符串。
/// - `W`: 实现`Write`特质的类型，用于写入数据。
///
/// # 返回值
///
/// 如果写入过程中没有发生错误，则返回`Ok(())`；否则返回`Err(io::Error)`。
///
/// # 示例
///
/// ```rust
/// use std::{io, thread, time, fs::File};
///
/// let mut stdout = io::stdout();
///
/// slow_scan_write("Hello World!".chars(), time::Duration::from_secs(1), &mut stdout).unwrap();
/// ```
///
/// 在上面的示例中，我们将字符串"Hello World!"以每秒一个的速度写入到 stdout
pub fn slow_scan_write<I, W>(iter: I, dur: Duration, writer: &mut W) -> Result<(), io::Error>
where
    I: IntoIterator,
    I::Item: Display,
    W: Write {
    for item in iter {
        writer.write_all(item.to_string().as_bytes())?;
        writer.flush()?;
        thread::sleep(dur);
    }
    Ok(())
}

#[macro_export]
macro_rules! print_chars {
    ($display:expr) => {{
        let dur = Duration::from_millis(20);
        let mut stdout = io::stdout();
        slow_scan_write(
            $display.lines().flat_map(|l| {
                let mut v: Vec<char> = format!("{}", l).chars().collect();
                v.push('\n');
                v
            }),
            dur,
            &mut stdout
        )
    }};
    ($display:expr, $dur:expr) => {{
        let mut stdout = io::stdout();
        slow_scan_write(
            $display.lines().flat_map(|l| {
                let mut v: Vec<char> = format!("{}", l).chars().collect();
                v.push('\n');
                v
            }),
            $dur,
            &mut stdout
        )
    }};
    ($display:expr, $dur:expr, $writer:expr) => {{
        slow_scan_write(
            $display.lines().flat_map(|l| {
                let mut v: Vec<char> = format!("{}", l).chars().collect();
                v.push('\n');
                v
            }),
            $dur,
            $writer
        )
    }};
}

#[macro_export]
macro_rules! print_chars_with_reader {
    ($reader:expr) => {{
        let dur = Duration::from_millis(20);
        let mut stdout = io::stdout();
        slow_scan_write(
            std::io::BufRead::lines($reader).flat_map(|l| {
                let mut v: Vec<char> = format!("{}", l.unwrap_or(String::from(""))).chars(String::from("")).collect();
                v.push('\n');
                v
            }),
            dur,
            &mut stdout
        )
    }};
    ($reader:expr, $dur:expr) => {{
        let mut stdout = io::stdout();
        slow_scan_write(
            std::io::BufRead::lines($reader).flat_map(|l| {
                let mut v: Vec<char> = format!("{}", l.unwrap_or(String::from(""))).chars(String::from("")).collect();
                v.push('\n');
                v
            }),
            $dur,
            &mut stdout
        )
    }};
    ($reader:expr, $dur:expr, $writer:expr) => {{
        slow_scan_write(
            std::io::BufRead::lines($reader).flat_map(|l| {
                let mut v: Vec<char> = format!("{}", l.unwrap_or(String::from(""))).chars().collect();
                v.push('\n');
                v
            }),
            $dur,
            $writer
        )
    }};
}

#[macro_export]
macro_rules! print_lines {
    ($display:expr) => {{
        let dur = Duration::from_millis(20);
        let mut stdout = io::stdout();
        slow_scan_write($display.lines().map(|l| format!("{}\n", l)), dur, &mut stdout)
    }};
    ($display:expr, $dur:expr) => {{
        let mut stdout = io::stdout();
        slow_scan_write($display.lines().map(|l| format!("{}\n", l)), $dur, &mut stdout)
    }};
    ($display:expr, $dur:expr, $writer:expr) => {{
        slow_scan_write($display.lines().map(|l| format!("{}\n", l)), $dur, $writer)
    }};
}

#[macro_export]
macro_rules! print_lines_with_reader {
    ($reader:expr) => {{
        let dur = Duration::from_millis(20);
        let mut stdout = io::stdout();
        slow_scan_write(
            std::io::BufRead::lines($reader).map(|l| format!("{}\n", l.unwrap_or(String::from("")))),
            dur,
            &mut stdout
        )
    }};
    ($reader:expr, $dur:expr) => {{
        let mut stdout = io::stdout();
        slow_scan_write(
            std::io::BufRead::lines($reader).map(|l| format!("{}\n", l.unwrap_or(String::from("")))),
            $dur,
            &mut stdout()
        )
    }};
    ($reader:expr, $dur:expr, $writer:expr) => {{
        slow_scan_write(
            std::io::BufRead::lines($reader).map(|l| format!("{}\n", l.unwrap_or(String::from("")))),
            $dur,
            $writer
        )
    }};
}
