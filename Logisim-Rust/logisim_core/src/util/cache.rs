/*
 * Logisim-evolution - digital logic design tool and simulator
 * Copyright by the Logisim-evolution developers
 *
 * https://github.com/logisim-evolution/
 *
 * This is free software released under GNU GPLv3 license
 */

//! Cache utility for immutable object caching
//!
//! Rust port of Cache.java

use std::hash::{Hash, Hasher};

/// A simple cache that allows immutable objects to be cached in memory
/// to reduce the creation of duplicate objects.
///
/// This is equivalent to Java's Cache class but uses Rust's type system
/// for better memory safety.
#[derive(Debug)]
pub struct Cache<T> {
    data: Vec<Option<T>>,
    mask: usize,
}

impl<T> Cache<T>
where
    T: Clone + PartialEq + Hash,
{
    /// Create a new cache with default size (256 entries)
    pub fn new() -> Self {
        Self::with_log_size(8)
    }

    /// Create a new cache with specified log size
    /// log_size determines the size as 2^log_size entries
    /// Maximum log_size is 12 (4096 entries)
    pub fn with_log_size(log_size: u32) -> Self {
        let log_size = log_size.min(12);
        let size = 1 << log_size;
        let mut data = Vec::with_capacity(size);
        data.resize_with(size, || None);

        Self {
            data,
            mask: size - 1,
        }
    }

    /// Get an object by hash code
    pub fn get_by_hash(&self, hash_code: usize) -> Option<&T> {
        self.data[hash_code & self.mask].as_ref()
    }

    /// Get or insert an object in the cache
    /// If the object exists and equals the provided value, return the cached version
    /// Otherwise, cache the new value and return it
    pub fn get_or_insert(&mut self, value: T) -> T {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        value.hash(&mut hasher);
        let hash_code = hasher.finish() as usize;
        let index = hash_code & self.mask;

        match &self.data[index] {
            Some(cached) if *cached == value => cached.clone(),
            _ => {
                self.data[index] = Some(value.clone());
                value
            }
        }
    }

    /// Put an object in the cache at the specified hash code
    pub fn put(&mut self, hash_code: usize, value: T) {
        self.data[hash_code & self.mask] = Some(value);
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        for slot in &mut self.data {
            *slot = None;
        }
    }

    /// Get the capacity of the cache
    pub fn capacity(&self) -> usize {
        self.data.len()
    }

    /// Check if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.data.iter().all(|slot| slot.is_none())
    }

    /// Get the number of cached items
    pub fn len(&self) -> usize {
        self.data.iter().filter(|slot| slot.is_some()).count()
    }
}

impl<T> Default for Cache<T>
where
    T: Clone + PartialEq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

/// A specialized string cache for common string operations
#[derive(Debug)]
pub struct StringCache {
    inner: Cache<String>,
}

impl StringCache {
    pub fn new() -> Self {
        Self {
            inner: Cache::new(),
        }
    }

    pub fn intern(&mut self, s: String) -> String {
        self.inner.get_or_insert(s)
    }

    pub fn intern_str(&mut self, s: &str) -> String {
        self.inner.get_or_insert(s.to_string())
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl Default for StringCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_new() {
        let cache: Cache<String> = Cache::new();
        assert_eq!(cache.capacity(), 256);
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_cache_with_log_size() {
        let cache: Cache<String> = Cache::with_log_size(4);
        assert_eq!(cache.capacity(), 16);

        let cache: Cache<String> = Cache::with_log_size(15); // Should be clamped to 12
        assert_eq!(cache.capacity(), 4096);
    }

    #[test]
    fn test_get_or_insert() {
        let mut cache = Cache::new();

        let value1 = "test".to_string();
        let cached1 = cache.get_or_insert(value1.clone());
        assert_eq!(cached1, value1);
        assert_eq!(cache.len(), 1);

        // Same value should return cached version
        let value2 = "test".to_string();
        let cached2 = cache.get_or_insert(value2);
        assert_eq!(cached2, value1);
        assert_eq!(cache.len(), 1); // Should not increase

        // Different value should be cached separately
        let value3 = "different".to_string();
        let cached3 = cache.get_or_insert(value3.clone());
        assert_eq!(cached3, value3);
        assert!(cache.len() >= 1); // Could be 1 or 2 depending on hash collision
    }

    #[test]
    fn test_put_and_get_by_hash() {
        let mut cache = Cache::new();
        let value = "test".to_string();
        cache.put(42, value.clone());

        let retrieved = cache.get_by_hash(42);
        assert_eq!(retrieved, Some(&value));

        let not_found = cache.get_by_hash(99);
        assert_eq!(not_found, None);
    }

    #[test]
    fn test_clear() {
        let mut cache = Cache::new();
        cache.put(1, "test1".to_string());
        cache.put(2, "test2".to_string());
        assert!(!cache.is_empty());

        cache.clear();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_string_cache() {
        let mut cache = StringCache::new();

        let s1 = cache.intern("hello".to_string());
        let s2 = cache.intern_str("hello");
        let s3 = cache.intern("world".to_string());

        assert_eq!(s1, "hello");
        assert_eq!(s2, "hello");
        assert_eq!(s3, "world");

        assert!(!cache.is_empty());
        cache.clear();
        assert!(cache.is_empty());
    }

    #[test]
    fn test_cache_with_integers() {
        let mut cache: Cache<i32> = Cache::new();

        let val1 = cache.get_or_insert(42);
        let val2 = cache.get_or_insert(42);
        let val3 = cache.get_or_insert(100);

        assert_eq!(val1, 42);
        assert_eq!(val2, 42);
        assert_eq!(val3, 100);

        assert!(cache.len() >= 1);
    }
}
