use crate::router::Transform;
use heck::MixedCase;

#[derive(Default)]
pub struct Camelize;

impl Transform for Camelize {
    fn transform(&self, str: String) -> String {
        str.to_mixed_case()
    }
}
