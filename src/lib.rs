use std::collections::HashSet;
use std::hash::BuildHasher;
use std::hash::RandomState;
use std::marker::PhantomData;

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
  THashHasher = RandomState,
> {
  _kind: PhantomData<T>,
  value_hasher: TValueHasher,
  checked: HashSet<u64, THashHasher>,
}

impl<T: std::hash::Hash + ?Sized, TValueHasher: Clone, THashHasher: Clone> Clone
  for SeenSet<T, TValueHasher, THashHasher>
{
  fn clone(&self) -> Self {
    Self {
      _kind: Default::default(),
      value_hasher: self.value_hasher.clone(),
      checked: self.checked.clone(),
    }
  }
}

impl<T: std::hash::Hash + ?Sized, TValueHasher, THashHasher> std::fmt::Debug
  for SeenSet<T, TValueHasher, THashHasher>
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("CheckedSet")
      .field("checked", &self.checked)
      .finish()
  }
}

impl<
    T: std::hash::Hash + ?Sized,
    TValueHasher: Default,
    THashHasher: Default,
  > Default for SeenSet<T, TValueHasher, THashHasher>
{
  fn default() -> Self {
    Self {
      _kind: Default::default(),
      value_hasher: Default::default(),
      checked: Default::default(),
    }
  }
}

impl<T: std::hash::Hash + ?Sized> SeenSet<T, RandomState> {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn with_capacity(capacity: usize) -> Self {
    Self::with_capacity_and_value_hasher(capacity, Default::default())
  }
}

impl<T: std::hash::Hash + ?Sized, TValueHasher> SeenSet<T, TValueHasher> {
  pub fn with_capacity_and_value_hasher(
    capacity: usize,
    value_hasher: TValueHasher,
  ) -> Self {
    Self::with_capacity_and_hashers(capacity, value_hasher, Default::default())
  }
}

impl<T: std::hash::Hash + ?Sized, TValueHasher, THashHasher>
  SeenSet<T, TValueHasher, THashHasher>
{
  pub fn with_capacity_and_hashers(
    capacity: usize,
    value_hasher: TValueHasher,
    hash_hasher: THashHasher,
  ) -> Self {
    Self {
      _kind: PhantomData,
      value_hasher,
      checked: HashSet::with_capacity_and_hasher(capacity, hash_hasher),
    }
  }
}

impl<
    T: std::hash::Hash + ?Sized,
    TValueHasher: BuildHasher,
    THashHasher: BuildHasher,
  > SeenSet<T, TValueHasher, THashHasher>
{
  /// Check if the set contains the value.
  pub fn contains(&mut self, value: &T) -> bool {
    self.checked.contains(&self.get_hash(value))
  }

  /// Inserts a hash of the value into the set returning `true`
  /// if never seen before or `false` otherwise.
  pub fn insert(&mut self, value: &T) -> bool {
    self.checked.insert(self.get_hash(value))
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
