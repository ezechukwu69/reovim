use std::{
    fs::{File, OpenOptions},
    io::Write,
};

use anyhow::Error;

pub struct Logger {}

impl Logger {
    pub fn log(text: String) -> anyhow::Result<(), Error> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("/home/ezechukwu69/.log/reovim/log.txt")?;
        file.write_all(format!("{}\n", text).as_bytes())?;
        Ok(())
    }
}
