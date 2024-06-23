// use can_types::prelude::*;
// use chrono::{DateTime, Utc};
// use nom::{
//     bytes::complete::{tag, take_until},
//     character::complete::{digit1, hex_digit1, space1},
//     combinator::map_res,
//     sequence::{delimited, separated_pair},
//     IResult,
// };
// use std::{
//     fs::File, io::{BufRead, BufReader}, marker::PhantomData, path::Path
// };

// pub trait IsLogFormat {}

// pub trait ParseLineFormat {
//     type Output;
//     fn parse_line<'a>(input: &'a str) -> IResult<&'a str, Self::Output>;
// }

// pub trait ParseFileFormat {
//     type Output;
//     fn parse_candump<P: AsRef<Path>>(filepath: P) -> Result<Vec<Self::Output>, anyhow::Error>;
// }

// pub struct FileParser<F: IsLogFormat> {
//     phantom: PhantomData<F>
// }

// #[derive(Debug)]
// pub struct StandardLog<I: IdKind> {
//     pub timestamp: DateTime<Utc>,
//     pub i_face: String,
//     pub id: Id<I>,
//     pub pdu: Pdu<Data>,
// }

// pub struct HumanReadableLog<I: IdKind> {
//     pub i_face: String,
//     pub id: Id<I>,
//     pub data_len: usize,
//     pub pdu: Pdu<Data>,
// }

// impl IsLogFormat for StandardLog<Standard> {}
// impl IsLogFormat for StandardLog<Extended> {}
// impl IsLogFormat for HumanReadableLog<Standard> {}
// impl IsLogFormat for HumanReadableLog<Extended> {}

// pub enum LogFormat {
//     StandardLog,
//     HumanReadableLog,
// }

// impl ParseLineFormat for StandardLog<Extended> {
//     type Output = StandardLog<Extended>;
//     fn parse_line(input: &str) -> IResult<&str, StandardLog<Extended>> {
//         let (input, (secs, microsecs)) = delimited(
//             tag("("),
//             separated_pair(
//                 map_res(digit1, |s: &str| s.parse::<i64>()),
//                 tag("."),
//                 map_res(digit1, |s: &str| s.parse::<u32>()),
//             ),
//             tag(") "),
//         )(input)?;
//         let (input, interface) = take_until(" ")(input)?;
//         let (input, _) = space1(input)?;
//         let (input, (id, pdu)) = separated_pair(
//             map_res(hex_digit1, |s| {
//                 let res: Result<Id<Extended>, anyhow::Error> = Ok(IdExtended::from_hex(s));
//                 res
//             }),
//             tag("#"),
//             map_res(hex_digit1, |s| {
//                 let res: Result<Pdu<Data>, anyhow::Error> = Ok(PduData::from_hex(s));
//                 res
//             }),
//         )(input)?;
//         let timestamp = DateTime::from_timestamp(secs, microsecs * 1000).unwrap_or_default();

//         Ok((
//             input,
//             StandardLog::<Extended> {
//                 timestamp,
//                 i_face: interface.to_string(),
//                 id,
//                 pdu,
//             },
//         ))
//     }
// }

// impl ParseLineFormat for StandardLog<Standard> {
//     type Output = StandardLog<Standard>;
//     fn parse_line(input: &str) -> IResult<&str, StandardLog<Standard>> {
//         let (input, (secs, microsecs)) = delimited(
//             tag("("),
//             separated_pair(
//                 map_res(digit1, |s: &str| s.parse::<i64>()),
//                 tag("."),
//                 map_res(digit1, |s: &str| s.parse::<u32>()),
//             ),
//             tag(") "),
//         )(input)?;
//         let (input, interface) = take_until(" ")(input)?;
//         let (input, _) = space1(input)?;
//         let (input, (id, pdu)) = separated_pair(
//             map_res(hex_digit1, |s| {
//                 let res: Result<Id<Standard>, anyhow::Error> = Ok(IdStandard::from_hex(s));
//                 res
//             }),
//             tag("#"),
//             map_res(hex_digit1, |s| {
//                 let res: Result<Pdu<Data>, anyhow::Error> = Ok(PduData::from_hex(s));
//                 res
//             }),
//         )(input)?;
//         let timestamp = DateTime::from_timestamp(secs, microsecs * 1000).unwrap_or_default();

//         Ok((
//             input,
//             StandardLog::<Standard> {
//                 timestamp,
//                 i_face: interface.to_string(),
//                 id,
//                 pdu,
//             },
//         ))
//     }
// }

// impl ParseFileFormat for FileParser<StandardLog<Extended>> {
//     type Output = StandardLog<Extended>;
    
//     /// Parse a `can-utils` `candump` log file with entries in the following format:
//     ///
//     /// (0000000000.000000) can0 FFFFFFFF#FFFFFFFFFFFFFFFF
//     ///
//     /// # Errors
//     /// - If file does not exist
//     /// - If input file format is invalid
//     fn parse_candump<P: AsRef<Path>>(filepath: P) -> Result<Vec<Self::Output>, anyhow::Error> {
//         let file = File::open(filepath)?;
//         let reader = BufReader::new(file);

//         let mut messages: Vec<Self::Output> = Vec::new();

//         for line in reader.lines() {
//             if let Ok(line_string) = line {
//                 let trimmed_line = line_string.trim();
//                 let (_, parsed_line) =
//                     Self::Output::parse_line(trimmed_line).map_err(|e| anyhow::anyhow!("{e}"))?;
//                 messages.push(parsed_line)
//             }
//         }

//         Ok(messages)
//     }
// }

// impl ParseFileFormat for FileParser<StandardLog<Standard>> {
//     type Output = StandardLog<Standard>;
    
//     /// Parse a `can-utils` `candump` log file with entries in the following format:
//     ///
//     /// (0000000000.000000) can0 FFFFFFFF#FFFFFFFFFFFFFFFF
//     ///
//     /// # Errors
//     /// - If file does not exist
//     /// - If input file format is invalid
//     fn parse_candump<P: AsRef<Path>>(filepath: P) -> Result<Vec<Self::Output>, anyhow::Error> {
//         let file = File::open(filepath)?;
//         let reader = BufReader::new(file);

//         let mut messages: Vec<Self::Output> = Vec::new();

//         for line in reader.lines() {
//             if let Ok(line_string) = line {
//                 let trimmed_line = line_string.trim();
//                 let (_, parsed_line) =
//                     Self::Output::parse_line(trimmed_line).map_err(|e| anyhow::anyhow!("{e}"))?;
//                 messages.push(parsed_line)
//             }
//         }

//         Ok(messages)
//     }
// }

// // impl<I: IdKind> CandumpFileFormat for HumanReadableLog<I> {
// //     type Output = Self;

// //     /// Parse a `can-utils` `candump` log file with entries in the following format:
// //     ///
// //     /// can0 FFF [8] FF FF FF FF FF FF FF FF
// //     ///
// //     /// # Errors
// //     /// - If file does not exist
// //     /// - If input file format is invalid
// //     fn parse_candump<P: AsRef<Path>>(&self, filepath: P) -> Result<Vec<Self::Output>, anyhow::Error> {
// //         let invalid_file_err = anyhow!("Invalid file! The file must use the following format: can0 FFF [8] FF FF FF FF FF FF FF FF");

// /// Parse a `can-utils` `candump` log file with entries in the following format:
// ///
// /// can0 FFF [8] FF FF FF FF FF FF FF FF
// ///
// /// # Errors
// /// - If file does not exist
// /// - If input file format is invalid
// // pub fn parse_candump_a<P: AsRef<Path>>(filepath: P) -> Result<Vec<CandumpA>, anyhow::Error> {
// //     let invalid_file_err = anyhow!("Invalid file! The file must use the following format: can0 FFF [8] FF FF FF FF FF FF FF FF");

// #[cfg(test)]
// mod file_tests {
//     use can_types::prelude::*;

//     use super::*;

//     #[test]
//     fn test_file_parser_standard_extended() -> Result<(), anyhow::Error> {
//         let parsed_file = FileParser::<StandardLog<Extended>>::parse_candump("test_files/j1939dump.log")?;
//         for i in parsed_file {
//             let a: Addr = i.id.source_address().into();
//             println!("[{}]", a);
//         }
//         Ok(())
//     }

//     #[test]
//     fn test_file_parser_standard_standard() -> Result<(), anyhow::Error> {
//         let parsed_file = FileParser::<StandardLog<Standard>>::parse_candump("../../../Desktop/road/ambient/ambient_dyno_drive_basic_long.log")?;
//         for i in parsed_file {
//             println!(
//                 "{}, {}, ID={}, DATA={}",
//                 i.timestamp,
//                 i.i_face,
//                 i.id.into_bits(),
//                 i.pdu.into_bits()
//             );
//         }
//         Ok(())
//     }
// }
