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

pub trait Transform {
    fn transform(&self, str: String) -> String;
}
