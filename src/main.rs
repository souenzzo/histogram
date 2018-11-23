use std::ops::Add;
use std::ops::Div;
use std::collections::BTreeMap;

trait Number: Copy {
    fn one() -> Self;
    fn max(self, a: Self) -> Self;
    fn min(self, a: Self) -> Self;
    fn add(self, a: Self) -> Self;
    fn sorted_merge(a: Vec<Self>, b: Vec<Self>) -> Self;
}

trait IHistogram<T> {
    fn new(_: T) -> Box<Self>;
    fn combine(self, _: Box<Self>) -> Box<Self>;
}

struct Histogram<T> {
    max: T,
    min: T,
    size: T,
    sorted_values: Vec<T>,
}

impl<T> IHistogram<T> for Histogram<T> where T: HistValue {
    fn new(a: T) -> Box<Self> {
        return Box::new(
            Histogram {
                max: a,
                min: a,
                size: HistValue::one(),
                sorted_values: vec!(a),
            }
        );
    }

    fn combine(self, a: Box<Self>) -> Box<Self> {
        return Box::new(
            Histogram {
                max: T::max(self.max, a.max),
                min: T::min(self.min, a.min),
                size: T::add(self.size, a.size),
                sorted_values: T::sorted_merge(
                    self.sorted_values,
                    a.sorted_values),
            }
        );
    }
}

fn main() {
    let a = Average::new(5.0);
    let b = Average::new(3.0);
    let c = a.combine(b);
    let x = c.result();
    println!("Hello, world! {} ", x);
}
