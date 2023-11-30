use crate::JsonValue;
use splutter::{Output, Separated, SurroundWithAngleBrackets};

pub trait JsonArray {
    fn json_array(self) -> impl Output;
}

impl<T1, T2> JsonArray for (T1, T2)
where
    T1: JsonValue,
    T2: JsonValue,
{
    fn json_array(self) -> impl Output {
        (self.0.json_value(), self.1.json_value())
            .separated(',')
            .surround_with_angle_brackets()
    }
}

impl<T1, T2, T3> JsonArray for (T1, T2, T3)
where
    T1: JsonValue,
    T2: JsonValue,
    T3: JsonValue,
{
    fn json_array(self) -> impl Output {
        (
            self.0.json_value(),
            self.1.json_value(),
            self.2.json_value(),
        )
            .separated(',')
            .surround_with_angle_brackets()
    }
}
