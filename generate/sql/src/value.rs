use crate::combinators::Output;
use crate::{Separated, Sql};

pub trait SqlValue {
    fn sql_value(self) -> impl Output;
}

impl SqlValue for bool {
    fn sql_value(self) -> impl Output {
        self as u8
    }
}

impl SqlValue for u8 {
    fn sql_value(self) -> impl Output {
        self
    }
}

impl SqlValue for i8 {
    fn sql_value(self) -> impl Output {
        self
    }
}

impl SqlValue for u16 {
    fn sql_value(self) -> impl Output {
        self
    }
}

impl SqlValue for i16 {
    fn sql_value(self) -> impl Output {
        self
    }
}

impl SqlValue for u32 {
    fn sql_value(self) -> impl Output {
        self
    }
}

impl SqlValue for i32 {
    fn sql_value(self) -> impl Output {
        self
    }
}

impl SqlValue for u64 {
    fn sql_value(self) -> impl Output {
        self
    }
}

impl SqlValue for i64 {
    fn sql_value(self) -> impl Output {
        self
    }
}

impl<'a> SqlValue for &'a str {
    fn sql_value(self) -> impl Output {
        SqlString(self)
    }
}

struct SqlString<'a>(&'a str);

impl<'a> Output for SqlString<'a> {
    fn output(self, output: &mut String) {
        output.push('\'');
        for char in self.0.chars() {
            if char == '\'' {
                output.push(char);
            }
            output.push(char);
        }
        output.push('\'');
    }
}

pub trait SqlValues {
    fn sql_values(self) -> impl Output;
}

impl<V1, V2> SqlValues for (V1, V2)
where
    V1: SqlValue,
    V2: SqlValue,
{
    fn sql_values(self) -> impl Output {
        (
            "VALUES (",
            (self.0.sql_value(), self.1.sql_value()).separated(", "),
            ')',
        )
    }
}

impl<V1, V2, V3> SqlValues for (V1, V2, V3)
where
    V1: SqlValue,
    V2: SqlValue,
    V3: SqlValue,
{
    fn sql_values(self) -> impl Output {
        (
            "VALUES (",
            (self.0.sql_value(), self.1.sql_value(), self.2.sql_value()).separated(", "),
            ')',
        )
    }
}

impl<V1, V2, V3, V4> SqlValues for (V1, V2, V3, V4)
where
    V1: SqlValue,
    V2: SqlValue,
    V3: SqlValue,
    V4: SqlValue,
{
    fn sql_values(self) -> impl Output {
        (
            "VALUES (",
            (
                self.0.sql_value(),
                self.1.sql_value(),
                self.2.sql_value(),
                self.3.sql_value(),
            )
                .separated(", "),
            ')',
        )
    }
}
