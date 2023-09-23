use anyhow::{anyhow, Result};
use clap::Parser;
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Clone, Parser)]
pub struct OutputArgs {
    #[clap(long = "json", help = "JSON file where to write the ABI")]
    json: Option<PathBuf>,
    #[clap(
        long = "expandable",
        help = "Rust file where to write the ABI and the abigen macro"
    )]
    expandable: Option<PathBuf>,
    #[clap(
        long = "name",
        help = "Name of the contract (required if expandable is used)"
    )]
    contract_name: Option<String>,
}

impl OutputArgs {
    pub fn write<T>(&self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if let Some(json_path) = &self.json {
            let mut file = File::create(&json_path)?;
            serde_json::to_writer_pretty(&mut file, value)?;
            file.write_all(b"\n")?;
        };

        if let Some(expandable_path) = &self.expandable {
            let contract_name = if let Some(n) = &self.contract_name {
                n
            } else {
                return Err(anyhow!(
                    "When --expandable is used, you must also provide --name"
                ));
            };

            let mut file = File::create(&expandable_path)?;
            file.write_all(b"// WARNING: This file is auto-generated.\n\n")?;
            file.write_all(b"use abigen_macro::abigen;\n")?;
            file.write_all(b"use cairo_types::CairoType;\n")?;
            file.write_all(b"use starknet::core::types::{BlockId, BlockTag};\n")?;
            file.write_all(b"use starknet::providers::Provider;\n")?;
            file.write_all(b"use starknet::accounts::Account;\n")?;

            let abigen_line = format!("abigen!({},\nr#\"\n", contract_name);
            file.write_all(abigen_line.as_bytes())?;
            serde_json::to_writer_pretty(&mut file, value)?;
            file.write_all(b"\n\"#);\n")?;
        };

        Ok(())
    }
}
