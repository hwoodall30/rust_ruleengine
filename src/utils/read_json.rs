use serde::de::DeserializeOwned;
use serde_json::Deserializer;
use std::{error::Error, fs::File, io::BufReader};

pub fn read_json_file<T>(file_path: &str) -> Result<T, Box<dyn Error>>
where
    T: DeserializeOwned,
{
    let file = File::open(file_path)?;

    let data: T = serde_json::from_reader(file)?;

    Ok(data)
}

pub fn read_json_file_streaming<T>(file_path: &str) -> Result<T, Box<dyn std::error::Error>>
where
    T: serde::de::DeserializeOwned,
{
    let file = File::open(file_path)?;

    let reader = BufReader::new(file);

    let mut deserializer = Deserializer::from_reader(reader);

    let value: T = serde::de::Deserialize::deserialize(&mut deserializer)?;

    Ok(value)
}
