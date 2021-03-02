use std::cell::RefCell;
use std::collections::VecDeque;

use hamcrest2::assert_that;
use hamcrest2::prelude::*;
pub(crate) use log::Level;

static LOGGER: TestLogger = TestLogger;

thread_local!(
    static QUEUE: RefCell<VecDeque<LogEntry>> = RefCell::new(VecDeque::new());
);

#[derive(Clone, Debug, PartialEq)]
struct LogEntry {
    message: String,
    level: Level,
}

struct TestLogger;

impl log::Log for TestLogger {
    fn enabled(&self, _: &log::Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &log::Record<'_>) {
        QUEUE.with(|q| {
            let queue = &mut *q.borrow_mut();
            queue.push_back(LogEntry {
                message: record.args().to_string(),
                level: record.level(),
            });
        });
    }

    fn flush(&self) {}
}

pub(crate) struct LoggerContext {
    // force use of constructor
    _secret: (),
}

impl LoggerContext {
    pub(crate) fn new() -> Self {
        log::set_max_level(log::LevelFilter::Trace);
        // fails if another test already registered this logger
        let _ = log::set_logger(&LOGGER);

        Self { _secret: () }
    }

    pub(crate) fn assert_entry<T: Into<String>>(&self, level: Level, message: T) {
        QUEUE.with(|q| {
            let queue = &mut *q.borrow_mut();
            assert_that!(queue.is_empty(), is(false));

            let entry = queue.pop_front().unwrap();
            let expected = LogEntry {
                level,
                message: message.into(),
            };

            assert_that!(entry, is(equal_to(expected)));
        });
    }
}

impl Drop for LoggerContext {
    fn drop(&mut self) {
        QUEUE.with(|q| {
            let queue = &mut *q.borrow_mut();
            let clone = queue.clone();
            queue.truncate(0);

            assert_that!(clone, is(equal_to(VecDeque::new())));
        });
    }
}
