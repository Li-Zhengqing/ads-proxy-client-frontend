// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::{File};
use std::io::Write;

use ads::{Handle, AmsNetId, symbol};

use log::{info, warn, error, debug, Log};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};
use serde::Serialize;

#[tauri::command]
fn query_variable_list() -> Vec<String> {
  println!("Setup ADS Client...");
    // let client = ads::Client::new(("127.0.0.1", ads::PORT), ads::Timeouts::none(), ads::Source::Auto)?;
    let timeout = ads::Timeouts::new(std::time::Duration::from_secs(5));
    // let client = ads::Client::new(("101.6.57.107", ads::PORT), 
    /* FIXME
    let client = ads::Client::new(("127.0.0.1", ads::PORT), 
                timeout, ads::Source::Auto)?;
     */
    let client = ads::Client::new(("127.0.0.1", ads::PORT), 
                timeout, ads::Source::Request).unwrap();
    info!("Client setup!");
    // let device = client.device(ads::AmsAddr::new([101, 6, 57, 107, 1, 1].into(), 851));
    let device = client.device(ads::AmsAddr::new(AmsNetId::local(), 851));
    // let device = client.device(ads::AmsAddr::new([127, 0, 0, 1, 1, 1].into(), 851));
    debug!("{}", AmsNetId::local());
    println!("Device State: {:?}", device.get_state().unwrap().0);
    info!("Device setup!");

    // assert!(device.get_state()?.0 == ads::AdsState::Run);
    let (symbols, type_map) = ads::symbol::get_symbol_info(device).unwrap();

    let symbol_name_list: Vec<String> = Vec::new();
    for sym in symbols.iter() {
        println!("{:4x}:{:6x} ({:6x}) {:50} {}",
            sym.ix_group, sym.ix_offset, sym.size, sym.name, sym.typ);
            // sym.ix_group, sym.ix_offset, sym.size, sym.name, sym.typ);
    }

    let symbols: Vec<String> = symbols.iter().map(|item| String::from(&item.name)).collect();
    // symbol_name_list
    symbols
}

fn main() {
  println!("Initializing logger...");
    let stdout = ConsoleAppender::builder().build();
    let requests = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build("log/requests.log")
        .unwrap();
    let log_config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("requests", Box::new(requests)))
        .logger(Logger::builder().build("app::backend::db", log::LevelFilter::Debug))
        .logger(Logger::builder().appender("requests").additive(false).build("app::requests", log::LevelFilter::Debug))
        .build(Root::builder().appender("stdout").build(log::LevelFilter::Debug))
        .unwrap();
    let _log_handle = log4rs::init_config(log_config).unwrap();
    println!("Logger initialization done!");

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![query_variable_list])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
