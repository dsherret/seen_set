use std::hash::BuildHasher;
use std::hash::RandomState;
use std::marker::PhantomData;

use hashbrown::HashTable;

/// A HashSet that doesn't store values, but instead only stores hashes.
///
/// This is useful when you only need to tell if you've seen a value before
/// and you don't want to clone the value for performance reasons.
///
/// ```rs
/// let mut seen = SeenSet::new();
/// for path in some_func_that_may_return_duplicate_paths() {
///   if !seen.insert(&path) {
///     continue;
///   }
///   // we haven't seen this path before
/// }
/// ```
///
/// Note: This will be slower for values like numbers. Just use a regular
/// HashSet for that.
pub struct SeenSet<
  T: std::hash::Hash + ?Sized,
  TValueHasher = RandomState,
> {
  _kind: PhantomData<T>,
  value_hasher: TValueHasher,
  checked_table: hashbrown::HashTable<u64>,
}

impl<T: std::hash::Hash + ?Sized, TValueHasher: Clone> Clone
  for SeenSet<T, TValueHasher>
{
  fn clone(&self) -> Self {
    Self {
      _kind: Default::default(),
      value_hasher: self.value_hasher.clone(),
      checked_table: self.checked_table.clone(),
    }
  }
}

impl<T: std::hash::Hash + ?Sized, TValueHasher> std::fmt::Debug
  for SeenSet<T, TValueHasher>
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("CheckedSet")
      .field("checked", &self.checked_table)
      .finish()
  }
}

impl<
    T: std::hash::Hash + ?Sized,
    TValueHasher: Default,
  > Default for SeenSet<T, TValueHasher>
{
  fn default() -> Self {
    Self {
      _kind: Default::default(),
      value_hasher: Default::default(),
      checked_table: Default::default(),
    }
  }
}

impl<T: std::hash::Hash + ?Sized> SeenSet<T, RandomState> {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn with_capacity(capacity: usize) -> Self {
    Self::with_capacity_and_hasher(capacity, Default::default())
  }
}

impl<T: std::hash::Hash + ?Sized, TValueHasher> SeenSet<T, TValueHasher> {
  pub fn with_capacity_and_hasher(
    capacity: usize,
    value_hasher: TValueHasher,
  ) -> Self {
    Self {
      _kind: PhantomData,
      value_hasher,
      checked_table: HashTable::with_capacity(capacity),
    }
  }
}

impl<
    T: std::hash::Hash + ?Sized,
    TValueHasher: BuildHasher,
  > SeenSet<T, TValueHasher>
{
  /// Check if the set contains the value.
  pub fn contains(&mut self, value: &T) -> bool {
    let hash = self.get_hash(value);
    let hasher = |val: &_| self.value_hasher.hash_one(val);
    match self.checked_table.entry(hash, |key| key == &hash, hasher) {
      hashbrown::hash_table::Entry::Occupied(_) => true,
      hashbrown::hash_table::Entry::Vacant(_) => false,
    }
  }

  /// Inserts a hash of the value into the set returning `true`
  /// if never seen before or `false` otherwise.
  pub fn insert(&mut self, value: &T) -> bool {
    let hash = self.get_hash(value);
    let hasher = |val: &_| self.value_hasher.hash_one(val);
    match self.checked_table.entry(hash, |key| key == &hash, hasher) {
      hashbrown::hash_table::Entry::Occupied(_) => false,
      hashbrown::hash_table::Entry::Vacant(entry) => {
        entry.insert(hash);
        true
      },
    }
  }

  #[inline]
  fn get_hash(&self, value: &T) -> u64 {
    self.value_hasher.hash_one(value)
  }
}

#[cfg(test)]
mod test {
  #[test]
  fn test_seen_set() {
    // Obviously don't use this with numbers because it will be slower
    // than a regular HashSet. This is just for testing.
    let mut seen = super::SeenSet::new();
    assert!(seen.insert(&1));
    assert!(!seen.insert(&1));
    assert!(seen.insert(&2));
    assert!(!seen.insert(&2));
    assert!(seen.contains(&2));
    assert!(!seen.contains(&3));
  }
}
