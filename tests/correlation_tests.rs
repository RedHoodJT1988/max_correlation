use max_correlation::algorithms::correlation::max_array_correlation;

#[test]
fn test_max_array_correlation() {
    assert_eq!(max_array_correlation(vec![3, 1, 4], vec![2, 5, 6]), 11);
    assert_eq!(max_array_correlation(vec![1, 2, 3], vec![4, 5, 6]), 15);
    assert_eq!(max_array_correlation(vec![5, 6, 7], vec![1, 2, 3]), 0);
    assert_eq!(max_array_correlation(vec![2, 2, 2], vec![2, 2, 2]), 0);
    assert_eq!(max_array_correlation(vec![5], vec![10]), 10);
}
