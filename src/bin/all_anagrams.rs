use std::collections::HashMap;
use std::hash::Hash;

pub struct Solution;

#[allow(unused_variables)]
impl Solution {
    pub fn find_anagrams_brute(s: String, p: String) -> Vec<i32> {

        let mut p_chars: HashMap<char, i32> = HashMap::new();
        
        for letter in p.chars() {
            p_chars.insert(letter, 1 + if p_chars.contains_key(&letter) { p_chars[&letter] } else { 0 });
        }

        // println!("{:?}", p_chars);

        let mut window: HashMap<char, i32> = HashMap::new();

        let s_len = s.len();
        let p_len = p.len();

        let mut res: Vec<i32> = vec![];

        for i in 0..s_len {
            let letter = s.chars().nth(i).unwrap();
            window.insert(letter, 1 + if window.contains_key(&letter) { window[&letter] } else { 0 });
            if i >= p_len {
                let first_letter = &s.chars().nth(i-p_len).unwrap();
                println!("{first_letter}");
                if window[first_letter] > 1 {
                    *window.get_mut(first_letter).unwrap() -= 1;
                } else {
                    window.remove(first_letter);
                }
            }

            println!("{:?}", p_chars);
            println!("{:?}", window);

            if Solution::keys_match(&p_chars, &window) {
                res.push((i+1-p_len) as i32);
            }
            println!("---------------");
        }

        res
    }


    fn keys_match<T: Eq + Hash, U: std::cmp::PartialEq>(
        map1: &HashMap<T, U>, 
        map2: &HashMap<T, U>,
    ) -> bool {
        map1.len() == map2.len() && map1.keys().all(|k| map2.get(k).as_deref() == map1.get(k))
    }

    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {

        let p_len = p.len();
        let s_len = s.len();

        let mut res: Vec<i32> = vec![];

        if s_len < p_len {
            return vec![];
        }

        let mut letters_matching = vec![0;27];
        p.chars().for_each(|letter| letters_matching[letter as usize - 97] += 1);

        let s_chars: Vec<char> = s.chars().collect();


        (0..p_len).for_each(|index| letters_matching[s_chars[index] as usize - 97] -= 1);

        if Solution::all_zeros(&letters_matching) { res.push(0) };
        // println!("{:?}", letters_matching);


        for index in p_len..s_len {
            letters_matching[s_chars[index] as usize - 97] -= 1;
            // println!("{}", s_chars[index - p_len]);
            letters_matching[s_chars[index - p_len] as usize - 97] += 1;

            // println!("{:?}", letters_matching);


            if Solution::all_zeros(&letters_matching) { res.push((index - p.len() + 1) as i32) };
        }

        // println!("{:?}", letters_matching);

        res
    }

    fn all_zeros(arr: &Vec<i32>) -> bool {
        arr.iter().all(|&x| x == 0)
    }
}


fn main() {
    let s = String::from("baa");
    let p = String::from("aa");
    println!("{:?}", Solution::find_anagrams(s, p));
}