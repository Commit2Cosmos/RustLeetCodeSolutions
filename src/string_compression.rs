pub struct Solution;


impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        //* where to sub the change
        let mut point = 0;
        let (mut prev, mut count) = (chars[0], 1);

        for i in 1..chars.len() {
            if chars[i] == prev {
                count += 1;
            } else if count == 1 {
                chars[point] = prev;
                point += 1;
                prev = chars[i];
                count = 1;
            } else {
                chars[point] = prev;
                point += 1;

                for count_digit in count.to_string().chars() {
                    chars[point] = count_digit;
                    point += 1;
                    prev = chars[i];
                    count = 1;
                }
                
            }
        }

        if count == 1 {
            chars[point] = prev;
            point += 1;
        } else {
            chars[point] = prev;
            point += 1;
            
            for count_digit in count.to_string().chars() {
                chars[point] = count_digit;
                point += 1;
            }
            
        }

        chars.truncate(point);

        chars.len() as i32
    }
}