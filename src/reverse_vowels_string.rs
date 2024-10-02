pub struct Solution;


impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut vowels = s.chars().filter(|x| {
            x.is_alphabetic() && ['a', 'e', 'i', 'o', 'u'].contains(&x.to_ascii_lowercase())
        })
        .rev();

        s.chars().map(|x| {
            match x.to_ascii_lowercase() {
                'a' | 'e' | 'i' | 'o' | 'u' => vowels.next().unwrap(),
                _ => x
            }
        }).collect()
    }
}