use std::{fmt::Write, panic, panic::PanicHookInfo};

use js_sys::JsString;
pub use log::LevelFilter::*;
use log::*;
use screeps::game;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console;

struct JsLog;
struct JsNotify;

impl Log for JsLog {
    fn enabled(&self, _: &Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &Record<'_>) {
        console::log_1(&JsString::from(format!("{}", record.args())));
    }

    fn flush(&self) {}
}
impl Log for JsNotify {
    fn enabled(&self, _: &Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &Record<'_>) {
        game::notify(&format!("{}", record.args()), None);
    }

    fn flush(&self) {}
}

pub fn setup_logging(verbosity: LevelFilter) {
    fern::Dispatch::new()
        .level(verbosity)
        .format(|out, message, record| {
            let color = match record.level() {
                Level::Error => "#DC5257",
                Level::Warn => "#DC5257",
                Level::Info => "#F3C87B",
                Level::Debug => "#7986CB",
                Level::Trace => "#444444",
            };
            out.finish(format_args!(
                "<font color=\"{}\">[{}]</font> {}: {}",
                color,
                record.level(),
                record.target(),
                message
            ))
        })
        .chain(Box::new(JsLog) as Box<dyn Log>)
        .chain(
            fern::Dispatch::new()
                .level(Warn)
                .format(|out, message, _record| {
                    let time = game::time();
                    out.finish(format_args!("[{}] {}", time, message))
                })
                .chain(Box::new(JsNotify) as Box<dyn Log>),
        )
        .apply()
        .expect("expected setup_logging to only ever be called once per instance");
    panic::set_hook(Box::new(panic_hook));
}

#[wasm_bindgen]
extern "C" {
    type Error;

    #[wasm_bindgen(constructor)]
    fn new() -> Error;

    #[wasm_bindgen(structural, method, getter)]
    fn stack(error: &Error) -> String;

    #[wasm_bindgen(static_method_of = Error, setter, js_name = stackTraceLimit)]
    fn stack_trace_limit(size: f32);
}

fn panic_hook(info: &PanicHookInfo) {
    // import JS Error API to get backtrace info (backtraces don't work in wasm)
    // Node 8 does support this API: https://nodejs.org/docs/latest-v8.x/api/errors.html#errors_error_stack

    let mut fmt_error: String = String::new();
    let _ = writeln!(fmt_error, "{}", info);

    // this could be controlled with an env var at compilation instead
    const SHOW_BACKTRACE: bool = true;

    if SHOW_BACKTRACE {
        Error::stack_trace_limit(10000_f32);
        let stack = Error::new().stack();
        // Skip all frames before the special symbol `__rust_end_short_backtrace`
        // and then skip that frame too.
        // Note: sometimes wasm-opt seems to delete that symbol.
        if stack.contains("__rust_end_short_backtrace") {
            for line in stack
                .lines()
                .skip_while(|line| !line.contains("__rust_end_short_backtrace"))
                .skip(1)
            {
                let _ = writeln!(fmt_error, "{}", line);
            }
        } else {
            // If there was no `__rust_end_short_backtrace` symbol, use the whole stack
            // but skip the first line, it just says Error.
            let (_, stack) = stack.split_once('\n').unwrap();
            let _ = writeln!(fmt_error, "{}", stack);
        }
    }

    error!("{}", fmt_error);
}
