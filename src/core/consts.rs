use crate::core::db::Database;

lazy_static! {
    pub static ref DB: Database = Database::connect();
}


pub const MESSAGE_CACHE: usize = 100;
