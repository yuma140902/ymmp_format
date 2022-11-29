use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Ymmp {
    #[serde(rename = "FilePath")]
    file_path: PathBuf,
}
