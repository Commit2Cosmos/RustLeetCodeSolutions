use lazy_static::*;

pub struct Solution;


impl Solution {
    //* FINDING A SQUARE OF A PRIME -> SPECIAL NUMBER */
    pub fn non_special_count(l: i32, r: i32) -> i32 {

        lazy_static! {
            static ref primes: Vec<i32> = {
                //* cz 10^9 is the largest possible r */
                const BIG: usize = 31623;

                let mut pr = vec![0; BIG+1];

                for i in 2..=BIG {
                    if pr[i]==0 {
                        pr[i] = pr[i-1] + 1;
                        for j in ((i * i)..=BIG).step_by(i) {
                            pr[j] = -1;
                        } 
                    } else {
                        pr[i] = pr[i-1];
                    }
                }

                pr
            };
        }
        
        r-l+primes[(l as f32).sqrt().ceil() as usize-1]-primes[(r as f32).sqrt().floor() as usize]+1
    }
}