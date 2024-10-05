use clap::{FromArgMatches, Parser};
use clap::CommandFactory;
use log::info;
use colored::*;
use std::fs::File;
use std::path::Path;
use filetime::{FileTime,set_file_times};
use std::time::{SystemTime,UNIX_EPOCH};
use std::os::windows::fs::OpenOptionsExt;
use std::fs::OpenOptions;
use chrono::{TimeZone,DateTime,Local,NaiveDateTime,Utc};
use touch::touch::*;

#[derive(Parser)]
#[command(name="Touch")]
struct Touch{
    #[arg(value_name="ACCESS",short,long,global = true)]
    access:bool,
    #[arg(value_name="MODIFY",short,long,global = true)]
    modify:Option<String>,
    #[arg(value_name="NO CREATE",short='c',long,global = true)]
    no_create:bool,
    #[arg(value_name="DATE",short,long,global = true)]
    date:Option<String>,
    #[arg(value_name="TIME",short,long,global = true)]
    time:Option<String>,
    #[arg(value_name="REFERENCE",short,long,global = true)]
    reference:Option<String>,
    #[arg(value_name="FILE")]
    file:Vec<String>,
}


fn main()->Result<(),Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    std::env::set_var("RUST_LOG","debug");
    env_logger::init();
    let touch = Touch::parse();
    if touch.file.len() < 1 {
        eprintln!("{}", "touch: missing file operand".truecolor(255, 165, 0));
        eprintln!("{}", "Try 'touch --help or --h or -h' for more information".red());
        Touch::command().print_long_help().unwrap();
        std::process::exit(1);
    }else{
        for f in touch.file.clone(){
            info!("file_name: {:?}",f.clone());
            if touch.access{
                update_access_time(&f)?;
            }else if touch.modify.is_some(){
                update_modification_time(&f)?;
            }else if touch.time.is_some(){
                let timestamp = touch.time.clone().unwrap();
                update_timestamp(&f,&timestamp)?;
            }else if touch.no_create{
                // ファイルが存在しない場合は何もせず処理を進める
                if !Path::new(&f).exists(){
                    continue;
                }
                update_access_time(&f)?;
                update_modification_time(&f)?;
            }else if touch.date.clone().is_some(){
                let date_str = touch.date.clone().unwrap();
                change_file_time(&f,&date_str)?;
            }else if touch.reference.is_some(){
                let reference_path = touch.reference.clone().unwrap();
                update_time_from_reference(&f, &reference_path)?;
            }else {
                // ファイルが存在する場合はアクセス時間と修正時間のみ上書き
                if Path::new(&f).exists() {
                    update_access_time(&f)?;
                    update_modification_time(&f)?;
                    continue;
                }
                File::create(&f)?;
            }
        }
    }
    #[cfg(debug_assertions)]
    std::env::remove_var("RUST_LOG");
    Ok(())
}
