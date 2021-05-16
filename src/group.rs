use crate::operator::Operator;
use std::collections::HashSet;
use std::hash::Hash;

struct Group<T> {
    elements: HashSet<T>,
}

impl<T: Operator<T> + Eq + Hash> Group<T> {
    // fn from_elements(elements: HashSet<T>) -> Group<T> {
    //     if Group<T>.is_a_group(&elements):
    //         Group<T>{elements}
    //     else {
    //         panic!("elements and operator do not form a group")
    //     }
    // }

    fn is_a_group(elements: &HashSet<T>) -> bool {
        !elements.is_empty()
            && Self::is_closed(elements)
            && Self::is_associative(elements)
            && Self::has_identity(elements).0
            && Self::has_inverse(elements).0
    }

    fn is_closed(elements: &HashSet<T>) -> bool {
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

    fn is_associative(elements: &HashSet<T>) -> bool {
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

    fn has_identity(elements: &HashSet<T>) -> (bool, Option<&T>) {
        'outer: for x in elements {
            for y in elements {
                if &x.operate(y) != y {
                    continue 'outer;
                }
            }
            return (true, Some(x));
        }
        (false, None)
    }

    fn has_inverse(elements: &HashSet<T>) -> (bool, Option<Vec<(&T, &T)>>) {
        if let Some(identity) = (Self::has_identity(elements)).1 {
            let mut inverses: Vec<(&T, &T)> = Vec::with_capacity(elements.len());

            for x in elements {
                for y in elements {
                    if &x.operate(y) == identity {
                        inverses.push((x, y))
                    }
                }
            }
            if inverses.len() != elements.len() {
                return (false, None);
            }
            let non_self_inverses_1: HashSet<&T> =
                (&inverses).iter().map(|(a, _)| *a).collect::<HashSet<&T>>();
            let non_self_inverses_2: HashSet<&T> =
                (&inverses).iter().map(|(_, b)| *b).collect::<HashSet<&T>>();
            if (non_self_inverses_1 == non_self_inverses_2)
                && (non_self_inverses_2 == elements.iter().collect::<HashSet<&T>>())
            {
                return (true, Some(inverses));
            }
        }
        (false, None)
    }
}

#[cfg(test)]
mod test_group {
    use crate::group::Group;
    use crate::operator::{Operator, TestStruct};
    use itertools::Itertools;
    use std::collections::HashSet;

    #[test]
    fn test_mod_12_is_closed() {
        let elements: HashSet<TestStruct<u32>> = (0..12).map(|x| TestStruct { x }).collect();
        assert!(Group::is_closed(&elements))
    }

    #[test]
    fn test_mod_12_smaller_set_is_not_closed() {
        let elements: HashSet<TestStruct<u32>> = (0..11).map(|x| TestStruct { x }).collect();
        assert!(Group::is_closed(&elements) == false)
    }

    #[test]
    fn test_mod_12_is_associative() {
        let elements: HashSet<TestStruct<u32>> = (0..12).map(|x| TestStruct { x }).collect();
        assert!(Group::is_associative(&elements))
    }

    #[test]
    fn test_mod_12_subtraction_is_not_associative() {
        let elements: HashSet<TestStruct<i32>> = (0..12).map(|x| TestStruct { x }).collect();
        assert!(Group::is_associative(&elements) == false)
    }

    #[test]
    fn test_mod_12_has_identity() {
        let elements: HashSet<TestStruct<u32>> = (0..12).map(|x| TestStruct { x }).collect();
        assert!(Group::has_identity(&elements) == (true, Some(&TestStruct { x: 0u32 })))
    }

    #[test]
    fn test_mod_12_without_0_has_no_identity() {
        let elements: HashSet<TestStruct<u32>> = (1..12).map(|x| TestStruct { x }).collect();
        assert!(Group::has_identity(&elements) == (false, None))
    }

    #[test]
    fn test_mod_12_has_inverses() {
        let elements: HashSet<TestStruct<u32>> = (0..12).map(|x| TestStruct { x }).collect();
        let inverses: Vec<(&TestStruct<u32>, &TestStruct<u32>)> = vec![
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
        ];

        let (actual_bool, actual_vec): (bool, Option<Vec<(&TestStruct<u32>, &TestStruct<u32>)>>) =
            Group::has_inverse(&elements);
        assert!(actual_bool);

        let actual_vec = actual_vec.unwrap();
        assert_eq!(inverses.len(), actual_vec.len());
        for x in &actual_vec {
            assert!(inverses.contains(&x));
        }
        for x in inverses {
            assert!(&actual_vec.contains(&x));
        }
    }
}
