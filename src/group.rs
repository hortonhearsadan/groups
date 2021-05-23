use crate::operator::Operator;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::axioms::*;
use itertools::Itertools;

pub struct Group<'a, T> {
    elements: &'a HashSet<T>,
    inverses: HashMap<&'a T, &'a T>,
    identity: &'a T,
}

impl<'b, T: 'b + Operator<T> + Eq + Hash> Group<'b, T> {
    fn is_abelian(&self) -> bool {
        self.elements.iter().combinations(2).all(|x| {
            x.first().unwrap().operate(x.last().unwrap())
                == x.last().unwrap().operate(x.first().unwrap())
        })
    }
}

pub struct GroupBuilder<'a, T> {
    elements: &'a HashSet<T>,
    check_associative: bool,
}

impl<'a, T: Operator<T> + Eq + Hash> GroupBuilder<'a, T> {
    pub fn new(elements: &HashSet<T>) -> GroupBuilder<T> {
        GroupBuilder {
            elements,
            check_associative: true,
        }
    }

    pub fn check_associativity(mut self, check: bool) -> GroupBuilder<'a, T> {
        self.check_associative = check;
        self
    }

    pub fn build(self) -> Group<'a, T> {
        let identity = identity(self.elements);
        let inverses = inverses(self.elements);
        let closure = is_closed(self.elements);

        if !(closure && identity.is_some() && inverses.is_some()) {
            panic!("elements and operator do not form a group")
        }

        if self.check_associative {
            if is_associative(self.elements) {
                Group {
                    elements: self.elements,
                    identity: identity.unwrap(),
                    inverses: inverses.unwrap(),
                }
            } else {
                panic!("elements and operator do not satisfy associativity")
            }
        } else {
            Group {
                elements: self.elements,
                identity: identity.unwrap(),
                inverses: inverses.unwrap(),
            }
        }
    }
}

#[cfg(test)]
mod test_group_builder {
    use crate::group::GroupBuilder;
    use crate::operator::{Operator, TestStruct};
    use std::cmp::Ordering::Greater;
    use std::collections::HashSet;

    #[test]
    fn test_mod_12_group_() {
        let elements: HashSet<TestStruct<u32>> = (0..12).map(|x| TestStruct { x }).collect();
        let group = GroupBuilder::new(&elements).build();

        assert_eq!(group.identity, &TestStruct { x: 0 })
    }
}
