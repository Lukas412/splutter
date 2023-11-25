use splutter::{Output, Separated, StrRefIdentifier};

pub trait SqlFrom {
    fn sql_from(self) -> impl Output;
}

impl<'a> SqlFrom for StrRefIdentifier<'a> {
    fn sql_from(self) -> impl Output {
        ("FROM ", self)
    }
}

impl<'a> SqlFrom for (StrRefIdentifier<'a>, StrRefIdentifier<'a>) {
    fn sql_from(self) -> impl Output {
        ("FROM ", self.separated('.'))
    }
}

impl<'a> SqlFrom
    for (
        StrRefIdentifier<'a>,
        StrRefIdentifier<'a>,
        StrRefIdentifier<'a>,
    )
{
    fn sql_from(self) -> impl Output {
        ("FROM ", self.separated('.'))
    }
}
