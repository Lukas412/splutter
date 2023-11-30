use crate::Output;

pub struct DoubleQuotedStr<'a> {
    value: &'a str,
    escape: char,
}

impl<'a> DoubleQuotedStr<'a> {
    pub fn new(value: &'a str, escape: char) -> Self {
        Self { value, escape }
    }
}

impl<'a> Output for DoubleQuotedStr<'a> {
    fn output(self, output: &mut String) {
        surround_str_with(output, self.value, '"', self.escape)
    }
}

pub struct SingleQuotedStr<'a> {
    value: &'a str,
    escape: char,
}

impl<'a> Output for SingleQuotedStr<'a> {
    fn output(self, output: &mut String) {
        surround_str_with(output, self.value, '\'', self.escape)
    }
}

impl<'a> SingleQuotedStr<'a> {
    pub fn new(value: &'a str, escape: char) -> Self {
        Self { value, escape }
    }
}

#[inline]
fn surround_str_with(output: &mut String, value: &str, surround: char, escape: char) {
    output.push(surround);
    for char in value.chars() {
        if char == surround {
            output.push(char);
        }
        output.push(char);
    }
    output.push(surround);
}
