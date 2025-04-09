use std::collections::VecDeque;
use std::io::{BufRead, BufReader};
use std::process::exit;
use std::time::Duration;

use chain_reader::*;
use clap::{Arg, ArgAction, Command, arg};
use console::Term;
use line_ending::LineEnding;
use once_cell::sync::Lazy;
use rust_i18n::{set_locale, t};
use slow_scan_print::input::InputSource;
use slow_scan_print::output::SlowScanWrite;
use utf8_chars::BufReadCharsExt;

rust_i18n::i18n!();

#[derive(Debug, Clone)]
struct Args {
    pub delay: Duration,
    pub line_mode: bool,
    pub hide_cursor: bool,
    pub files: Vec<String>
}

static ARGS: Lazy<Args> = Lazy::new(Args::new);
static STDOUT: Lazy<Term> = Lazy::new(Term::stdout);
static STDERR: Lazy<Term> = Lazy::new(Term::stderr);
static LINE_ENDING: Lazy<&str> =
    Lazy::new(|| LineEnding::from_current_platform().as_str());

fn main() {
    init_locale();
    setup_ctrlc_handle();

    if ARGS.hide_cursor {
        STDOUT.hide_cursor().unwrap()
    }

    slow_scan_print();

    if ARGS.hide_cursor {
        STDOUT.show_cursor().unwrap()
    }
}
impl Args {
    pub fn new() -> Self {
        let args = [
            arg!(delay: -d --delay <TIME>)
                .short_alias('t')
                .alias("time")
                .action(ArgAction::Set)
                .default_value("20ms")
                .help(t!("main.clap.delay.help").to_string())
                .long_help(t!("main.clap.delay.long_help").to_string()),
            Arg::new("line-mode")
                .short('l')
                .long("line-mode")
                .action(ArgAction::SetTrue)
                .help(t!("main.clap.line_mode.help").to_string()),
            Arg::new("hide-cursor")
                .short('c')
                .long("hide-cursor")
                .action(ArgAction::SetTrue)
                .help(t!("main.clap.hide_cursor.help").to_string()),
            Arg::new("files")
                .action(ArgAction::Append)
                .default_value("-")
                .help(t!("main.clap.files.help").to_string()),
            Arg::new("version")
                .short('v')
                .long("version")
                .action(ArgAction::Version)
                .help(t!("main.clap.version.help").to_string()),
            Arg::new("help")
                .short('h')
                .short_alias('?')
                .long("help")
                .action(ArgAction::Help)
                .help(t!("main.clap.help").to_string())
                .long_help(t!("main.clap.long_help").to_string())
        ];

        let matches = Command::new(env!("CARGO_PKG_NAME"))
            .disable_version_flag(true)
            .disable_help_flag(true)
            .version(env!("CARGO_PKG_VERSION"))
            .about(t!("main.clap.about").to_string())
            .author(env!("CARGO_PKG_AUTHORS"))
            .args(&args)
            .get_matches();

        let delay = match matches.get_one::<String>("delay") {
            Some(it) => duration_str::parse_std(it).unwrap_or_else(|it| {
                panic!(
                    "{}",
                    t!(
                        "main.panic.convert_string_to_duration_error",
                        error = it
                    )
                )
            }),
            None => unsafe { std::hint::unreachable_unchecked() }
        };

        let line_mode = match matches.get_one::<bool>("line-mode") {
            Some(it) => *it,
            None => unsafe { std::hint::unreachable_unchecked() }
        };

        let hide_cursor = match matches.get_one::<bool>("hide-cursor") {
            Some(it) => *it,
            None => unsafe { std::hint::unreachable_unchecked() }
        };

        let files = match matches.get_many::<String>("files") {
            Some(it) => it.map(|it| it.to_owned()).collect::<Vec<String>>(),
            None => unsafe { std::hint::unreachable_unchecked() }
        };

        Self {
            delay,
            line_mode,
            hide_cursor,
            files
        }
    }
}

#[inline]
fn init_locale() {
    if let Some(it) = sys_locale::get_locale() {
        set_locale(it.as_str());
    };
}

#[inline]
fn setup_ctrlc_handle() {
    ctrlc::set_handler(move || {
        STDOUT.show_cursor().unwrap();
        exit(1)
    })
    .unwrap_or_else(|it| {
        let _ = STDERR.write_line(
            t!("main.error.set_ctrlc_handle_error", error = it).as_ref()
        );
    });
}

#[inline]
fn slow_scan_print() {
    let mut readers = VecDeque::with_capacity(ARGS.files.len());

    for it in ARGS.files.iter() {
        match InputSource::open(it) {
            Ok(it) => readers.push_back(it),
            Err(it) => {
                let _ = STDERR.write_line(it.to_string().as_str());
            }
        }
    }

    let mut reader =
        BufReader::new(ChainReader::new(readers, |_| ErrorAction::Skip));

    if ARGS.line_mode {
        let iter = reader.lines().map(|it| {
            let mut it = it.unwrap_or_else(|_| String::new());
            it.push_str(&LINE_ENDING);
            it
        });

        STDOUT.clone().slow_scan_write(iter, ARGS.delay)
    } else {
        let iter = reader.chars().map(|it| it.unwrap());
        STDOUT.clone().slow_scan_write_use_chars(iter, ARGS.delay)
    }
    .unwrap_or_else(|it| {
        panic!("{}\n{}", t!("main.panic.io_error_on_slow_scan_print"), it)
    });
}
