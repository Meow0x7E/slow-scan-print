use std::{
    io::{self, stdin, BufReader},
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc
    },
    thread::{self, JoinHandle}
};

use clap::Parser;
use console::Term;
use signal_hook::{consts::signal, low_level::exit};
use slow_scan_print::{print_chars_with_reader, print_lines_with_reader, slow_scan_write};

#[derive(Parser, Debug)]
#[command(name = "slow-scan-print", version)]
#[command(about = "Text is printed at fixed intervals by character or by line, and its name is inspired by SSTV (Slow Scan TV).")]
#[command(long_about = "
Text is printed at fixed intervals by character or by line, and its name is inspired by SSTV (Slow Scan TV).

slow-scan-print Copyright (C) 2024 Meow0x7E <Meow0x7E@outlook.com>
This program comes with ABSOLUTELY NO WARRANTY;
This is free software, and you are welcome to redistribute it under certain conditions;
")]
#[command(author = "Meow0x7E <Meow0x7E@outlook.com>")]
struct Args {
    #[arg(short, long)]
    #[arg(default_value_t = String::from("20ms"))]
    #[arg(
        help = "Set the time interval for text printing. Time units of nanoseconds (ns) and higher are supported. Multiple time values can be combined using the \"+\" symbol."
    )]
    time: String,

    #[arg(short, long)]
    #[arg(help = "Print text by character instead of line")]
    line_mode: bool,

    #[arg(short = 'c', long)]
    #[arg(help = "Hides the cursor while printing and displays it when finished.")]
    hide_cursor: bool
}

fn main() -> Result<(), io::Error> {
    let args = Args::parse();
    let time = duration_str::parse(args.time).expect("时间格式不正确!");
    let mut term = Term::stdout();
    let stdin = stdin();
    let reader = BufReader::new(stdin.lock());

    let done = Arc::new(AtomicBool::new(false));
    let signal_hook = hook_sigint_signal(Arc::clone(&done));

    if args.hide_cursor {
        term.hide_cursor().unwrap()
    }

    if args.line_mode {
        print_lines_with_reader!(reader, time, &mut term)?;
    } else {
        print_chars_with_reader!(reader, time, &mut term)?;
    }

    if args.hide_cursor {
        term.show_cursor().unwrap()
    }

    Arc::clone(&done).store(true, Ordering::Relaxed);
    signal_hook.join().unwrap();
    Ok(())
}

fn hook_sigint_signal(done: Arc<AtomicBool>) -> JoinHandle<()> {
    thread::spawn(move || {
        let term = Arc::new(AtomicUsize::new(0));
        const SIGINT_U: usize = signal::SIGINT as usize;
        signal_hook::flag::register_usize(signal::SIGINT, Arc::clone(&term), SIGINT_U).expect("注册 Signal Hook 失败");

        loop {
            match term.load(Ordering::Relaxed) {
                0 => {
                    if done.load(Ordering::Relaxed) {
                        break;
                    }
                    continue;
                }
                SIGINT_U => {
                    Term::stdout().show_cursor().unwrap();
                    exit(1);
                }
                _ => unreachable!("The match should be unreachable.")
            }
        }
    })
}
