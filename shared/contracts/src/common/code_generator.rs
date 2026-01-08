pub trait CodeGenerator: Send + Sync {
    fn generate(&self, prefix: &str) -> String;
}
