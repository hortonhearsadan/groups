use crate::operator::Operator;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

fn is_a_group<T: Operator<T> + Eq + Hash>(elements: &HashSet<T>) -> bool {
    !elements.is_empty()
        && is_closed(elements)
        && is_associative(elements)
        && identity(elements).is_some()
        && inverses(elements).is_some()
}

pub(crate) fn is_closed<T: Operator<T> + Eq + Hash>(elements: &HashSet<T>) -> bool {
    for x in elements {
        for y in elements {
            if elements.contains(&x.operate(y)) {
                continue;
            } else {
                return false;
            }
        }
    }
    true
}

pub(crate) fn is_associative<T: Operator<T> + Eq + Hash>(elements: &HashSet<T>) -> bool {
    for x in elements {
        for y in elements {
            for z in elements {
                if (x.operate(y)).operate(z) != x.operate(&y.operate(z)) {
                    return false;
                }
            }
        }
    }
    true
}

pub(crate) fn identity<T: Operator<T> + Eq + Hash>(elements: &HashSet<T>) -> Option<&T> {
    'outer: for x in elements {
        for y in elements {
            if &x.operate(y) != y {
                continue 'outer;
            }
        }
        return Some(x);
    }
    None
}

pub(crate) fn inverses<T: Operator<T> + Eq + Hash>(
    elements: &HashSet<T>,
) -> Option<HashMap<&T, &T>> {
    if let Some(identity) = identity(elements) {
        let mut inverses: HashMap<&T, &T> = HashMap::with_capacity(elements.len());

        for x in elements {
            for y in elements {
                if &x.operate(y) == identity {
                    inverses.insert(x, y);
                }
            }
        }
        if inverses.len() != elements.len() {
            return None;
        }
        let non_self_inverses_1: HashSet<&T> =
            (&inverses).iter().map(|(a, _)| *a).collect::<HashSet<&T>>();
        let non_self_inverses_2: HashSet<&T> =
            (&inverses).iter().map(|(_, b)| *b).collect::<HashSet<&T>>();
        if (non_self_inverses_1 == non_self_inverses_2)
            && (non_self_inverses_2 == elements.iter().collect::<HashSet<&T>>())
        {
            return Some(inverses);
        }
    }
    None
}

#[cfg(test)]
mod test_group {
    use crate::axioms::{identity, inverses, is_a_group, is_associative, is_closed};
    use crate::group::Group;
    use crate::operator::{Operator, TestStruct};
    use std::collections::{HashMap, HashSet};
    use std::iter::FromIterator;

    #[test]
    fn test_mod_12_is_closed() {
        let elements: HashSet<TestStruct<u32>> = (0..12).map(|x| TestStruct { x }).collect();
        assert!(is_closed(&elements))
    }

    #[test]
    fn test_mod_12_smaller_set_is_not_closed() {
        let elements: HashSet<TestStruct<u32>> = (0..11).map(|x| TestStruct { x }).collect();
        assert!(is_closed(&elements) == false)
    }

    #[test]
    fn test_mod_12_is_associative() {
        let elements: HashSet<TestStruct<u32>> = (0..12).map(|x| TestStruct { x }).collect();
        assert!(is_associative(&elements))
    }

    #[test]
    fn test_mod_12_subtraction_is_not_associative() {
        let elements: HashSet<TestStruct<i32>> = (0..12).map(|x| TestStruct { x }).collect();
        assert!(is_associative(&elements) == false)
    }

    #[test]
    fn test_mod_12_has_identity() {
        let elements: HashSet<TestStruct<u32>> = (0..12).map(|x| TestStruct { x }).collect();
        assert!(identity(&elements) == Some(&TestStruct { x: 0 }))
    }

    #[test]
    fn test_mod_12_without_0_has_no_identity() {
        let elements: HashSet<TestStruct<u32>> = (1..12).map(|x| TestStruct { x }).collect();
        assert!(identity(&elements).is_none())
    }

    #[test]
    fn test_mod_12_has_inverses() {
        let elements: HashSet<TestStruct<u32>> = (0..12).map(|x| TestStruct { x }).collect();
        let inverse: HashMap<&TestStruct<u32>, &TestStruct<u32>> = HashMap::from_iter(vec![
            (&TestStruct { x: 0 }, &TestStruct { x: 0 }),
            (&TestStruct { x: 1 }, &TestStruct { x: 11 }),
            (&TestStruct { x: 2 }, &TestStruct { x: 10 }),
            (&TestStruct { x: 3 }, &TestStruct { x: 9 }),
            (&TestStruct { x: 4 }, &TestStruct { x: 8 }),
            (&TestStruct { x: 5 }, &TestStruct { x: 7 }),
            (&TestStruct { x: 6 }, &TestStruct { x: 6 }),
            (&TestStruct { x: 7 }, &TestStruct { x: 5 }),
            (&TestStruct { x: 8 }, &TestStruct { x: 4 }),
            (&TestStruct { x: 9 }, &TestStruct { x: 3 }),
            (&TestStruct { x: 10 }, &TestStruct { x: 2 }),
            (&TestStruct { x: 11 }, &TestStruct { x: 1 }),
        ]);

        let actual_inverses = inverses(&elements);

        let actual_inverses = actual_inverses.unwrap();
        assert_eq!(inverse.len(), actual_inverses.len());
        assert_eq!(inverse, actual_inverses);
    }

    #[test]
    fn test_mod_12_is_a_group() {
        let elements: HashSet<TestStruct<u32>> = (0..12).map(|x| TestStruct { x }).collect();
        assert!(is_a_group(&elements))
    }

    #[test]
    fn test_mod_12_skipping_odd_is_a_group() {
        let elements: HashSet<TestStruct<u32>> = (&[0, 2, 4, 6, 8, 10])
            .into_iter()
            .map(|x| TestStruct { x: *x as u32 })
            .collect();
        assert!(is_a_group(&elements))
    }

    #[test]
    fn test_mod_12_larger_set_is_not_a_group() {
        let elements: HashSet<TestStruct<u32>> = (0..15).map(|x| TestStruct { x }).collect();
        assert!(identity(&elements).is_none())
    }
}
