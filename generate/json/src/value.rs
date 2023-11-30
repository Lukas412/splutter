use splutter::{DoubleQuotedStr, Output};

pub trait JsonValue {
    fn json_value(self) -> impl Output;
}

impl JsonValue for bool {
    fn json_value(self) -> impl Output {
        if self {
            "true"
        } else {
            "false"
        }
    }
}

macro_rules! impl_json_value {
    ($type:ty) => {
        impl JsonValue for $type {
            fn json_value(self) -> impl Output {
                self
            }
        }
    };
}

impl_json_value!(u8);
impl_json_value!(i8);
impl_json_value!(u16);
impl_json_value!(i16);
impl_json_value!(u32);
impl_json_value!(i32);
impl_json_value!(u64);
impl_json_value!(i64);

impl<'a> JsonValue for &'a str {
    fn json_value(self) -> impl Output {
        DoubleQuotedStr::new(self, '\\')
    }
}
