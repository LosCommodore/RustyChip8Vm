use anyhow::Result;
use ndarray::Array2;

pub trait Screen {
    fn draw(&mut self, mem: &Array2<bool>) -> Result<()>;
    fn key_input(&mut self) -> Result<Option<(char, bool)>>;
}
