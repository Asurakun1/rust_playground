pub mod pig_latin{
    pub fn pig_latin(word: &String){
        let mut copy_of_word = String::from(word);
        let first_char = get_first_element(&word);
    
        if check_consonants(&first_char){
            copy_of_word = change_on_consonant(&copy_of_word);
        }else{
            copy_of_word = change_on_vowel(&copy_of_word);    
        }
        println!("{}", copy_of_word);
    }
    
    fn change_on_vowel (word: &String) -> String {
        let word = word[0..].to_string() + "-hay";
        word 
    }
    
    fn change_on_consonant (word: &String) -> String {
        let first_char = get_first_element(word).to_string();
    
        let word = word[1..].to_string() + "-" + &first_char + "ay";
        word
    }
    
    fn get_first_element (word: &String) -> char{
    
        let ch = word.chars().next().unwrap();
        ch
    }
    
    fn check_consonants(ch: &char) -> bool {
        let consonants = ['b', 'c','d','f','g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'z', 'y'];
        let mut result = false;
    
        for con in consonants{
            match con {
                con if con == *ch => {
                    result = true;
                    break;
                },
                _ => {
    
                }
            }
        }
        result
    }
}
