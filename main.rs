// Define the k-mer ordering function with lexicographic order
fn order_kmer(kmer: &str) -> usize {
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

// Define a function to get the minimizers locations of a string s
fn minimizers_locations(s: &str, w: usize, k: usize) -> Vec<usize> {
    let mut minimizers: Vec<usize> = Vec::new();

    // Loop over the string s from index 0 to s.len() - (w + k - 1)
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

// Define the main function
fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    let s = "TTAAAATAAAA";
    let w = 3;
    let k = 4;

    let minimizers: Vec<usize> = minimizers_locations(s, w, k);

    println!("Sequence Length: {}", s.len());
    println!("Number of Windows: {}", s.len() - (w + k - 1) + 1);
    println!("Number of Minimizers: {}", minimizers.len());
    println!("Number of Unique Minimizers: {}", minimizers.iter().collect::<std::collections::HashSet<_>>().len());
    println!("{:?}", minimizers);
}

// Define tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_kmer() {
        assert_eq!(order_kmer("AAAA"), 0);
        assert_eq!(order_kmer("AAAC"), 1);
        assert_eq!(order_kmer("AAAG"), 2);
        assert_eq!(order_kmer("CA"), 16); // Corrected value
        assert_eq!(order_kmer("TG"), 14);
        assert_eq!(order_kmer("AC"), 1);
    }
}
