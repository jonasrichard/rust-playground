use std::io::Write;
use std::sync::Mutex;
use std::{fs::File, time};

use log::{self, info, LevelFilter, Log};

struct FileLog {
    file: Mutex<File>,
}

impl Log for FileLog {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let mut f = self.file.lock().unwrap();

            writeln!(
                f,
                "{:?} - {} - {}",
                time::Instant::now(),
                record.level(),
                record.args()
            )
            .unwrap();
        }
    }

    fn flush(&self) {}
}

impl FileLog {
    fn new() -> FileLog {
        let log_file = File::create("out.log").expect("Cannot create out.log");

        FileLog {
            file: Mutex::new(log_file),
        }
    }
}

fn main() {
    log::set_boxed_logger(Box::new(FileLog::new())).expect("Cannot set global logger");
    log::set_max_level(LevelFilter::Info);

    info!("Let us try it out");
}
