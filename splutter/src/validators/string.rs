use crate::output::Output;

pub trait StrValidationExt<'a> {
    fn as_integer(&self) -> Option<StrRefInteger<'a>>;
    fn as_decimal(&self) -> Option<StrRefDecimal<'a>>;
    fn as_identifier(&self) -> Option<StrRefIdentifier<'a>>;
}

impl<'a> StrValidationExt<'a> for &'a str {
    fn as_integer(&self) -> Option<StrRefInteger<'a>> {
        StrRefInteger::new(self)
    }

    fn as_decimal(&self) -> Option<StrRefDecimal<'a>> {
        StrRefDecimal::new(self)
    }

    fn as_identifier(&self) -> Option<StrRefIdentifier<'a>> {
        StrRefIdentifier::new(self)
    }
}

pub trait Swap {
    type Output;
    fn swap(self) -> Self::Output;
}

impl<T1, T2> Swap for (T1, T2) {
    type Output = (T2, T1);

    fn swap(self) -> Self::Output {
        (self.1, self.0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StrRefInteger<'a>(&'a str);

impl<'a> StrRefInteger<'a> {
    pub fn new(value: &'a str) -> Option<Self> {
        let is_not_empty = !value.is_empty();
        let is_digit = value.chars().all(|char| char.is_ascii_digit());
        (is_not_empty && is_digit).then_some(Self(value))
    }
}

impl<'a> Output for StrRefInteger<'a> {
    fn output(self, output: &mut String) {
        self.0.output(output)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StrRefDecimal<'a>(&'a str);

impl<'a> StrRefDecimal<'a> {
    pub fn new(value: &'a str) -> Option<Self> {
        let initial = value;
        let (value, before) = take_integer_or_nothing(value)?;
        let value = value.strip_prefix('.')?;
        let (value, after) = take_integer_or_nothing(value)?;
        (value.is_empty() && (!before.is_empty() || !after.is_empty())).then_some(Self(initial))
    }
}

impl<'a> Output for StrRefDecimal<'a> {
    fn output(self, output: &mut String) {
        self.0.output(output)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StrRefIdentifier<'a>(&'a str);

impl<'a> StrRefIdentifier<'a> {
    pub fn new(value: &'a str) -> Option<Self> {
        let mut chars = value.chars();
        let first_is_letter = chars.next()?.is_ascii_alphabetic();
        let remaining_are_identifier = chars.all(is_ascii_identifier);
        (remaining_are_identifier && first_is_letter).then_some(Self(value))
    }
}

impl<'a> Output for StrRefIdentifier<'a> {
    fn output(self, output: &mut String) {
        self.0.output(output)
    }
}

#[inline]
fn is_ascii_identifier(char: char) -> bool {
    matches!(char, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_')
}

fn take_integer(input: &str) -> Option<(&str, &str)> {
    let index = input
        .find(|char: char| !char.is_ascii_digit())
        .unwrap_or(input.len());
    (index != 0)
        .then_some(index)
        .map(|index| input.split_at(index).swap())
}

fn take_integer_or_nothing(input: &str) -> Option<(&str, &str)> {
    let index = input
        .find(|char: char| !char.is_ascii_digit())
        .unwrap_or(input.len());
    Some(input.split_at(index).swap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod integer {
        use super::*;

        #[test]
        fn can_parse() {
            let integer = "12".as_integer();
            assert_eq!(integer, Some(StrRefInteger("12")));
        }

        #[test]
        fn fail_on_empty() {
            let integer = "".as_integer();
            assert_eq!(integer, None);
        }

        #[test]
        fn fail_on_text() {
            let integer = "123abc".as_integer();
            assert_eq!(integer, None);
        }
    }

    #[cfg(test)]
    mod decimal {
        use super::*;

        #[test]
        fn can_parse_with_both_sides() {
            let decimal = "12.34".as_decimal();
            assert_eq!(decimal, Some(StrRefDecimal("12.34")))
        }

        #[test]
        fn can_parse_before_dot() {
            let decimal = "12.".as_decimal();
            assert_eq!(decimal, Some(StrRefDecimal("12.")));
        }

        #[test]
        fn can_parse_after_dot() {
            let decimal = ".34".as_decimal();
            assert_eq!(decimal, Some(StrRefDecimal(".34")));
        }

        #[test]
        fn fail_on_empty() {
            let decimal = "".as_decimal();
            assert_eq!(decimal, None);
        }

        #[test]
        fn fail_on_comma() {
            let decimal = "12,34".as_decimal();
            assert_eq!(decimal, None);
        }

        #[test]
        fn fail_on_text() {
            let decimal = "12.34abc".as_decimal();
            assert_eq!(decimal, None)
        }
    }

    #[cfg(test)]
    mod identifier {}

    #[cfg(test)]
    mod take_integer {
        use super::*;

        #[test]
        fn can_take_integer() {
            let result = take_integer("123abc");
            assert_eq!(result, Some(("abc", "123")));
        }

        #[test]
        fn can_take_integer_only() {
            let result = take_integer("123");
            assert_eq!(result, Some(("", "123")))
        }

        #[test]
        fn fail_for_no_integer() {
            let result = take_integer("abc");
            assert_eq!(result, None);
        }

        #[test]
        fn fail_if_not_starting_with_integer() {
            let result = take_integer("abc123");
            assert_eq!(result, None);
        }
    }

    #[cfg(test)]
    mod take_integer_or_nothing {
        use crate::validators::string::take_integer_or_nothing;

        #[test]
        fn can_take_integer() {
            let result = take_integer_or_nothing("123abc");
            assert_eq!(result, Some(("abc", "123")))
        }

        #[test]
        fn can_take_integer_only() {
            let result = take_integer_or_nothing("123");
            assert_eq!(result, Some(("", "123")))
        }

        #[test]
        fn can_take_nothing() {
            let result = take_integer_or_nothing("abc");
            assert_eq!(result, Some(("abc", "")))
        }
    }
}
