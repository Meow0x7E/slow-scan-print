use std::borrow::Cow;
use std::fs::File;
use std::{error, fmt, io};

use rust_i18n::t;

/// 表示输入源的类型，支持标准输入、文件和空输入
///
/// 提供从 URI 字符串创建输入源的能力
///
/// ---
///
/// Represents input sources including standard input, files and empty input
///
/// Provides capabilities to create from URI strings and concatenate multiple sources
#[derive(Debug)]
pub enum InputSource {
    /// 标准输入源
    ///
    /// ---
    ///
    /// Standard input source
    Stdin(io::Stdin),
    /// 文件输入源
    ///
    /// ---
    ///
    /// File input source
    File(File),
    /// 空输入源（读取时返回 EOF）
    ///
    /// ---
    ///
    /// Empty input source (returns EOF when read)
    Empty
}

impl InputSource {
    /// 通过 URI 字符串打开输入源
    ///
    /// # 参数
    /// - `uri`: 输入源标识符（空字符串表示错误，"-" 表示标准输入）
    ///
    /// # 错误
    /// 返回 [`Error`] 类型错误，包含无法打开资源的原因
    ///
    /// ---
    ///
    /// Open input source by URI string
    ///
    /// # Arguments
    /// - `uri`: Input source identifier (empty string for error, "-" for stdin)
    ///
    /// # Errors
    /// Returns [`Error`] containing reasons when failing to open resource
    pub fn open(uri: &str) -> Result<Self, Error> {
        if uri.is_empty() {
            return Err(Error {
                kind: ErrorKind::UriIsEmpty,
                uri: Cow::Borrowed(uri),
                source: None
            });
        }

        if uri == "-" {
            return Ok(Self::Stdin(io::stdin()));
        }

        File::open(uri).map(Self::File).map_err(|it| Error {
            kind: ErrorKind::CannotOpenUri,
            uri: Cow::Borrowed(uri),
            source: Some(it)
        })
    }
}

impl io::Read for InputSource {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            InputSource::Stdin(it) => it.read(buf),
            InputSource::File(it) => it.read(buf),
            InputSource::Empty => Ok(0)
        }
    }
}

/// 输入源错误类型
///
/// ---
///
/// Input source error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// URI 为空字符串
    ///
    /// ---
    ///
    /// URI is empty string
    UriIsEmpty,
    /// 无法打开指定 URI
    ///
    /// ---
    ///
    /// Failed to open specified URI
    CannotOpenUri
}

/// 输入系统错误封装
///
/// 包含错误类型、相关 URI 和底层错误原因
///
/// ---
///
/// Input system error wrapper
///
/// Contains error type, related URI and underlying error cause
#[derive(Debug)]
pub struct Error<'a> {
    kind: ErrorKind,
    uri: Cow<'a, str>,
    source: Option<io::Error>
}

impl Error<'_> {
    /// 获取错误类型
    ///
    /// ---
    ///
    /// Get error kind
    pub fn kind(&self) -> ErrorKind { self.kind }

    /// 获取相关 URI 引用
    ///
    /// ---
    ///
    /// Get related URI reference
    pub fn uri(&self) -> &str { self.uri.as_ref() }

    /// 将借用数据转换为自有数据
    ///
    /// ---
    ///
    /// Convert borrowed data to owned
    pub fn into_owned(self) -> Error<'static> {
        Error {
            kind: self.kind,
            uri: Cow::Owned(self.uri.into_owned()),
            source: self.source
        }
    }
}

impl fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::CannotOpenUri => {
                let source = self
                    .source
                    .as_ref()
                    .map_or_else(String::new, |it| it.to_string());

                #[rustfmt::skip]
                write!(f, "{}", t!(
                    "input.ErrorKind.CannotOpenUri",
                    uri = self.uri,
                    source = source
                ))?;
                Ok(())
            }
            ErrorKind::UriIsEmpty => {
                write!(f, "{}", t!("input.ErrorKind.UriIsEmpty"))
            }
        }
    }
}

impl error::Error for Error<'_> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.source.as_ref().map(|e| e as _)
    }
}
