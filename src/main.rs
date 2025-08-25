use std::collections::VecDeque;
use std::io::{BufRead, BufReader};
use std::process::exit;

use chain_reader::*;
use console::Term;
use line_ending::LineEnding;
use once_cell::sync::Lazy;
use rust_i18n::{set_locale, t};
use slow_scan_print::SlowScanWrite;
use utf8_chars::BufReadCharsExt;

use crate::args::Args;
use crate::input::InputSource;

rust_i18n::i18n!();

mod args;
mod input;

static ARGS: Lazy<Args> = Lazy::new(Args::new);
static STDOUT: Lazy<Term> = Lazy::new(Term::stdout);
static LINE_ENDING: Lazy<&str> =
    Lazy::new(|| LineEnding::from_current_platform().as_str());

fn main() {
    init_locale();
    setup_ctrlc_handle();

    if ARGS.hide_cursor {
        let _ = STDOUT.hide_cursor();
    }

    slow_scan_print();

    if ARGS.hide_cursor {
        let _ = STDOUT.show_cursor();
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
        if ARGS.hide_cursor {
            let _ = STDOUT.show_cursor();
        }
        exit(1)
    })
    .unwrap_or_else(|it| {
        eprintln!("{}", t!("error.set_ctrlc_handle_error", error = it));
    });
}

#[inline]
fn slow_scan_print() {
    let mut readers = VecDeque::with_capacity(ARGS.files.len());

    for it in ARGS.files.iter() {
        match InputSource::open(it) {
            Ok(it) => readers.push_back(it),
            Err(it) => eprintln!("{}", it)
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

        STDOUT
            .clone()
            .slow_scan_write_by_chunks(iter, ARGS.slow_scan_config)
    } else {
        let iter = reader.chars().map(|it| it.unwrap());

        STDOUT
            .clone()
            .slow_scan_write_by_chars(iter, ARGS.slow_scan_config)
    }
    .unwrap_or_else(|it| {
        eprintln!("{}", t!("error.io_error_on_slow_scan_print", error = it));
    });
}
