//! Small exercises.

use std::collections::HashMap;

use itertools::Itertools;

/// Returns whether the given sequence is a fibonacci sequence starts from the given sequence's
/// first two terms.
///
/// Returns `true` if the length of sequence is less or equal than 2.
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment09::small_exercises::*;
///
/// assert_eq!(is_fibonacci([1, 1, 2, 3, 5, 8, 13].into_iter()), true);
/// assert_eq!(is_fibonacci([1, 1, 2, 3, 5, 8, 14].into_iter()), false);
/// ```
pub fn is_fibonacci(mut inner: impl Iterator<Item = i64>) -> bool {
    let Some(mut first) = inner.next() else {
        return true;
    };
    let Some(mut second) = inner.next() else {
        return true;
    };

    for val in inner {
        if val != (first + second) {
            return false;
        } else {
            first = second;
            second = val;
        }
    }
    true
}

/// Returns the sum of `f(v)` for all element `v` the given array.
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment09::small_exercises::*;
///
/// assert_eq!(sigma([1, 2].into_iter(), |x| x + 2), 7);
/// assert_eq!(sigma([1, 2].into_iter(), |x| x * 4), 12);
/// ```
pub fn sigma<T, F: Fn(T) -> i64>(inner: impl Iterator<Item = T>, f: F) -> i64 {
    inner.map(f).sum()
}

/// Alternate elements from three iterators until they have run out.
///
/// You can assume that the number of elements of three iterators are same.
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment09::small_exercises::*;
///
/// assert_eq!(
///     interleave3([1, 2].into_iter(), [3, 4].into_iter(), [5, 6].into_iter()),
///     vec![1, 3, 5, 2, 4, 6]
/// );
/// ```
pub fn interleave3<T>(
    list1: impl Iterator<Item = T>,
    list2: impl Iterator<Item = T>,
    list3: impl Iterator<Item = T>,
) -> Vec<T> {
    let mut ret: Vec<T> = Vec::new();
    for (a, b, c) in itertools::multizip((list1, list2, list3)) {
        ret.push(a);
        ret.push(b);
        ret.push(c);
    }
    ret
}

/// Alternate elements from array of n iterators until they have run out.
///
/// You can assume that the number of elements of iterators are same.
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment09::small_exercises::*;
///
/// assert_eq!(
///     interleave_n(&mut [[1, 2].into_iter(), [3, 4].into_iter(), [5, 6].into_iter()]),
///     vec![1, 3, 5, 2, 4, 6]
/// );
/// ```
pub fn interleave_n<T, const N: usize>(
    mut iters: [impl Iterator<Item = T>; N],
) -> impl Iterator<Item = T> {
    let mut ret: Vec<T> = Vec::new();
    let len = iters[0].size_hint().0;
    let mut count = 0;
    'a: loop {
        for iter in &mut iters {
            if count < len {
                ret.push(iter.next().unwrap());
            } else {
                break 'a;
            }
        }
        count += 1;
    }

    ret.into_iter()
}

/// Returns mean of k smallest value's mean.
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment09::small_exercises::*;
///
/// assert_eq!(
///     k_smallest_mean(vec![1, 3, 2].into_iter(), 2),
///     ((1 + 2) as f64 / 2.0)
/// );
/// assert_eq!(
///     k_smallest_mean(vec![7, 5, 3, 6].into_iter(), 3),
///     ((3 + 5 + 6) as f64 / 3.0)
/// );
/// ```
pub fn k_smallest_mean(inner: impl Iterator<Item = i64>, k: usize) -> f64 {
    let mut result: Vec<i64> = inner.collect();
    result.sort();
    let sum: i64 = result.iter().take(k).sum();
    sum as f64 / k as f64
}

/// Returns mean for each class.
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment09::small_exercises::*;
///
/// assert_eq!(
///     calculate_mean(
///         [
///             ("CS100".to_string(), 60),
///             ("CS200".to_string(), 60),
///             ("CS200".to_string(), 80),
///             ("CS300".to_string(), 100),
///         ]
///         .into_iter()
///     ),
///     [
///         ("CS100".to_string(), 60.0),
///         ("CS200".to_string(), 70.0),
///         ("CS300".to_string(), 100.0)
///     ]
///     .into_iter()
///     .collect()
/// );
/// ```
pub fn calculate_mean(inner: impl Iterator<Item = (String, i64)>) -> HashMap<String, f64> {
    let mut ret: HashMap<String, (i64, usize)> = HashMap::new();
    for (s, v) in inner {
        let entry = ret.entry(s).or_insert((0, 0));
        entry.0 += v;
        entry.1 += 1;
    }

    ret.into_iter()
        .map(|(s, (sum, count))| (s, sum as f64 / count as f64))
        .collect()
}

/// Among the cartesian product of input vectors, return the number of sets whose sum equals `n`.
///
/// # Example
///
/// The cartesian product of [1, 2, 3] and [2, 3] are:
///     [1, 2], [1, 3], [2, 2], [2, 3], [3, 2], [3, 3].
///
/// Among these sets, the number of sets whose sum is 4 is 2, which is [1, 3] and [2, 2].
///
/// ```
/// use cs220::assignments::assignment09::small_exercises::*;
///
/// assert_eq!(sum_is_n(vec![vec![1, 2, 3], vec![2, 3]], 3), 1);
/// assert_eq!(sum_is_n(vec![vec![1, 2, 3], vec![2, 3]], 4), 2);
/// assert_eq!(sum_is_n(vec![vec![1, 2, 3], vec![2, 3]], 5), 2);
/// assert_eq!(sum_is_n(vec![vec![1, 2, 3], vec![2, 3]], 6), 1);
/// assert_eq!(sum_is_n(vec![vec![1, 2, 3], vec![2, 3]], 2), 0);
/// ```
pub fn sum_is_n(inner: Vec<Vec<i64>>, n: i64) -> usize {
    let mut count = 0;
    for m in &inner[0] {
        for i in &inner[1] {
            if m + i == n {
                count += 1;
            }
        }
    }
    count
}

/// Returns a new vector that contains the item that appears `n` times in the input vector in
/// increasing order.
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment09::small_exercises::*;
///
/// assert_eq!(find_count_n(vec![1, 2], 1), vec![1, 2]);
/// assert_eq!(find_count_n(vec![1, 3, 3], 1), vec![1]);
/// assert_eq!(find_count_n(vec![1, 3, 3], 2), vec![3]);
/// assert_eq!(find_count_n(vec![1, 2, 3, 4, 4], 1), vec![1, 2, 3]);
/// ```
pub fn find_count_n(inner: Vec<usize>, n: usize) -> Vec<usize> {
    let mut count = HashMap::new();

    for num in inner {
        *count.entry(num).or_insert(0) += 1;
    }

    let mut ret: Vec<usize> = count
        .into_iter()
        .filter_map(|(k, v)| if v == n { Some(k) } else { None })
        .collect();
    ret.sort();
    ret
}

/// Return the position of the median element in the vector.
///
/// For a data set `x` of `n` elements, the median can be defined as follows:
///
/// - If `n` is odd, the median is `(n+1)/2`-th smallest element of `x`.
/// - If `n` is even, the median is `(n/2)+1`-th smallest element of `x`.
///
/// Please following these rules:
///
/// - If the list is empty, returns `None`.
/// - If several elements are equally median, the position of the first of them is returned.
///
/// # Example
///
/// ```
/// use cs220::assignments::assignment09::small_exercises::*;
///
/// assert_eq!(position_median(vec![1, 3, 3, 6, 7, 8, 9]), Some(3));
/// assert_eq!(position_median(vec![1, 3, 3, 3]), Some(1));
/// ```
pub fn position_median<T: Ord + Clone + PartialEq>(inner: Vec<T>) -> Option<usize> {
    if inner.is_empty() {
        return None;
    }
    let mut sorted = inner.clone();
    sorted.sort();

    let n = sorted.len();
    let median = if n % 2 == 0 { n / 2 } else { (n - 1) / 2 };

    let val = &sorted[median];
    inner.iter().position(|x| x == val)
}

/// Returns the sum of all elements in a two-dimensional array.
///
/// # Example
/// ```
/// use cs220::assignments::assignment09::small_exercises::*;
///
/// assert_eq!(
///     two_dimensional_sum([[1, 2, 3].into_iter(), [4, 5, 6].into_iter()].into_iter()),
///     21
/// );
/// ```
pub fn two_dimensional_sum(inner: impl Iterator<Item = impl Iterator<Item = i64>>) -> i64 {
    let mut ret: i64 = 0;
    for iter in inner {
        ret += iter.sum::<i64>();
    }
    ret
}

/// Returns whether the given string is palindrome or not.
///
/// A palindrome is a word, number, phrase, or other sequence of characters which reads the same
/// backward as forward.
///
/// We consider the empty string as a palindrome.
///
/// Consult <https://en.wikipedia.org/wiki/Palindrome>.
pub fn is_palindrome(s: String) -> bool {
    let mut iter = s.chars();
    while let (Some(front), Some(back)) = (iter.next(), iter.next_back()) {
        if front != back {
            return false;
        }
    }
    true
}
