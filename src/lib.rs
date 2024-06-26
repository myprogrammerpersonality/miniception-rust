pub fn order_kmer(kmer: &str) -> usize {
    let mut order: usize = 0;
    let mut factor: usize = 1;
    for i in (0..kmer.len()).rev() {
        let c = kmer.chars().nth(i).unwrap();
        order += factor * match c {
            'A' => 0,
            'C' => 1,
            'G' => 2,
            'T' => 3,
            _ => panic!("Invalid character in k-mer"),
        };
        factor *= 4;
    }
    order
}

pub fn minimizers_locations(s: &str, w: usize, k: usize) -> Vec<usize> {
    let mut minimizers: Vec<usize> = Vec::new();

    for i in 0..s.len() - (w + k - 1) + 1 {
        let mut minimizer_order = order_kmer(&s[i..i + k]);
        let mut minimizer_pos = i;
        for j in 0..w {
            let kmer_order = order_kmer(&s[i + j..i + j + k]);
            log::debug!("i {} j {} k-mer {}, k-mer order {} minimizer order {}", i, j, &s[i + j..i + j + k], kmer_order, minimizer_order);
            if kmer_order < minimizer_order {
                minimizer_order = kmer_order;
                minimizer_pos = i + j;
            }
        }
        minimizers.push(minimizer_pos);
    }
    minimizers
}
