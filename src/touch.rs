use std::path::Path;
use filetime::{FileTime,set_file_times};
use std::time::{SystemTime};
use chrono::{TimeZone,DateTime,Local,NaiveDateTime,Utc};

// 日時文字列を SystemTime に変換する関数
pub fn parse_datetime(date_str: &str) -> Result<SystemTime, Box<dyn std::error::Error>> {
    let format = "%Y-%m-%d %H:%M";
    let datetime = NaiveDateTime::parse_from_str(date_str, format)?;
    let local_datetime: DateTime<Local> = Local.from_local_datetime(&datetime).unwrap();
    let system_time = local_datetime.with_timezone(&Utc).into();
    Ok(system_time)
}
// ファイルの時刻を指定日時に変更
pub fn change_file_time(file_path:&str,datetime_str:&str)->Result<(),Box<dyn std::error::Error>>{
    let system_time = parse_datetime(datetime_str)?;
    let file_time = FileTime::from_system_time(system_time);
    filetime::set_file_times(file_path,file_time,file_time)?;
    Ok(())
}

// ファイルの時刻を現在時刻に変更
pub fn update_access_time<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
    let now = SystemTime::now();
    let access_time = FileTime::from_system_time(now);
    let metadata = std::fs::metadata(&path)?;
    let modification_time = FileTime::from_last_modification_time(&metadata);
    set_file_times(path, access_time, modification_time)?;
    Ok(())
}
// ファイルの修正時間を現在時刻に変更する関数
pub fn update_modification_time<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
    let now = SystemTime::now();
    let modification_time = FileTime::from_system_time(now);
    let metadata = std::fs::metadata(&path)?;
    let access_time = FileTime::from_last_access_time(&metadata);
    set_file_times(path, access_time, modification_time)?;
    Ok(())
}
// ファイルの時刻を参照ファイルの時刻に変更
pub fn update_time_from_reference<P: AsRef<Path>>(path: P, reference: P) -> Result<(), Box<dyn std::error::Error>> {
    let metadata = std::fs::metadata(reference)?;
    let access_time = FileTime::from_last_access_time(&metadata);
    let modification_time = FileTime::from_last_modification_time(&metadata);
    set_file_times(path, access_time, modification_time)?;
    Ok(())
}
pub fn update_timestamp<P:AsRef<Path>>(path:P,timestamp_str:&str)->Result<(),Box<dyn std::error::Error>>{
    // タイムスタンプをパース
    let datetime = chrono::NaiveDateTime::parse_from_str(timestamp_str, "%Y%m%d%H%M.%S").expect("Invalid timestamp format");
    let timestamp = datetime.timestamp();
    let atime = FileTime::from_unix_time(timestamp,0);
    let mtime = FileTime::from_unix_time(timestamp,0);
    set_file_times(path,atime,mtime)?;
    Ok(())
}