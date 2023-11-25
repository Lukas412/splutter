use crate::sql::names::SqlName;
use crate::{Output, StrRefIdentifier};

pub trait SqlInsert {
    fn sql_insert(self) -> impl Output;
}

impl<N> SqlInsert for N
where
    N: SqlName,
{
    fn sql_insert(self) -> impl Output {
        ("INSERT INTO ", self.sql_name())
    }
}
