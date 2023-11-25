use crate::fields::SqlFields;
use crate::insert::SqlInsert;
use crate::value::SqlValues;
use crate::SqlStatement;
use splutter::Output;

pub struct SqlInsertInto<T, F, V> {
    table: T,
    fields: F,
    values: V,
}

impl<I, F, V> SqlInsertInto<I, F, V>
where
    I: SqlInsert,
    F: SqlFields,
    V: SqlValues,
{
    pub fn new(table: I, fields: F, values: V) -> Self {
        Self {
            table,
            fields,
            values,
        }
    }
}

impl<I, F, V> SqlStatement for SqlInsertInto<I, F, V>
where
    I: SqlInsert,
    F: SqlFields,
    V: SqlValues,
{
    fn sql(self) -> impl Output {
        (
            self.table.sql_insert(),
            ' ',
            self.fields.sql_fields(),
            ' ',
            self.values.sql_values(),
            ';',
        )
    }
}
