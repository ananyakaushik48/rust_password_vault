// The struct is always made first
use chrono::prelude::*;
pub struct User {
    pub id: String,
    pub user_id: String,
    pub email_str: String,
    pub mobile_str: String,
    pub pincode_hash: String,
    pub added_at: DateTime<Utc>,
    pub description: String,
    pub tags: Vec<String>,
}