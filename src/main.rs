mod pig_latin;
use crate::pig_latin::pig_latin::pig_latin as piglatin;
fn main(){
    let word = String::from("apple");
    piglatin(&word);
}