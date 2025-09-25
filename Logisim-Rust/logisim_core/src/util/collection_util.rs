/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Collection utility functions and structures
//!
//! Rust port of CollectionUtil.java

use std::collections::{HashMap, HashSet};

/// Collection utility functions equivalent to Java's CollectionUtil class
pub struct CollectionUtil;

impl CollectionUtil {
    /// Check if a collection is null or empty
    /// Equivalent to Java's isNullOrEmpty(Collection collection)
    pub fn is_null_or_empty<T>(collection: Option<&[T]>) -> bool {
        match collection {
            None => true,
            Some(slice) => slice.is_empty(),
        }
    }

    /// Check if a Vec is null or empty
    pub fn is_vec_null_or_empty<T>(collection: Option<&Vec<T>>) -> bool {
        match collection {
            None => true,
            Some(vec) => vec.is_empty(),
        }
    }

    /// Check if a HashMap is null or empty
    pub fn is_map_null_or_empty<K, V>(collection: Option<&HashMap<K, V>>) -> bool {
        match collection {
            None => true,
            Some(map) => map.is_empty(),
        }
    }

    /// Check if a HashSet is null or empty
    pub fn is_set_null_or_empty<T>(collection: Option<&HashSet<T>>) -> bool {
        match collection {
            None => true,
            Some(set) => set.is_empty(),
        }
    }

    /// Check if a collection is not empty and not null
    /// Equivalent to Java's isNotEmpty(Collection collection)
    pub fn is_not_empty<T>(collection: Option<&[T]>) -> bool {
        match collection {
            None => false,
            Some(slice) => !slice.is_empty(),
        }
    }

    /// Check if a Vec is not empty and not null
    pub fn is_vec_not_empty<T>(collection: Option<&Vec<T>>) -> bool {
        match collection {
            None => false,
            Some(vec) => !vec.is_empty(),
        }
    }

    /// Check if a HashMap is not empty and not null
    pub fn is_map_not_empty<K, V>(collection: Option<&HashMap<K, V>>) -> bool {
        match collection {
            None => false,
            Some(map) => !map.is_empty(),
        }
    }

    /// Check if a HashSet is not empty and not null
    pub fn is_set_not_empty<T>(collection: Option<&HashSet<T>>) -> bool {
        match collection {
            None => false,
            Some(set) => !set.is_empty(),
        }
    }
}

/// Union of two vectors (creates a new vector containing all elements)
/// Equivalent to Java's UnionList functionality
#[derive(Debug, Clone)]
pub struct UnionVec<T> {
    first: Vec<T>,
    second: Vec<T>,
}

impl<T> UnionVec<T> {
    pub fn new(first: Vec<T>, second: Vec<T>) -> Self {
        Self { first, second }
    }

    pub fn len(&self) -> usize {
        self.first.len() + self.second.len()
    }

    pub fn is_empty(&self) -> bool {
        self.first.is_empty() && self.second.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.first.len() {
            self.first.get(index)
        } else {
            self.second.get(index - self.first.len())
        }
    }
}

impl<T> IntoIterator for UnionVec<T> {
    type Item = T;
    type IntoIter = std::iter::Chain<std::vec::IntoIter<T>, std::vec::IntoIter<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.first.into_iter().chain(self.second)
    }
}

/// Union of two HashSets (creates a new set containing all unique elements)
/// Equivalent to Java's UnionSet functionality
#[derive(Debug, Clone)]
pub struct UnionSet<T> {
    first: HashSet<T>,
    second: HashSet<T>,
}

impl<T: Clone + Eq + std::hash::Hash> UnionSet<T> {
    pub fn new(first: HashSet<T>, second: HashSet<T>) -> Self {
        Self { first, second }
    }

    pub fn len(&self) -> usize {
        let mut combined = self.first.clone();
        combined.extend(self.second.iter().cloned());
        combined.len()
    }

    pub fn is_empty(&self) -> bool {
        self.first.is_empty() && self.second.is_empty()
    }

    pub fn contains(&self, value: &T) -> bool {
        self.first.contains(value) || self.second.contains(value)
    }

    pub fn to_set(self) -> HashSet<T> {
        let mut combined = self.first;
        combined.extend(self.second);
        combined
    }
}

impl<T: Clone + Eq + std::hash::Hash> IntoIterator for UnionSet<T> {
    type Item = T;
    type IntoIter = std::collections::hash_set::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.to_set().into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_null_or_empty() {
        let empty_vec: Vec<i32> = vec![];
        let non_empty_vec = vec![1, 2, 3];

        assert!(CollectionUtil::is_vec_null_or_empty::<i32>(None));
        assert!(CollectionUtil::is_vec_null_or_empty(Some(&empty_vec)));
        assert!(!CollectionUtil::is_vec_null_or_empty(Some(&non_empty_vec)));

        let empty_slice: &[i32] = &[];
        let non_empty_slice = &[1, 2, 3];

        assert!(CollectionUtil::is_null_or_empty::<i32>(None));
        assert!(CollectionUtil::is_null_or_empty(Some(empty_slice)));
        assert!(!CollectionUtil::is_null_or_empty(Some(non_empty_slice)));
    }

    #[test]
    fn test_is_not_empty() {
        let empty_vec: Vec<i32> = vec![];
        let non_empty_vec = vec![1, 2, 3];

        assert!(!CollectionUtil::is_vec_not_empty::<i32>(None));
        assert!(!CollectionUtil::is_vec_not_empty(Some(&empty_vec)));
        assert!(CollectionUtil::is_vec_not_empty(Some(&non_empty_vec)));
    }

    #[test]
    fn test_union_vec() {
        let first = vec![1, 2, 3];
        let second = vec![4, 5, 6];
        let union = UnionVec::new(first, second);

        assert_eq!(union.len(), 6);
        assert!(!union.is_empty());
        assert_eq!(union.get(0), Some(&1));
        assert_eq!(union.get(3), Some(&4));
        assert_eq!(union.get(6), None);

        let collected: Vec<i32> = union.into_iter().collect();
        assert_eq!(collected, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_union_set() {
        let mut first = HashSet::new();
        first.insert(1);
        first.insert(2);
        first.insert(3);

        let mut second = HashSet::new();
        second.insert(3);
        second.insert(4);
        second.insert(5);

        let union = UnionSet::new(first, second);
        assert!(union.contains(&1));
        assert!(union.contains(&3));
        assert!(union.contains(&5));
        assert!(!union.contains(&6));

        let result_set = union.to_set();
        assert_eq!(result_set.len(), 5); // 1, 2, 3, 4, 5
    }

    #[test]
    fn test_map_operations() {
        let empty_map: HashMap<i32, String> = HashMap::new();
        let mut non_empty_map = HashMap::new();
        non_empty_map.insert(1, "one".to_string());

        assert!(CollectionUtil::is_map_null_or_empty::<i32, String>(None));
        assert!(CollectionUtil::is_map_null_or_empty(Some(&empty_map)));
        assert!(!CollectionUtil::is_map_null_or_empty(Some(&non_empty_map)));

        assert!(!CollectionUtil::is_map_not_empty::<i32, String>(None));
        assert!(!CollectionUtil::is_map_not_empty(Some(&empty_map)));
        assert!(CollectionUtil::is_map_not_empty(Some(&non_empty_map)));
    }

    #[test]
    fn test_set_operations() {
        let empty_set: HashSet<i32> = HashSet::new();
        let mut non_empty_set = HashSet::new();
        non_empty_set.insert(1);

        assert!(CollectionUtil::is_set_null_or_empty::<i32>(None));
        assert!(CollectionUtil::is_set_null_or_empty(Some(&empty_set)));
        assert!(!CollectionUtil::is_set_null_or_empty(Some(&non_empty_set)));

        assert!(!CollectionUtil::is_set_not_empty::<i32>(None));
        assert!(!CollectionUtil::is_set_not_empty(Some(&empty_set)));
        assert!(CollectionUtil::is_set_not_empty(Some(&non_empty_set)));
    }
}
