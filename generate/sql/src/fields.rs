use crate::sql::names::{SqlName, SqlNames};
use crate::Output;

pub trait SqlFields {
    fn sql_fields(self) -> impl Output;
}

impl<N1, N2> SqlFields for (N1, N2)
where
    N1: SqlName,
    N2: SqlName,
{
    fn sql_fields(self) -> impl Output {
        ('(', self.sql_names(), ')')
    }
}

impl<N1, N2, N3> SqlFields for (N1, N2, N3)
where
    N1: SqlName,
    N2: SqlName,
    N3: SqlName,
{
    fn sql_fields(self) -> impl Output {
        ('(', self.sql_names(), ')')
    }
}

impl<N1, N2, N3, N4> SqlFields for (N1, N2, N3, N4)
where
    N1: SqlName,
    N2: SqlName,
    N3: SqlName,
    N4: SqlName,
{
    fn sql_fields(self) -> impl Output {
        ('(', self.sql_names(), ')')
    }
}
