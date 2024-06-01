use std::{
    collections::{hash_set, HashMap, HashSet},
    fs,
    marker::PhantomData,
    path::{Path, PathBuf},
};

use crate::{
    cli::{CandumpFormat, IsLogFormat, IsProtocol, B, J1939},
    file::{parse_candump_a, parse_candump_b, CandumpA, CandumpB},
};

#[derive(Debug)]
pub struct FileStat<P: IsProtocol, F: IsLogFormat> {
    pub id_stat: bool,
    phantom_p: PhantomData<P>,
    phantom_f: PhantomData<F>,
}

impl FileStat<J1939, B> {
    pub fn new(id_stat: bool) -> Self {
        Self {
            id_stat,
            phantom_p: PhantomData,
            phantom_f: PhantomData,
        }
    }

    pub fn show_stats<P: AsRef<Path>>(&self, filepath: P) -> Result<(), anyhow::Error> {
        let parsed = parse_candump_b(filepath)?;

        if self.id_stat {
            self.show_id_stat(&parsed[..])
        }

        Ok(())
    }

    fn show_id_stat(&self, parsed_lines: &[CandumpB]) {
        let mut unique_ids: HashMap<&str, usize> = HashMap::new();

        for line in parsed_lines.iter() {
            let id = &line.message;
        }
    }
}

#[cfg(test)]
mod file_stat_tests {
    use crate::cli::{A, J1939};

    use super::FileStat;

    #[test]
    fn test_new_file_stat() -> Result<(), anyhow::Error> {
        let fs_flags = FileStat::new(true);
        fs_flags
            .show_stats("../../Desktop/road/road/ambient/ambient_dyno_drive_basic_short.log")?;

        Ok(())
    }
}
