use crate::core::db::Database;

lazy_static! {
    pub static ref DB: Database = Database::connect();
}
