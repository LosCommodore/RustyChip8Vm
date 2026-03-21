use anyhow::Result;

pub trait Screen {
    fn draw(&mut self) -> Result<()>;
    fn key_input(&mut self) -> Result<Option<char>>;
}
