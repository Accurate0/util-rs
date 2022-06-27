use uuid::Uuid;

pub fn get() -> String {
    Uuid::new_v4().as_hyphenated().to_string()
}
