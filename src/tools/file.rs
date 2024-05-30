use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use anyhow::anyhow;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct CandumpB {
    pub timestamp: DateTime<Utc>,
    pub i_face: String,
    pub message: String,
}

/// # Errors
/// - If input file format is invalid
pub fn parse_candump_b(filepath: PathBuf) -> Result<Vec<CandumpB>, anyhow::Error> {
    let anyhow_err = anyhow!("Invalid file!!! The file must have a timestamp surrounded by '()' seperated by '.', interface, and message");
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut messages: Vec<CandumpB> = Vec::with_capacity(128);

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
                                return Err(anyhow_err);
                            }
                        }
                        _ => {
                            return Err(anyhow_err);
                        }
                    }
                }
                _ => {
                    return Err(anyhow_err);
                }
            }
        } else {
            return Err(anyhow_err);
        }
    }
    Ok(messages)
}

#[cfg(test)]
mod file_tests {
    use std::{path::PathBuf, str::FromStr};

    use crate::file::parse_candump_b;

    #[test]
    fn test_timestamp_from_log() -> Result<(), anyhow::Error> {
        let f = PathBuf::from_str("test_files/j1939dump.log")?;

        let file = parse_candump_b(f)?;

        for i in file {
            println!("{:?}", i)
        }
        Ok(())
    }
}
