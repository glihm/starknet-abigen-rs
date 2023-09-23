use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use starknet::core::types::contract::{SierraClass};


use std::fs::File;
use crate::args::OutputArgs;

#[derive(Debug, Parser)]
pub struct FromSierra {
    #[clap(flatten)]
    output: OutputArgs,
    #[clap(help = "The Sierra file path to extract the ABI from")]
    sierra_path: PathBuf,
}

impl FromSierra {
    pub async fn run(self) -> Result<()> {
        let sierra_class = serde_json::from_reader::<_, SierraClass>(File::open(&self.sierra_path)?)?;
        let abi = sierra_class.abi;
        self.output.write(&abi)
    }
}
