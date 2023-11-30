use crate::Output;

pub trait SurroundWithBrackets {
    fn surround_with_brackets(self) -> impl Output;
}

impl<O> SurroundWithBrackets for O
where
    O: Output,
{
    fn surround_with_brackets(self) -> impl Output {
        ('(', self, ')')
    }
}

pub trait SurroundWithAngleBrackets {
    fn surround_with_angle_brackets(self) -> impl Output;
}

impl<O> SurroundWithAngleBrackets for O
where
    O: Output,
{
    fn surround_with_angle_brackets(self) -> impl Output {
        ('[', self, ']')
    }
}

pub trait SurroundWithCurlyBrackets {
    fn surround_with_curly_brackets(self) -> impl Output;
}

impl<O> SurroundWithCurlyBrackets for O
where
    O: Output,
{
    fn surround_with_curly_brackets(self) -> impl Output {
        ('{', self, '}')
    }
}
