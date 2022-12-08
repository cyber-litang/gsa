use anyhow::{Result, Context};
use std::{iter::repeat, path::Path};

pub fn write_file(path: &Path, data: Vec<(String, Vec<String>)>) -> Result<()> {
    let mut writer = csv::Writer::from_path(path)?;

    let titles = data.iter().map(|v| v.0.clone()).collect::<Vec<_>>();
    writer.write_record(titles)?;

    let mut datas = data
        .into_iter()
        .map(|v| v.1.into_iter().chain(repeat("".to_string())))
        .collect::<Vec<_>>();
    loop {
        let mut record = Vec::new();
        let mut has_data = false;

        for data in &mut datas {
            let s = data.next().context("data is empty")?;
            has_data |= !s.is_empty();
            record.push(s);
        }

        if has_data {
            writer.write_record(record)?;
        } else {
            break;
        }
    }
    Ok(())
}
