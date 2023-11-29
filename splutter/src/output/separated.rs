use crate::output::Output;

pub struct SeparatedBy<V, B> {
    values: V,
    by: B,
}

impl<V, B> SeparatedBy<V, B>
where
    Self: Output,
{
    pub fn new(values: V, by: B) -> Self {
        Self { values, by }
    }
}

impl<V1, V2, B> Output for SeparatedBy<(V1, V2), B>
where
    V1: Output,
    V2: Output,
    B: Output,
{
    fn output(self, output: &mut String) {
        self.values.0.output(output);
        self.by.output(output);
        self.values.1.output(output);
    }
}

impl<V1, V2, V3, B> Output for SeparatedBy<(V1, V2, V3), B>
where
    V1: Output,
    V2: Output,
    V3: Output,
    B: Copy,
    B: Output,
{
    fn output(self, output: &mut String) {
        self.values.0.output(output);
        self.by.output(output);
        self.values.1.output(output);
        self.by.output(output);
        self.values.2.output(output);
    }
}

impl<V1, V2, V3, V4, B> Output for SeparatedBy<(V1, V2, V3, V4), B>
where
    V1: Output,
    V2: Output,
    V3: Output,
    V4: Output,
    B: Copy,
    B: Output,
{
    fn output(self, output: &mut String) {
        self.values.0.output(output);
        self.by.output(output);
        self.values.1.output(output);
        self.by.output(output);
        self.values.2.output(output);
        self.by.output(output);
        self.values.3.output(output);
    }
}

impl<V1, V2, V3, V4, V5, B> Output for SeparatedBy<(V1, V2, V3, V4, V5), B>
where
    V1: Output,
    V2: Output,
    V3: Output,
    V4: Output,
    V5: Output,
    B: Copy,
    B: Output,
{
    fn output(self, output: &mut String) {
        self.values.0.output(output);
        self.by.output(output);
        self.values.1.output(output);
        self.by.output(output);
        self.values.2.output(output);
        self.by.output(output);
        self.values.3.output(output);
        self.by.output(output);
        self.values.4.output(output);
    }
}

pub trait Separated<B> {
    fn separated(self, by: B) -> impl Output;
}

impl<O1, O2, By> Separated<By> for (O1, O2)
where
    O1: Output,
    O2: Output,
    By: Copy,
    By: Output,
{
    fn separated(self, by: By) -> impl Output {
        (self.0, by, self.1)
    }
}

impl<O1, O2, O3, By> Separated<By> for (O1, O2, O3)
where
    O1: Output,
    O2: Output,
    O3: Output,
    By: Copy,
    By: Output,
{
    fn separated(self, by: By) -> impl Output {
        SeparatedBy::new(self, by)
    }
}

impl<O1, O2, O3, O4, By> Separated<By> for (O1, O2, O3, O4)
where
    O1: Output,
    O2: Output,
    O3: Output,
    O4: Output,
    By: Copy,
    By: Output,
{
    fn separated(self, by: By) -> impl Output {
        SeparatedBy::new(self, by)
    }
}
impl<O1, O2, O3, O4, O5, By> Separated<By> for (O1, O2, O3, O4, O5)
where
    O1: Output,
    O2: Output,
    O3: Output,
    O4: Output,
    O5: Output,
    By: Copy,
    By: Output,
{
    fn separated(self, by: By) -> impl Output {
        SeparatedBy::new(self, by)
    }
}
