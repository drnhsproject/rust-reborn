use uuid::Uuid;
use crate::common::CodeGenerator;

pub struct UuidV7CodeGenerator;

impl CodeGenerator for UuidV7CodeGenerator {
    fn generate(&self, prefix: &str) -> String {
        format!("{}-{}", prefix, Uuid::now_v7())
    }
}
