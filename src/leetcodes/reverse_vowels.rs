impl Solutions{
    pub fn reverse_vowels(s: String) ->String{
        let vowels = "aeiouAEIOU";

        let mut chars : Vec<char> =s.chars().collect();

        let mut left =0;
        let mut righ = s.len() -1;

        while left<right{
            while  left <right && !vowels.contains(chars[left]){
                left += 1;
            }
            while left <right && !vowels.contains(chars[right]){
                right -= 1;
            }
            chars.swap(left,right);
            left +=1;
            right -=1;
        }
        chars.into_iter().collect()

    }
}