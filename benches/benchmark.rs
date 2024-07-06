use std::collections::HashSet;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use seen_set::SeenSet;

fn create_strings(size: usize) -> Vec<String> {
  (0..1000)
    .map(|i| format!("{}{}", "1".repeat(size), i))
    .collect()
}

fn seen_set(strings: &[String]) {
  let mut set = SeenSet::with_capacity(strings.len());
  for string in strings {
    assert!(set.insert(string));
  }
  for string in strings {
    assert!(!set.insert(string));
  }
  for string in strings {
    assert!(set.contains(string));
  }
}

fn hash_set_clone(strings: &[String]) {
  let mut set: HashSet<String> = HashSet::with_capacity(strings.len());
  for string in strings {
    assert!(set.insert(string.clone()));
  }
  for string in strings {
    assert!(!set.insert(string.clone()));
  }
  for string in strings {
    assert!(set.contains(string));
  }
}

fn hash_set_ref(strings: &[String]) {
  let mut set: HashSet<&String> = HashSet::with_capacity(strings.len());
  for string in strings {
    assert!(set.insert(string));
  }
  for string in strings {
    assert!(!set.insert(string));
  }
  for string in strings {
    assert!(set.contains(string));
  }
}

fn criterion_benchmark(c: &mut Criterion) {
  for i in [1, 10, 100, 1_000, 10_000] {
    let strings = create_strings(i);
    let mut group = c.benchmark_group(i.to_string());
    group.bench_function("SeenSet", |b| b.iter(|| seen_set(&strings)));
    group.bench_function("HashSet (Clone)", |b| {
      b.iter(|| hash_set_clone(&strings))
    });
    group.bench_function("HashSet (Reference)", |b| {
      b.iter(|| hash_set_ref(&strings))
    });
  }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
