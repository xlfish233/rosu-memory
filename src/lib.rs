pub mod reading_loop;
pub mod structs;
pub mod utils;

use anyhow::Result;
use reading_loop::process_reading_loop;
use rosu_mem::process::{Process, ProcessTraits};
pub use structs::OutputValues;
use structs::{State, StaticAddresses};

#[derive(Default)]
pub struct MemoryReader {
    process: Option<Process>,
    state: State,
}

// Workaround for new winello umu-run stuff
static EXCLUDE_WORDS: [&str; 2] = ["umu-run", "waitforexitandrun"];

impl MemoryReader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self) -> Result<()> {
        //Check process's status
        if self.process.is_none() {
            return Err(anyhow::anyhow!("process not initialized"));
        }

        //Init static addresses
        match StaticAddresses::new(self.process.as_ref().unwrap()) {
            Ok(v) => self.state.addresses = v,
            Err(e) => {
                return Err(e);
            }
        };
        process_reading_loop(self.process.as_ref().unwrap(), &mut self.state)?;
        Ok(())
    }

    pub fn get_values(&self) -> &OutputValues {
        &self.state.values
    }

    pub fn update_process(&mut self) -> Result<()> {
        let process = Process::initialize("osu!.exe", &EXCLUDE_WORDS)?;
        self.process = Some(process);
        let osu_executable_dir = self
            .process
            .as_ref()
            .unwrap()
            .executable_dir
            .clone()
            .ok_or(anyhow::anyhow!("Failed to get osu! path"))?;
        self.state.values.osu_path = osu_executable_dir.to_path_buf();
        Ok(())
    }
}
