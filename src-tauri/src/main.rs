// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::{File};
use std::io::Write;

use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use ads::{Handle, AmsNetId, symbol};

use log::{info, warn, error, debug, Log};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};
// use serde::Serialize;

use lazy_static::lazy_static;


enum RecorderState {
  Ready,
  Recording(Vec<usize>),
}

struct ClientControlBlock {
  state: RecorderState,
  ads_client: Option<ads::Client>,
  // record_var_list: Vec<String>,
  record_job: Option<JoinHandle<()>>
  // ads_device: Option<ads::Device<'static>>,
}

lazy_static! {
  static ref CONTROL_BLOCK: Arc<Mutex<ClientControlBlock>> = Arc::new(
    Mutex::new(ClientControlBlock {
      state: RecorderState::Ready,
      ads_client: Option::None,
      // record_var_list: Vec::new(),
      record_job: Option::None,
      // ads_device: Option::None,
    }));
}

#[tauri::command]
fn start_record(var_list: Vec<String>) -> bool {
  let var_list: Vec<usize> = var_list.iter().map(|x| x.parse().unwrap()).collect();
  println!("Variable List Index: {:?}", var_list);
  info!("Start Recorder");

  let control_block = CONTROL_BLOCK.clone();
  let mut control_block_inner = control_block.lock().unwrap();

  // let plc_var_list = query_variable_list();
  // for i in var_list.clone() {
  //   control_block_inner.record_var_list.push(plc_var_list.get(i).unwrap().to_string());
  // }

  let record_job = thread::spawn(record_backend);
  control_block_inner.state = RecorderState::Recording(var_list);
  control_block_inner.record_job = Some(record_job);

  return true;
}

#[tauri::command]
fn stop_record() -> bool {
  info!("Stop Recorder");
  let control_block = CONTROL_BLOCK.clone();
  let mut control_block_inner = control_block.lock().unwrap();
  control_block_inner.state = RecorderState::Ready;

  // control_block_inner.record_job.unwrap().join();
  // let record_job = control_block_inner.record_job.as_mut();
  // record_job.unwrap().join();

  return true;
}

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
    let (symbols, _type_map) = ads::symbol::get_symbol_info(device).unwrap();

    // let symbol_name_list: Vec<String> = Vec::new();
    for sym in symbols.iter() {
        println!("{:4x}:{:6x} ({:6x}) {:50} {}",
            sym.ix_group, sym.ix_offset, sym.size, sym.name, sym.typ);
            // sym.ix_group, sym.ix_offset, sym.size, sym.name, sym.typ);
    }

    let symbols: Vec<String> = symbols.iter().map(|item| String::from(&item.name)).collect();
    // symbol_name_list
    symbols
}


fn record_backend() {
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

    // TODO: Multi-handlers
    let handle = Handle::new(device, "MAIN.counter").unwrap();
    // let control_block = CONTROL_BLOCK.clone();
    // let control_block_inner = control_block.lock().unwrap();

    // TODO: Here

    /*
    let handle: Vec<Handle<'_>> = Vec::new();
    let plc_var_list = query_variable_list();
    for i in var_index_list.clone() {
      // control_block_inner.record_var_list.push(plc_var_list.get(i).unwrap().to_string());
    }
    */

    // let fp: File = File::create("./data.csv").unwrap();
    let mut fp: File;
    if let Result::Ok(fp_) = File::create("./tmp/data.csv") {
        info!("Data record file opened successfully.");
        fp = fp_;
    } else {
        error!("Data record file failed to open!");
        std::process::exit(1);
    }

    let mut prev_value: i16 = 0;

    let mut record_flag: bool = true;
    while record_flag {

      for i in 0..250 {
          // TODO: Persistent data
          let value: i16 = handle.read_value().unwrap();
          // println!("[{}\t]: Value MAIN.counter = {}", i, value);
          fp.write_all((value.to_string() + ",\n").as_bytes()).expect("Failed when recording.");
          if value != prev_value + 1 {
              // warn!("[{}]: Data lost detected!", i);
          }
          prev_value = value;
      }

      let control_block = CONTROL_BLOCK.clone();
      let control_block_inner = control_block.lock().unwrap();
      match control_block_inner.state {
        RecorderState::Recording(_) => { record_flag = true },
        RecorderState::Ready => { record_flag = false },
        _ => { record_flag = false; },
      }
      // println!("Recording");
    }
}

fn backend() {
  // TODO: Main routine here.
  /* SECTION
   * 1. Connect to PLC and disconnect
   * 2. Query PLC variable list
   * 3. Start and stop monitor
   * 4. Data recording
   */
  
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
    .invoke_handler(tauri::generate_handler![start_record, stop_record, query_variable_list])
    // .invoke_handler(tauri::generate_handler![query_variable_list])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
