mod camelize;
mod dasherize;
mod lowercase;
mod replace;
mod slice;
mod underscorize;
mod uppercase;

pub use camelize::Camelize;
pub use dasherize::Dasherize;
pub use lowercase::Lowercase;
pub use replace::Replace;
pub use slice::Slice;
pub use underscorize::Underscorize;
pub use uppercase::Uppercase;

pub struct Transformer {
    pub name: String,
    pub marker: String,
    transforms: Vec<Box<dyn Transform>>,
}

pub trait Transform {
    fn transform(&self, str: String) -> String;
}

impl Transformer {
    pub fn new(name: String, marker: String, transforms: Vec<Box<dyn Transform>>) -> Transformer {
        Transformer { name, marker, transforms }
    }

    pub fn transform(&self, str: String, value: &str) -> String {
        let mut replace_value = value.to_string();

        for transform in &self.transforms {
            replace_value = transform.transform(replace_value);
        }

        str.replace(format!("@{}", self.name).as_str(), replace_value.as_str())
    }
}
