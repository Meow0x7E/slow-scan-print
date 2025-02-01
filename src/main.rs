use std::{
    io::{BufRead, BufReader, stdin},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering}
    },
    thread::{self},
    time::Duration
};

use clap::{Arg, ArgAction, Command, arg};
use console::Term;
use rust_i18n::t;
use signal_hook::{consts::signal, low_level::exit};
use slow_scan_print::SlowScanWrite;

#[derive(Debug)]
struct Args {
    time: Duration,
    line_mode: bool,
    hide_cursor: bool
}

rust_i18n::i18n!("locales", fallback = "zh-CN");

fn main() {
    rust_i18n::set_locale(&sys_locale::get_locale().unwrap_or_else(|| String::from("zh-CN")));

    let args = parser_args();
    let print_handle = thread::spawn(move || slow_scan_print(args));

    let listen = Arc::new(AtomicUsize::new(0));
    const SIGINT_U: usize = signal::SIGINT as usize;
    signal_hook::flag::register_usize(signal::SIGINT, Arc::clone(&listen), SIGINT_U)
        .unwrap_or_else(|_| panic!("{}", t!("error.c5b72dec-ae1b-49fb-8fb6-eff11313faea")));

    let delay = duration_str::parse("100ms").unwrap_or_else(|_| unreachable!("{}", t!("error.dbf620f1-e275-44c3-929a-5a946ca5daae")));
    loop {
        match listen.load(Ordering::Relaxed) {
            0 => {
                if print_handle.is_finished() {
                    break;
                }
                thread::sleep(delay);
                continue;
            }
            SIGINT_U => {
                break;
            }
            _ => {
                unreachable!("{}", t!("error.14837326-ccb0-42cd-9b30-27a4e64b5d01"))
            }
        }
    }

    Term::stdout().show_cursor().unwrap();
    exit(0);
}

fn parser_args() -> Args {
    #[inline]
    fn delay(it: &str) -> Duration {
        match duration_str::parse_std(it) {
            Ok(it) => it,
            Err(_) => panic!("{}", t!("clap.delay.value_parser.err_msg"))
        }
    }

    let matches = Command::new("slow-scan-print")
        .about(t!("clap.about").to_string())
        .author("Meow0x7E <Meow0x7E@outlook.com>")
        .args(&[
            arg!(delay: -d --delay <TIME>)
                .short_alias('t')
                .alias("time")
                .action(ArgAction::Set)
                .default_value("20ms")
                .help(format!("{}", t!("clap.delay.help"))),
            Arg::new("line-mode")
                .short('l')
                .long("line-mode")
                .action(ArgAction::SetTrue)
                .help(format!("{}", t!("clap.line-mode.help"))),
            Arg::new("hide-cursor")
                .short('c')
                .long("hide-cursor")
                .action(ArgAction::SetTrue)
                .help(format!("{}", t!("clap.hide-cursor.help")))
        ])
        .get_matches();

    Args {
        time: match matches.get_one::<String>("delay") {
            Some(it) => delay(it),
            None => duration_str::parse_std("20ms").unwrap_or_else(|_| unreachable!("{}", t!("error.dbf620f1-e275-44c3-929a-5a946ca5daae")))
        },
        line_mode: match matches.get_one::<bool>("line-mode") {
            Some(it) => *it,
            None => false
        },
        hide_cursor: match matches.get_one::<bool>("hide-cursor") {
            Some(it) => *it,
            None => false
        }
    }
}

fn slow_scan_print(args: Args) {
    let mut term = Term::stdout();
    let stdin = stdin();
    let reader = BufReader::new(stdin.lock());

    if args.hide_cursor {
        term.hide_cursor().unwrap()
    }

    if args.line_mode {
        let iter = reader
            .lines()
            .map(|l| format!("{}\n", l.unwrap_or(String::from(""))));
        term.slow_scan_write(iter, args.time)
            .unwrap_or_else(|it| panic!("{}\n{}", t!("error.026d13a1-ea8b-409a-8a4c-2ba551b475db"), it));
    } else {
        let iter = reader.lines().flat_map(|l| {
            let mut v: Vec<char> = l.unwrap_or(String::from("")).chars().collect();
            v.push('\n');
            v
        });
        term.slow_scan_write(iter, args.time)
            .unwrap_or_else(|it| panic!("{}\n{}", t!("error.026d13a1-ea8b-409a-8a4c-2ba551b475db"), it));
    }

    if args.hide_cursor {
        term.show_cursor().unwrap()
    }
}
