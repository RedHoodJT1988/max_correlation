/// Computes the maximum array correlation by rearranging `b`
/// to maximize the sum of all `b[i]` where `b[i] > a[i]`.
pub fn max_array_correlation(mut a: Vec<i32>, mut b: Vec<i32>) -> i32 {
    a.sort(); // Sort a in ascending order
    b.sort_by(|x, y| y.cmp(x)); // Sort b in descending order

    a.iter().zip(b.iter()).filter(|(&ai, &bi)| bi > ai).map(|(_, &bi)| bi).sum()
}
