use splutter::{Output, Separated, StrRefIdentifier};

pub trait SqlSelect {
    fn sql_select(self) -> impl Output;
}

pub struct SqlSelectAll;

impl SqlSelect for SqlSelectAll {
    fn sql_select(self) -> impl Output {
        "SELECT *"
    }
}

impl<'a> SqlSelect for (StrRefIdentifier<'a>, StrRefIdentifier<'a>) {
    fn sql_select(self) -> impl Output {
        ("SELECT ", (self.0, self.1).separated(", "))
    }
}

impl<'a> SqlSelect
    for (
        StrRefIdentifier<'a>,
        StrRefIdentifier<'a>,
        StrRefIdentifier<'a>,
    )
{
    fn sql_select(self) -> impl Output {
        ("SELECT ", (self.0, self.1, self.2).separated(", "))
    }
}
