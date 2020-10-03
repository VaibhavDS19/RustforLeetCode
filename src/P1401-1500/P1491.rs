impl Solution {
   pub fn average(salary: Vec<i32>) -> f64 {
       let mut maxs = salary[0];
       let mut mins = salary[0];
       let mut sum = 0;
       for i in &salary {
           sum += i;
           if(*i>maxs) { maxs=*i; }
           else if(*i<mins) { mins=*i; }
       }
       (sum-maxs-mins) as f64/(salary.len()-2) as f64
   }
}