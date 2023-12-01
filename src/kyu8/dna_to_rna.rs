// DNA to RNA Conversion
// https://www.codewars.com/kata/5556282156230d0e5e000089

#[allow(dead_code)]
fn dna_to_rna(dna: &str) -> String {
    dna.replace("T", "U")
}

#[allow(dead_code)]
pub fn dna_to_rna_test() {
    println!("{}", dna_to_rna("CGAT"));
}
