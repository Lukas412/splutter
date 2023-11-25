use splutter_generate::{Output, Sql, SqlSelectFrom, StrValidationExt};

fn main() -> Result<(), ()> {
    test().ok_or(())
}

fn test() -> Option<()> {
    let sql = SqlSelectFrom::new(("name".as_identifier()?, "age".as_identifier()?), "persons".as_identifier()?).sql();
    let mut buffer = String::new();
    sql.output(&mut buffer);
    println!("{}", buffer);
    Some(())
}
