use super::basic;

pub struct User {
    id: basic::Uuid,
    name: String,
    view_id: String,
    created: basic::Date,
    updated: Option<basic::Date>,
    icon: basic::Url,
    status: Option<String>,
    introduction: Option<String>
}
