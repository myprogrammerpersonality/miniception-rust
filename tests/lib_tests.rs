use miniception_rust::{order_kmer, minimizers_locations};

#[test]
fn test_order_kmer() {
    assert_eq!(order_kmer("AAAA"), 0);
    assert_eq!(order_kmer("AAAC"), 1);
    assert_eq!(order_kmer("AAAG"), 2);
    assert_eq!(order_kmer("CA"), 4);
    assert_eq!(order_kmer("TG"), 14);
    assert_eq!(order_kmer("AC"), 1);
    assert_eq!(order_kmer("AA"), 1000);
}

#[test]
fn test_minimizers_locations() {
    let s = "TTAAAATAAAA";
    let w = 3;
    let k = 4;
    let minimizers: Vec<usize> = minimizers_locations(s, w, k);
    assert_eq!(minimizers, vec![2, 2, 2, 3, 4, 7]);
}
