pub trait AuthContext: Send + Sync {
    fn user_id(&self) -> Option<i64>;
}
