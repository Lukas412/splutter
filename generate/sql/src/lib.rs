#![feature(return_position_impl_trait_in_trait)]

mod condition;
mod fields;
mod from;
mod insert;
mod names;
mod select;
mod statement;
mod value;

pub use statement::{SqlInsertInto, SqlSelectFrom, SqlSelectFromWhere, SqlStatement};

#[cfg(test)]
mod tests {
    use super::*;
    use splutter::{StrValidationExt, Output};

    #[test]
    fn test_sql_select_generation() {
        let sql = SqlSelectFrom::new(
            ("name".as_identifier().unwrap(), "age".as_identifier().unwrap()),
            "persons".as_identifier().unwrap(),
        )
        .sql_statement();
        let mut buffer = String::new();
        sql.output(&mut buffer);
        assert_eq!(buffer, "SELECT name, age FROM persons;")
    }

    fn test_sql_select_where_generation() {
        let sql = SqlSelectFrom::new(
            ("name".as_identifier().unwrap(), "age".as_identifier().unwrap()),
            "persons".as_identifier().unwrap(),
        )
        .sql_statement();
        let mut buffer = String::new();
        sql.output(&mut buffer);
        assert_eq!(buffer, "SELECT name, age FROM persons;")
    }

    #[test]
    fn test_sql_insert_generation() {
        let sql = SqlInsertInto::new(
            "persons".as_identifier().unwrap(),
            ("name".as_identifier().unwrap(), "age".as_identifier().unwrap()),
            ("Tom\'s name", 23),
        )
        .sql_statement();
        let mut buffer = String::new();
        sql.output(&mut buffer);
        assert_eq!(
            buffer,
            "INSERT INTO persons (name, age) VALUES ('Tom''s name', 23);"
        )
    }
}
