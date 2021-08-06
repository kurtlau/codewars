struct Tribonacci {
    signature: [f64; 3],
}

impl Tribonacci {
    fn new(signature: &[f64; 3]) -> Self {
        Tribonacci {
            signature: signature.clone(),
        }
    }
}

impl Iterator for Tribonacci {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.signature[0];
        let next = self.signature.iter().sum();
        self.signature[0] = self.signature[1];
        self.signature[1] = self.signature[2];
        self.signature[2] = next;
        Some(curr)
    }
}

fn tribonacci(signature: &[f64; 3], n: usize) -> Vec<f64> {
    Tribonacci::new(signature).into_iter().take(n).collect()
}
