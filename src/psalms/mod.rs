pub mod yaml;

pub trait Psalm<T> {
    fn invoke(&self, context: T) -> Result<String,String>;
}