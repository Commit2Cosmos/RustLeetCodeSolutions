pub struct Solution;


impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        let mut map1 = vec![0; 26];
        let mut map2 = vec![0; 26];

        word1.bytes().for_each(|x| map1[x as usize - 'a' as usize] += 1);
        word2.bytes().for_each(|x| map2[x as usize - 'a' as usize] += 1);
        
        for (&x1, &x2) in map1.iter().zip(map2.iter()) {
            if (x1==0) != (x2==0) {
                return false;
            }
        }

        map1.sort_unstable();
        map2.sort_unstable();

        map1 == map2
    }
}