//merge altenative solution

    pub fn merge_alternatively (word1:String, word2:String)->String{

        let char1 :Vec<char>= word1.chars().collect();
        let char2: Vec<char> = word2.chars().collect();

        let mut merged = String::new();

        let mut i =0;

        while i <char1.len() || i <char2.len(){
            if i < char1.len(){
                merged.push(char1[i]);
            }
            if i < char2.len(){
                merged.push(char2[i]);
            }
            i += 1;
        }
        merged
    }
