use anyhow::Error;

pub trait Component {
    fn draw(&self) -> anyhow::Result<(), Error>;
}
