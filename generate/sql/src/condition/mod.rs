use crate::combinators::Output;

mod and;

pub trait SqlWhere {
    fn sql_where(self) -> impl Output;
}

pub trait SqlCondition {
    fn sql_condition(self) -> impl Output;
}
