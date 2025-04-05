use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    process::exit,
    time::Duration
};

use clap::{Arg, ArgAction, Command, arg};
use console::Term;
use rust_i18n::{set_locale, t};
use slow_scan_print::SlowScanWrite;

#[derive(Debug, Clone)]
struct Args {
    time: Duration,
    line_mode: bool,
    hide_cursor: bool,
    files: Vec<String>
}

#[cfg(all(feature = "safe", feature = "unsafe"))]
compile_error!("feature \"safe\" and feature \"unsafe\" cannot be enabled at the same time");

rust_i18n::i18n!("language", fallback = "zh-CN");

fn main() {
    init_locale();

    let args = args_handle();
    let mut stdout = Term::stdout();
    let stderr = Term::stderr();

    #[cfg(debug_assertions)]
    let _ = stderr.write_line(format!("Args: {:#?}", args).as_str());

    setup_ctrlc_handle(stdout.clone(), stderr.clone());

    if args.hide_cursor {
        stdout.hide_cursor().unwrap()
    }

    for name in args.files {
        let reader = match create_reader(name.as_str()) {
            Ok(it) => it,
            Err(it) => {
                let _ = stderr.write_line(
                    t!("error.can_not_open_file", name = it.0, error = it.1)
                        .to_string()
                        .as_str()
                );
                continue;
            }
        };

        let iter = create_iterator(reader, args.line_mode);

        stdout
            .slow_scan_write(iter, args.time)
            .unwrap_or_else(|it| panic!("{}\n{}", t!("panic.io_error_on_slow_scan_print"), it));
    }

    if args.hide_cursor {
        stdout.show_cursor().unwrap()
    }
}

#[inline]
fn init_locale() {
    if let Some(it) = sys_locale::get_locale() {
        set_locale(it.as_str());
    };
}

fn args_handle() -> Args {
    let args = [
        arg!(delay: -d --delay <TIME>)
            .short_alias('t')
            .alias("time")
            .action(ArgAction::Set)
            .default_value("20ms")
            .help(t!("clap.delay.help").to_string())
            .long_help(t!("clap.delay.long_help").to_string()),
        Arg::new("line-mode")
            .short('l')
            .long("line-mode")
            .action(ArgAction::SetTrue)
            .help(t!("clap.line_mode.help").to_string()),
        Arg::new("hide-cursor")
            .short('c')
            .long("hide-cursor")
            .action(ArgAction::SetTrue)
            .help(t!("clap.hide_cursor.help").to_string()),
        Arg::new("files")
            .action(ArgAction::Append)
            .default_value("-")
            .help(t!("clap.files.help").to_string()),
        Arg::new("help")
            .short('h')
            .short_alias('?')
            .long("help")
            .action(ArgAction::Help)
            .help(format!("{}", t!("clap.help")))
            .long_help(format!("{}", t!("clap.long_help")))
    ];

    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .disable_version_flag(true)
        .disable_help_flag(true)
        .version(env!("CARGO_PKG_VERSION"))
        .about(t!("clap.about").to_string())
        .author(env!("CARGO_PKG_AUTHORS"))
        .args(&args)
        .get_matches();

    let time = match matches.get_one::<String>("delay") {
        Some(it) => duration_str::parse_std(it).unwrap_or_else(|it| panic!("{}", t!("panic.convert_string_to_duration_error", error = it))),
        #[cfg(feature = "unsafe")]
        None => unsafe { std::hint::unreachable_unchecked() },
        #[cfg(feature = "safe")]
        None => unreachable!()
    };

    let line_mode = match matches.get_one::<bool>("line-mode") {
        Some(it) => *it,
        #[cfg(feature = "unsafe")]
        None => unsafe { std::hint::unreachable_unchecked() },
        #[cfg(feature = "safe")]
        None => unreachable!()
    };

    let hide_cursor = match matches.get_one::<bool>("hide-cursor") {
        Some(it) => *it,
        #[cfg(feature = "unsafe")]
        None => unsafe { std::hint::unreachable_unchecked() },
        #[cfg(feature = "safe")]
        None => unreachable!()
    };

    let files = match matches.get_many::<String>("files") {
        Some(it) => it.map(|it| it.to_owned()).collect::<Vec<String>>(),
        #[cfg(feature = "unsafe")]
        None => unsafe { std::hint::unreachable_unchecked() },
        #[cfg(feature = "safe")]
        None => unreachable!()
    };

    Args {
        time: time,
        line_mode: line_mode,
        hide_cursor: hide_cursor,
        files: files
    }
}

fn setup_ctrlc_handle(stdout: Term, stderr: Term) {
    ctrlc::set_handler(move || {
        stdout.show_cursor().unwrap();
        exit(1)
    })
    .unwrap_or_else(|it| {
        let _ = stderr.write_line(
            t!("error.set_ctrlc_handle_error", error = it)
                .to_string()
                .as_str()
        );
    });
}

fn create_reader(name: &str) -> Result<BufReader<Box<dyn io::Read>>, (&str, io::Error)> {
    if name == "-" {
        let stdin = io::stdin();
        Ok(BufReader::new(Box::new(stdin)))
    } else {
        let file = match File::open(Path::new(name)) {
            Ok(it) => it,
            Err(err) => return Err((name, err))
        };
        Ok(BufReader::new(Box::new(file)))
    }
}

fn create_iterator(reader: BufReader<Box<dyn io::Read>>, line_mode: bool) -> Box<dyn Iterator<Item = String>> {
    let iter = reader.lines().map(|it| {
        let mut it = it.unwrap_or(String::new());
        #[cfg(target_family = "windows")]
        it.push('\r');
        it.push('\n');
        it
    });

    if line_mode {
        Box::new(iter)
    } else {
        Box::new(
            iter.flat_map(|it| it.chars().collect::<Vec<_>>())
                .map(|it| it.to_string())
        )
    }
}
