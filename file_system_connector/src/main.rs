use file_system_connector::*;
use file_system_connector::common::*;
use file_system_connector::document_processing::*;
use file_system_connector::settings::*;
use file_system_connector::indexing_api::*;
use log::{LevelFilter};
use log4rs::{append::console::ConsoleAppender, config::{Appender, Root}, encode::pattern::PatternEncoder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = bootstrap_configuration()?;
    bootstrap_logging()?;

    let document_processor = DocumentProcessor::new(
        IndexingApiClient::new(config.task.indexing_api.uri.clone(), config.task.indexing_api.token.clone()),
        config.task.indexing_api);

    for document in traverse::traverse(&config.task.traverse.roots) {
        document_processor.process_document(document)?;
    }

    Ok(())
}

fn bootstrap_configuration() -> Result<Config, Box<dyn std::error::Error>> {
    let mut config_builder = config::Config::default();
    config_builder
        .merge(config::File::with_name("config.json"))?;
    Ok(config_builder.try_into::<Config>()?)
}

fn bootstrap_logging() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("[{d(%H:%M:%S)} {l}] {m}{n}")))
        .build();

    let log_config = log4rs::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))?;
    log4rs::init_config(log_config)?;
    Ok(())
}
