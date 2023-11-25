use splutter::Output;

mod insert;
mod select;

pub use {
    insert::SqlInsertInto,
    select::{SqlSelectFrom, SqlSelectFromWhere},
};

pub trait SqlStatement {
    fn sql_statement(self) -> impl Output;
}
