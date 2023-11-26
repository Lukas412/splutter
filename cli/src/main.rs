use splutter::{Output, StrValidationExt};
use splutter_sql::{SqlSelectFrom, SqlStatement};

fn main() -> Result<(), ()> {
    test().ok_or(())
}

fn test() -> Option<()> {
    let sql = SqlSelectFrom::new(
        ("name".as_identifier()?, "age".as_identifier()?),
        "persons".as_identifier()?,
    )
    .sql_statement();
    let mut buffer = String::new();
    sql.output(&mut buffer);
    println!("{}", buffer);
    Some(())
}
