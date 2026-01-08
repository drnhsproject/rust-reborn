use crate::common::CodeGenerator;
use uuid::Uuid;

pub struct UuidV7CodeGenerator;

impl CodeGenerator for UuidV7CodeGenerator {
    fn generate(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, Uuid::now_v7())
    }
}
