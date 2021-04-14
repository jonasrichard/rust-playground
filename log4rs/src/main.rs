use log::{info, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;

fn setup_logger() {
    let stdout = ConsoleAppender::builder().build();

    let clientlog = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build("client.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("clientlog", Box::new(clientlog)))
        .logger(
            Logger::builder()
                .appender("clientlog")
                .additive(false)
                .build("client", LevelFilter::Info),
        )
        .build(
            Root::builder()
                .appender("clientlog")
                .build(LevelFilter::Debug),
        )
        .unwrap();

    log4rs::init_config(config).unwrap();
}

fn main() {
    setup_logger();

    info!("This is an info message");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_test() {
        setup_logger();

        log::info!("This comes from the test");

        assert!(true);
    }
}
