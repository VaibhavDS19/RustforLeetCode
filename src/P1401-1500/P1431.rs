impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut x = 0;
        let mut count:Vec<bool> = Vec::new();
        for i in candies.iter() {
            if *i>x {
                x=*i;
            }
        }
        for i in candies {
            count.push(i+extra_candies >= x);
        }
        count
    }
}