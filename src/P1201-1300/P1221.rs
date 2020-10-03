impl Solution {
   pub fn balanced_string_split(s: String) -> i32 {
       let mut x:i32=0;
       let mut count=0;
       for c in s.chars(){
           if c=='L'{
               x+=1;
           }
           else{
               x-=1;
           }
           if(x==0){
               count+=1;
           }
       }
       count
   }
}