extern crate num_traits;
extern crate core;

use std::collections::BTreeMap;
use std::ops;
use std::cmp::Ordering;
use std::fmt;

pub struct Histogram<T> {
    max: T,
    min: T,
    size: T,
    sorted_values: BTreeMap<T, T>,
}

pub trait Num: num_traits::One + num_traits::Zero + ops::Add + Ord + Clone + fmt::Display {}

impl<T> Histogram<T> where T: Num {
    fn new(a: T) -> Self {
        let mut sorted_values = BTreeMap::new();
        sorted_values.insert(T::clone(&a), T::one());
        return Self {
            max: T::clone(&a),
            min: T::clone(&a),
            size: T::one(),
            sorted_values: sorted_values,
        };
    }


    fn from_vec(data: Vec<T>) -> Option<Self> {
        let maybe_data = data.split_first();
        match maybe_data {
            None => {
                None
            }
            Some((v0, vs)) => {
                Some(vs.iter().fold(Self::new(T::clone(&v0)),
                                    |acc, v|
                                        acc.combine(Self::new(T::clone(&v)))))
            }
        }
    }

    fn combine(self, a: Self) -> Self {
        let mut sorted_values = BTreeMap::new();
        sorted_values.clone_from(&self.sorted_values);
        for (i, v) in a.sorted_values.iter() {
            let actual_v = self.sorted_values.get(i);
            match actual_v {
                None => {
                    sorted_values.insert(T::clone(&i), T::clone(&v));
                }
                Some(x) => {
                    sorted_values.insert(T::clone(&i),
                                         T::add(T::clone(&v), T::clone(&x)),
                    );
                }
            };
        }
        return Self {
            max: match T::cmp(&self.max, &a.max) {
                Ordering::Equal => {
                    self.max
                }
                Ordering::Less => {
                    a.max
                }
                Ordering::Greater => {
                    self.max
                }
            },
            min: match T::cmp(&self.min, &a.min) {
                Ordering::Equal => {
                    self.min
                }
                Ordering::Less => {
                    self.min
                }
                Ordering::Greater => {
                    a.min
                }
            },
            size: T::add(self.size, a.size),
            sorted_values: sorted_values,
        };
    }

    fn normalize_by_min_plus_interval(self, i: T) -> Self {
        unimplemented!()
    }
}

impl<T> PartialEq for Histogram<T> where T: Num {
    fn eq(&self, other: &Self) -> bool {
        return self.sorted_values == other.sorted_values;
    }
}


fn main() {
    println!("Hello, world! ");
}

#[cfg(test)]
mod test {
    extern crate core;

    use Histogram;
    use Num;
    use std::fmt;

    impl Num for i64 {}

    impl<T> fmt::Debug for Histogram<T> where T: Num {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let content = self.sorted_values.iter().map(|e| {
                let (k, v) = e;
                // TODO: Print as vec!() form
                return format!("{} {}", k, v);
            });
            write!(f, "Histogram::from_vec (vec!({}))",
                   content.fold(String::new(), |acc, x| format!("{} {},", acc, x)))
        }
    }

    #[test]
    fn test_01() {
        assert_eq!(
            Some(Histogram::new(1)),
            Histogram::from_vec(vec!(1)),
        )
    }
}
