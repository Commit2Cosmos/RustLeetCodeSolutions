pub struct Solution;


impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        const ONES : [&str;10] = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
        const TENS : [&str;10] = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
        const CENT : [&str;10] = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
        const MILS : [&str;4]  = ["", "M", "MM", "MMM"];



        format!("{}{}{}{}",
            MILS[(num/1000) as usize],
            CENT[((num/100) % 10) as usize],
            TENS[((num/10) % 10) as usize],
            ONES[((num) % 10) as usize],
        )
    }
}