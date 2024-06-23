// use serde::{ser::SerializeStruct, Deserialize, Serialize};

// use crate::{identifier::{Extended, Id, IdExtended}, pgn::{CommunicationMode, DestinationAddress, GroupExtension, PduAssignment, PduFormat, PgnBits}, prelude::{IdConvert, IdKind}};

// #[derive(Debug, Serialize)]
// pub struct ExtPgnInfo {
//     pub pdu_assignment: PduAssignment, 
//     pub pdu_format: PduFormat,
//     pub comms_mode: CommunicationMode,
//     pub destination_address: DestinationAddress,
//     pub group_extension: GroupExtension
// }

// #[derive(Debug, Serialize)]
// pub struct ExtIdInfo {
//     pub id_value: u32,
//     pub priority: u8,
//     pub pgn: ExtPgnInfo,
//     pub source_address: u8,
// }

// impl ExtIdInfo {
//     pub fn from_id(id: &IdExtended) -> Self {
//         let pgn = ExtPgnInfo {
//             pdu_assignment: id.pdu_assignment(), 
//             pdu_format: id.pdu_format(),
//             comms_mode: id.communication_mode(),
//             destination_address: id.destination_address(),
//             group_extension: id.group_extension()
//         };
//         Self {
//             id_value: id.into_bits(),
//             priority: id.priority_bits(),
//             pgn: pgn,
//             source_address: id.source_address_bits(),
//         }
//     }
// }

// impl Serialize for PgnBits {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let (r, dp, pf, ps) = &self.into_raw_parts();
//         let mut state = serializer.serialize_struct("pgn", 4)?;
//         state.serialize_field("reserved", r)?;
//         state.serialize_field("data_page", dp)?;
//         state.serialize_field("pdu_format", pf)?;
//         state.serialize_field("pdu_specific", ps)?;
        
//         state.end()
//     }
// }

// impl Serialize for Id<Extended> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         let mut state = serializer.serialize_struct("IdExtended", 3)?;
//         state.serialize_field("priority_bits", &self.priority_bits())?;
//         state.serialize_field("pgn", &self.pgn())?;
//         state.serialize_field("source_address_bits", &self.source_address_bits())?;
//         state.end()
//     }
// }

// impl<'de> Deserialize<'de> for IdExtended {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         const FIELDS: &[&str] = &["priority_bits", "reserved_bits", "data_page_bits", "pdu_format_bits", "pdu_specific_bits", "source_address_bits"];

//         struct Visitor;

//         impl<'de> serde::de::Visitor<'de> for Visitor {
//             type Value = IdExtended;

//             fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//                 formatter.write_str("struct IdExtended")
//             }

//             fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
//             where
//                 A: serde::de::MapAccess<'de>,
//             {
//                 let mut priority_bits = None;
//                 let mut reserved_bits = None;
//                 let mut data_page_bits = None;
//                 let mut pdu_format_bits = None;
//                 let mut pdu_specific_bits = None;
//                 let mut source_address_bits = None;

//                 while let Some(key) = map.next_key()? {
//                     match key {
//                         "priority_bits" => priority_bits = Some(map.next_value()?),
//                         "reserved_bits" => reserved_bits = Some(map.next_value()?),
//                         "data_page_bits" => data_page_bits = Some(map.next_value()?),
//                         "pdu_format_bits" => pdu_format_bits = Some(map.next_value()?),
//                         "pdu_specific_bits" => pdu_specific_bits = Some(map.next_value()?),
//                         "source_address_bits" => source_address_bits = Some(map.next_value()?),
//                         _ => { let _: serde_json::Value = map.next_value()?; }, // Ignore unknown fields
//                     }
//                 }

//                 let priority_bits = priority_bits.ok_or_else(|| serde::de::Error::missing_field("priority_bits"))?;
//                 let reserved_bits = reserved_bits.ok_or_else(|| serde::de::Error::missing_field("reserved_bits"))?;
//                 let data_page_bits = data_page_bits.ok_or_else(|| serde::de::Error::missing_field("data_page_bits"))?;
//                 let pdu_format_bits = pdu_format_bits.ok_or_else(|| serde::de::Error::missing_field("pdu_format_bits"))?;
//                 let pdu_specific_bits = pdu_specific_bits.ok_or_else(|| serde::de::Error::missing_field("pdu_specific_bits"))?;
//                 let source_address_bits = source_address_bits.ok_or_else(|| serde::de::Error::missing_field("source_address_bits"))?;

//                 Ok(IdExtended::from_raw_parts(priority_bits, reserved_bits, data_page_bits, pdu_format_bits, pdu_specific_bits, source_address_bits).unwrap())
//             }
//         }

//         deserializer.deserialize_struct("IdExtended", FIELDS, Visitor)
//     }
// }

// #[cfg(test)]
// mod serde_tests {
//     use std::println;

//     use crate::prelude::IdConvert;

//     use super::*;


//     #[test]
//     fn test_id_extended() -> Result<(), anyhow::Error>{
//         let id_a = IdExtended::from_hex("10FF2121");

//         let id_info = ExtIdInfo::from_id(&id_a);

//         let s = serde_json::to_string_pretty(&id_info).map_err(anyhow::Error::msg)?;

//         println!("{s}");

//         Ok(())
//     }
// }