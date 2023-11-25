use crate::{Output, Separated, StrRefIdentifier};

pub trait SqlName {
    fn sql_name(self) -> impl Output;
}

impl<'a> SqlName for StrRefIdentifier<'a> {
    fn sql_name(self) -> impl Output {
        self
    }
}

pub struct SqlNameWithPath<'a> {
    path: &'a [StrRefIdentifier<'a>],
    name: StrRefIdentifier<'a>,
}

impl<'a> SqlNameWithPath<'a> {
    pub fn new(name: StrRefIdentifier<'a>) -> Option<Self> {
        Some(Self { path: &[], name })
    }
}

impl<'a> SqlName for SqlNameWithPath<'a> {
    fn sql_name(self) -> impl Output {
        (self.path, self.name)
    }
}

pub trait SqlNames {
    fn sql_names(self) -> impl Output;
}

impl<N1, N2> SqlNames for (N1, N2)
where
    N1: SqlName,
    N2: SqlName,
{
    fn sql_names(self) -> impl Output {
        (self.0.sql_name(), self.1.sql_name()).separated(", ")
    }
}

impl<N1, N2, N3> SqlNames for (N1, N2, N3)
where
    N1: SqlName,
    N2: SqlName,
    N3: SqlName,
{
    fn sql_names(self) -> impl Output {
        (self.0.sql_name(), self.1.sql_name(), self.2.sql_name())
    }
}

impl<N1, N2, N3, N4> SqlNames for (N1, N2, N3, N4)
where
    N1: SqlName,
    N2: SqlName,
    N3: SqlName,
    N4: SqlName,
{
    fn sql_names(self) -> impl Output {
        (
            self.0.sql_name(),
            self.1.sql_name(),
            self.2.sql_name(),
            self.3.sql_name(),
        )
    }
}
