pub use separated::Separated;
use std::ops::{Deref, Div, Index, Rem};
mod separated;

pub trait Output {
    fn output(self, output: &mut String);
}

impl<Ref> Output for &Ref
where
    Ref: Copy,
    Ref: Output,
{
    fn output(self, output: &mut String) {
        (*self).output(output)
    }
}

fn output_unsigned_integer(mut number: u64, output: &mut String) {
    const CHARS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut index = 0;
    loop {
        let (div, rem) = (number / 10, number % 10);
        let char = CHARS[rem as usize].clone();
        output.insert(output.len() - index, char);
        if div == 0 {
            break;
        }
        number = div;
        index += 1;
    }
}

fn output_signed_integer(mut number: i64, output: &mut String) {
    if number < 0 {
        output.push('-');
    }
    output_unsigned_integer(number.abs() as u64, output);
}

impl Output for u8 {
    fn output(self, output: &mut String) {
        output_unsigned_integer(self as u64, output);
    }
}

impl Output for i8 {
    fn output(self, output: &mut String) {
        output_signed_integer(self as i64, output);
    }
}

impl Output for u16 {
    fn output(self, output: &mut String) {
        output_unsigned_integer(self as u64, output);
    }
}

impl Output for i16 {
    fn output(self, output: &mut String) {
        output_signed_integer(self as i64, output);
    }
}

impl Output for u32 {
    fn output(self, output: &mut String) {
        output_unsigned_integer(self as u64, output);
    }
}

impl Output for i32 {
    fn output(self, output: &mut String) {
        output_signed_integer(self as i64, output);
    }
}

impl Output for u64 {
    fn output(self, output: &mut String) {
        output_unsigned_integer(self as u64, output);
    }
}

impl Output for i64 {
    fn output(self, output: &mut String) {
        output_signed_integer(self as i64, output);
    }
}

impl Output for char {
    fn output(self, output: &mut String) {
        output.push(self);
    }
}

impl<'a> Output for &'a str {
    fn output(self, output: &mut String) {
        output.push_str(self);
    }
}

impl<'a, T> Output for &'a [T]
where
    &'a T: Output,
{
    fn output(self, output: &mut String) {
        for item in self {
            item.output(output);
        }
    }
}

impl<O1, O2> Output for (O1, O2)
where
    O1: Output,
    O2: Output,
{
    fn output(self, output: &mut String) {
        self.0.output(output);
        self.1.output(output);
    }
}
impl<O1, O2, O3> Output for (O1, O2, O3)
where
    O1: Output,
    O2: Output,
    O3: Output,
{
    fn output(self, output: &mut String) {
        self.0.output(output);
        self.1.output(output);
        self.2.output(output);
    }
}

impl<O1, O2, O3, O4> Output for (O1, O2, O3, O4)
where
    O1: Output,
    O2: Output,
    O3: Output,
    O4: Output,
{
    fn output(self, output: &mut String) {
        self.0.output(output);
        self.1.output(output);
        self.2.output(output);
        self.3.output(output);
    }
}

impl<O1, O2, O3, O4, O5> Output for (O1, O2, O3, O4, O5)
where
    O1: Output,
    O2: Output,
    O3: Output,
    O4: Output,
    O5: Output,
{
    fn output(self, output: &mut String) {
        self.0.output(output);
        self.1.output(output);
        self.2.output(output);
        self.3.output(output);
        self.4.output(output);
    }
}

impl<O1, O2, O3, O4, O5, O6> Output for (O1, O2, O3, O4, O5, O6)
where
    O1: Output,
    O2: Output,
    O3: Output,
    O4: Output,
    O5: Output,
    O6: Output,
{
    fn output(self, output: &mut String) {
        self.0.output(output);
        self.1.output(output);
        self.2.output(output);
        self.3.output(output);
        self.4.output(output);
        self.5.output(output);
    }
}

impl<O1, O2, O3, O4, O5, O6, O7> Output for (O1, O2, O3, O4, O5, O6, O7)
where
    O1: Output,
    O2: Output,
    O3: Output,
    O4: Output,
    O5: Output,
    O6: Output,
    O7: Output,
{
    fn output(self, output: &mut String) {
        self.0.output(output);
        self.1.output(output);
        self.2.output(output);
        self.3.output(output);
        self.4.output(output);
        self.5.output(output);
        self.6.output(output);
    }
}
