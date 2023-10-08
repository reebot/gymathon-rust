pub fn get_setting(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| panic!("{} must be set.", key))
}
