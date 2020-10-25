impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut n=0;
        for log in logs.iter() {
            if log!="./" {
                if log=="../" {
                    if n>0 {
                        n=n-1;
                    }
                }
                else {
                    n=n+1
                }
            }
        }
        if n<0 {
            n=0;
        }
        n
    }
}