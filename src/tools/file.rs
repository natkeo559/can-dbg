use std::{
    fs::File,
    io::{BufRead, BufReader},
    marker::PhantomData,
    path::{Path, PathBuf},
};

use anyhow::anyhow;
use chrono::{DateTime, Utc};

pub trait IsLogFormat {}

#[derive(Debug)]
pub struct CandumpA {
    pub i_face: String,
    pub id: String,
    pub data_len: usize,
    pub data_bytes: String,
}

#[derive(Debug)]
pub struct CandumpB {
    pub timestamp: DateTime<Utc>,
    pub i_face: String,
    pub message: String,
}

/// Parse a `can-utils` `candump` log file with entries in the following format:
///
/// can0 FFF [8] FF FF FF FF FF FF FF FF
///
/// # Errors
/// - If file does not exist
/// - If input file format is invalid
pub fn parse_candump_a<P: AsRef<Path>>(filepath: P) -> Result<Vec<CandumpA>, anyhow::Error> {
    let invalid_file_err = anyhow!("Invalid file! The file must use the following format: can0 FFF [8] FF FF FF FF FF FF FF FF");

    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut messages: Vec<CandumpA> = Vec::new();

    for lines in reader.lines() {
        if let Ok(line) = lines {
            let mut split_line = line.split_whitespace();
            let i_face_split_whitespace_opt = split_line.by_ref().next();
            let id_split_whitespace_opt = split_line.by_ref().next();
            let data_len_split_whitespace_opt = split_line.by_ref().next();
            match (
                i_face_split_whitespace_opt,
                id_split_whitespace_opt,
                data_len_split_whitespace_opt,
            ) {
                (Some(i_face), Some(id_str), Some(data_len_str)) => {
                    let data_len_stripped = data_len_str.trim_matches(&['[', ']']);
                    let data_len = data_len_stripped.parse::<usize>()?;
                    let data_bytes_string = (0..data_len)
                        .filter_map(|_| split_line.by_ref().next())
                        .collect::<String>();

                    let msg = CandumpA {
                        i_face: i_face.to_string(),
                        id: id_str.to_string(),
                        data_len,
                        data_bytes: data_bytes_string,
                    };
                    messages.push(msg);
                }
                _ => {
                    return Err(invalid_file_err);
                }
            }
        } else {
            return Err(invalid_file_err);
        }
    }
    Ok(messages)
}

/// Parse a `can-utils` `candump` log file with entries in the following format:
///
/// (0000000000.000000) can0 FFFFFFFF#FFFFFFFFFFFFFFFF
///
/// # Errors
/// - If file does not exist
/// - If input file format is invalid
pub fn parse_candump_b<P: AsRef<Path>>(filepath: P) -> Result<Vec<CandumpB>, anyhow::Error> {
    let invalid_file_err = anyhow!("Invalid file! The file must use the following format: (0000000000.000000) can0 FFFFFFFF#FFFFFFFFFFFFFFFF");

    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut messages: Vec<CandumpB> = Vec::new();

    for lines in reader.lines() {
        if let Ok(line) = lines {
            let mut split_line = line.split_whitespace();
            let date_time_split_whitespace_opt = split_line.by_ref().next();
            let i_face_split_whitespace_opt = split_line.by_ref().next();
            let msg_split_whitespace_opt = split_line.by_ref().next();

            match (
                date_time_split_whitespace_opt,
                i_face_split_whitespace_opt,
                msg_split_whitespace_opt,
            ) {
                (Some(date_time_split_whitespace), Some(i_face), Some(message)) => {
                    let date_time_split = date_time_split_whitespace
                        .trim_matches('(')
                        .trim_end_matches(')');
                    let mut split_date_time = date_time_split.split('.');
                    let date_time_str_opt = split_date_time.by_ref().next();
                    let microseconds_str_opt = split_date_time.by_ref().next();

                    match (date_time_str_opt, microseconds_str_opt) {
                        (Some(date_time_str), Some(microseconds_str)) => {
                            let date_time = date_time_str.parse::<i64>()?;
                            let microseconds = microseconds_str.parse::<u32>()?;
                            if let Some(timestamp) =
                                DateTime::from_timestamp(date_time, microseconds * 1000)
                            {
                                let msg = CandumpB {
                                    timestamp,
                                    i_face: i_face.to_string(),
                                    message: message.to_string(),
                                };
                                messages.push(msg);
                            } else {
                                return Err(invalid_file_err);
                            }
                        }
                        _ => {
                            return Err(invalid_file_err);
                        }
                    }
                }
                _ => {
                    return Err(invalid_file_err);
                }
            }
        } else {
            return Err(invalid_file_err);
        }
    }
    Ok(messages)
}

#[cfg(test)]
mod file_tests {
    use std::{path::PathBuf, str::FromStr};

    use crate::file::parse_candump_a;

    use super::parse_candump_b;

    #[test]
    fn test_timestamp_from_log() -> Result<(), anyhow::Error> {
        let file = parse_candump_b("test_files/j1939dump.log")?;

        for i in file {
            println!("{:?}", i)
        }
        Ok(())
    }
}
