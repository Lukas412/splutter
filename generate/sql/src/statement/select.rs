use crate::condition::SqlWhere;
use crate::from::SqlFrom;
use crate::select::SqlSelect;
use crate::SqlStatement;
use splutter::Output;

pub struct SqlSelectFrom<S, F> {
    select: S,
    from: F,
}

impl<S, F> SqlSelectFrom<S, F>
where
    S: SqlSelect,
    F: SqlFrom,
{
    pub fn new(select: S, from: F) -> Self {
        Self { select, from }
    }
}

impl<S, F> SqlStatement for SqlSelectFrom<S, F>
where
    S: SqlSelect,
    F: SqlFrom,
{
    fn sql_statement(self) -> impl Output {
        (self.select.sql_select(), ' ', self.from.sql_from(), ';')
    }
}

pub struct SqlSelectFromWhere<S, F, W> {
    select: S,
    from: F,
    condition: W,
}

impl<S, F, W> SqlSelectFromWhere<S, F, W>
where
    S: SqlSelect,
    F: SqlFrom,
    W: SqlWhere,
{
    pub fn new(select: S, from: F, condition: W) -> Self {
        Self {
            select,
            from,
            condition,
        }
    }
}

impl<S, F, W> SqlStatement for SqlSelectFromWhere<S, F, W>
where
    S: SqlSelect,
    F: SqlFrom,
    W: SqlWhere,
{
    fn sql_statement(self) -> impl Output {
        (
            self.select.sql_select(),
            ' ',
            self.from.sql_from(),
            ' ',
            self.condition.sql_where(),
            ';',
        )
    }
}
