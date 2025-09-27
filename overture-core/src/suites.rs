// Sequence utilities for functional programming
// Equivalent to Swift's sequence operations

use std::collections::VecDeque;

/// Free `map` on sequences for function composition.
/// Equivalent to Swift's map<S: Sequence, A>(_ transform: @escaping (S.Element) -> A) -> (S) -> [A]
pub fn map<T, U, F>(transform: F) -> impl Fn(Vec<T>) -> Vec<U>
where
    F: Fn(T) -> U + 'static,
{
    move |sequence: Vec<T>| sequence.into_iter().map(&transform).collect()
}

/// Free `map` on sequences for throwing function composition.
/// Equivalent to Swift's map<S: Sequence, A>(_ transform: @escaping (S.Element) throws -> A) -> (S) throws -> [A]
pub fn map_throwing<T, U, E, F>(transform: F) -> impl Fn(Vec<T>) -> Result<Vec<U>, E>
where
    F: Fn(T) -> Result<U, E> + 'static,
{
    move |sequence: Vec<T>| {
        let mut result = Vec::with_capacity(sequence.len());
        for item in sequence {
            result.push(transform(item)?);
        }
        Ok(result)
    }
}

/// In-place collection mutation.
/// Equivalent to Swift's mutEach<C: MutableCollection>(_ transform: @escaping (inout C.Element) -> Void) -> (inout C) -> Void
pub fn mut_each<T, F>(transform: F) -> impl Fn(&mut Vec<T>)
where
    F: Fn(&mut T) + 'static,
{
    move |collection: &mut Vec<T>| {
        for item in collection.iter_mut() {
            transform(item);
        }
    }
}

/// Transforms a pair of sequences into a sequence of pairs.
/// Equivalent to Swift's zip<A, B, Z>(with transform: (A.Element, B.Element) -> Z, _ sequence1: A, _ sequence2: B) -> [Z]
pub fn zip_with<T, U, V, F>(transform: F) -> impl Fn(Vec<T>, Vec<U>) -> Vec<V>
where
    F: Fn(T, U) -> V + 'static,
{
    move |sequence1: Vec<T>, sequence2: Vec<U>| {
        sequence1
            .into_iter()
            .zip(sequence2.into_iter())
            .map(|(a, b)| transform(a, b))
            .collect()
    }
}

/// Basic zip operation for two sequences.
pub fn zip<T, U>(sequence1: Vec<T>, sequence2: Vec<U>) -> Vec<(T, U)> {
    sequence1.into_iter().zip(sequence2.into_iter()).collect()
}

/// Filter operation for sequences.
pub fn filter<T, F>(predicate: F) -> impl Fn(Vec<T>) -> Vec<T>
where
    F: Fn(&T) -> bool + 'static,
{
    move |sequence: Vec<T>| sequence.into_iter().filter(&predicate).collect()
}

/// Filter operation for sequences with error handling.
pub fn filter_throwing<T, E, F>(predicate: F) -> impl Fn(Vec<T>) -> Result<Vec<T>, E>
where
    F: Fn(&T) -> Result<bool, E> + 'static,
{
    move |sequence: Vec<T>| {
        let mut result = Vec::new();
        for item in sequence {
            if predicate(&item)? {
                result.push(item);
            }
        }
        Ok(result)
    }
}

/// Reduce operation for sequences.
pub fn reduce<T, F>(initial: T, operation: F) -> impl Fn(Vec<T>) -> T
where
    T: Clone + 'static,
    F: Fn(T, T) -> T + 'static,
{
    move |sequence: Vec<T>| {
        sequence
            .into_iter()
            .fold(initial.clone(), |acc, item| operation(acc, item))
    }
}

/// Flat map operation for sequences.
pub fn flat_map<T, U, F>(transform: F) -> impl Fn(Vec<T>) -> Vec<U>
where
    F: Fn(T) -> Vec<U> + 'static,
{
    move |sequence: Vec<T>| sequence.into_iter().flat_map(&transform).collect()
}

/// Flat map operation for sequences with error handling.
pub fn flat_map_throwing<T, U, E, F>(transform: F) -> impl Fn(Vec<T>) -> Result<Vec<U>, E>
where
    F: Fn(T) -> Result<Vec<U>, E> + 'static,
{
    move |sequence: Vec<T>| {
        let mut result = Vec::new();
        for item in sequence {
            result.extend(transform(item)?);
        }
        Ok(result)
    }
}

/// Compact map operation (filter out None values).
pub fn compact_map<T, U, F>(transform: F) -> impl Fn(Vec<Option<T>>) -> Vec<U>
where
    F: Fn(T) -> U + 'static,
{
    move |sequence: Vec<Option<T>>| {
        sequence
            .into_iter()
            .filter_map(|item| item.map(&transform))
            .collect()
    }
}

/// Compact map operation with error handling.
pub fn compact_map_throwing<T, U, E, F>(
    transform: F,
) -> impl Fn(Vec<Option<T>>) -> Result<Vec<U>, E>
where
    F: Fn(T) -> Result<U, E> + 'static,
{
    move |sequence: Vec<Option<T>>| {
        let mut result = Vec::new();
        for item in sequence {
            if let Some(value) = item {
                result.push(transform(value)?);
            }
        }
        Ok(result)
    }
}

/// Partition operation for sequences.
pub fn partition<T, F>(predicate: F) -> impl Fn(Vec<T>) -> (Vec<T>, Vec<T>)
where
    F: Fn(&T) -> bool + 'static,
{
    move |sequence: Vec<T>| {
        let mut true_vec = Vec::new();
        let mut false_vec = Vec::new();

        for item in sequence {
            if predicate(&item) {
                true_vec.push(item);
            } else {
                false_vec.push(item);
            }
        }

        (true_vec, false_vec)
    }
}

/// Group by operation for sequences.
pub fn group_by<T, K, F>(key_selector: F) -> impl Fn(Vec<T>) -> std::collections::HashMap<K, Vec<T>>
where
    T: Clone + 'static,
    K: std::hash::Hash + Eq + 'static,
    F: Fn(&T) -> K + 'static,
{
    move |sequence: Vec<T>| {
        let mut groups = std::collections::HashMap::new();
        for item in sequence {
            let key = key_selector(&item);
            groups.entry(key).or_insert_with(Vec::new).push(item);
        }
        groups
    }
}

/// Chunk operation for sequences.
pub fn chunk<T>(size: usize) -> impl Fn(Vec<T>) -> Vec<Vec<T>>
where
    T: Clone + 'static,
{
    move |sequence: Vec<T>| sequence.chunks(size).map(|chunk| chunk.to_vec()).collect()
}

/// Window operation for sequences.
pub fn window<T>(size: usize) -> impl Fn(Vec<T>) -> Vec<Vec<T>>
where
    T: Clone + 'static,
{
    move |sequence: Vec<T>| {
        if sequence.len() < size {
            return Vec::new();
        }

        let mut windows = Vec::new();
        for i in 0..=sequence.len() - size {
            windows.push(sequence[i..i + size].to_vec());
        }
        windows
    }
}

/// Take operation for sequences.
pub fn take<T>(count: usize) -> impl Fn(Vec<T>) -> Vec<T>
where
    T: Clone + 'static,
{
    move |sequence: Vec<T>| sequence.into_iter().take(count).collect()
}

/// Skip operation for sequences.
pub fn skip<T>(count: usize) -> impl Fn(Vec<T>) -> Vec<T>
where
    T: Clone + 'static,
{
    move |sequence: Vec<T>| sequence.into_iter().skip(count).collect()
}

/// Take while operation for sequences.
pub fn take_while<T, F>(predicate: F) -> impl Fn(Vec<T>) -> Vec<T>
where
    F: Fn(&T) -> bool + 'static,
{
    move |sequence: Vec<T>| sequence.into_iter().take_while(&predicate).collect()
}

/// Skip while operation for sequences.
pub fn skip_while<T, F>(predicate: F) -> impl Fn(Vec<T>) -> Vec<T>
where
    F: Fn(&T) -> bool + 'static,
{
    move |sequence: Vec<T>| sequence.into_iter().skip_while(&predicate).collect()
}

/// Distinct operation for sequences.
pub fn distinct<T>(sequence: Vec<T>) -> Vec<T>
where
    T: Clone + std::hash::Hash + Eq,
{
    let mut seen = std::collections::HashSet::new();
    sequence
        .into_iter()
        .filter(|item| seen.insert(item.clone()))
        .collect()
}

/// Sort operation for sequences.
pub fn sort<T, F>(compare: F) -> impl Fn(Vec<T>) -> Vec<T>
where
    T: Clone + 'static,
    F: Fn(&T, &T) -> std::cmp::Ordering + 'static,
{
    move |mut sequence: Vec<T>| {
        sequence.sort_by(&compare);
        sequence
    }
}

/// Reverse operation for sequences.
pub fn reverse<T>(sequence: Vec<T>) -> Vec<T>
where
    T: Clone,
{
    let mut result = sequence;
    result.reverse();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let numbers = vec![1, 2, 3, 4, 5];
        let double = map(|x: i32| x * 2);
        let result = double(numbers);
        assert_eq!(result, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_map_throwing() {
        let numbers = vec![1, 2, 3, 4, 5];
        let safe_divide = map_throwing(|x: i32| {
            if x == 0 {
                Err("Division by zero")
            } else {
                Ok(10 / x)
            }
        });
        let result = safe_divide(numbers);
        assert_eq!(result, Ok(vec![10, 5, 3, 2, 2]));
    }

    #[test]
    fn test_mut_each() {
        let mut numbers = vec![1, 2, 3, 4, 5];
        let double = mut_each(|x: &mut i32| *x *= 2);
        double(&mut numbers);
        assert_eq!(numbers, vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_zip_with() {
        let numbers1 = vec![1, 2, 3];
        let numbers2 = vec![10, 20, 30];
        let add = zip_with(|a: i32, b: i32| a + b);
        let result = add(numbers1, numbers2);
        assert_eq!(result, vec![11, 22, 33]);
    }

    #[test]
    fn test_filter() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let is_even = filter(|x: &i32| x % 2 == 0);
        let result = is_even(numbers);
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[test]
    fn test_reduce() {
        let numbers = vec![1, 2, 3, 4, 5];
        let sum = reduce(0, |acc: i32, x: i32| acc + x);
        let result = sum(numbers);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_flat_map() {
        let numbers = vec![1, 2, 3];
        let expand = flat_map(|x: i32| vec![x, x * 2]);
        let result = expand(numbers);
        assert_eq!(result, vec![1, 2, 2, 4, 3, 6]);
    }

    #[test]
    fn test_compact_map() {
        let numbers = vec![Some(1), None, Some(3), None, Some(5)];
        let double = compact_map(|x: i32| x * 2);
        let result = double(numbers);
        assert_eq!(result, vec![2, 6, 10]);
    }

    #[test]
    fn test_partition() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let is_even = partition(|x: &i32| x % 2 == 0);
        let (evens, odds) = is_even(numbers);
        assert_eq!(evens, vec![2, 4, 6]);
        assert_eq!(odds, vec![1, 3, 5]);
    }

    #[test]
    fn test_group_by() {
        let words = vec!["apple", "banana", "apricot", "cherry", "avocado"];
        let by_first_letter = group_by(|word: &&str| word.chars().next().unwrap());
        let groups = by_first_letter(words);
        assert_eq!(groups.get(&'a').unwrap().len(), 3);
        assert_eq!(groups.get(&'b').unwrap().len(), 1);
        assert_eq!(groups.get(&'c').unwrap().len(), 1);
    }

    #[test]
    fn test_chunk() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let chunk_by_3 = chunk(3);
        let result = chunk_by_3(numbers);
        assert_eq!(result, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8]]);
    }

    #[test]
    fn test_take() {
        let numbers = vec![1, 2, 3, 4, 5];
        let take_3 = take(3);
        let result = take_3(numbers);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_skip() {
        let numbers = vec![1, 2, 3, 4, 5];
        let skip_2 = skip(2);
        let result = skip_2(numbers);
        assert_eq!(result, vec![3, 4, 5]);
    }

    #[test]
    fn test_distinct() {
        let numbers = vec![1, 2, 2, 3, 3, 3, 4];
        let result = distinct(numbers);
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_sort() {
        let numbers = vec![3, 1, 4, 1, 5];
        let ascending = sort(|a: &i32, b: &i32| a.cmp(b));
        let result = ascending(numbers);
        assert_eq!(result, vec![1, 1, 3, 4, 5]);
    }

    #[test]
    fn test_reverse() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = reverse(numbers);
        assert_eq!(result, vec![5, 4, 3, 2, 1]);
    }
}
