// HashMap needs to be imported from the standard library before it can be used
use std::collections::HashMap;

// Implement the translate_fin function here
fn translate_fin(dic: &HashMap<String, String>, s: &str) -> String {
    match dic.get(s) {
        None => return format!("sorry, no translation for {s} available"),
        Some(translation) => return format!("{} in Finnish is {}", s, translation),
    };  
}

// Implement the eng_fin_dictionary function here
fn eng_fin_dictionary() -> HashMap<String,String> {
    let dic: HashMap<String,String> = HashMap::from([
        ("bear".to_string(), "karhu".to_string()),
        ("paw".to_string(), "tassu".to_string()),
        ("tail".to_string(), "häntä".to_string()),
        ("ear".to_string(), "korva".to_string()),
    ]);
    return dic
}

fn main() {
    let dictionary: HashMap<String, String> = eng_fin_dictionary();
    println!("{dictionary:#?}");

    println!("{}", translate_fin(&dictionary, "bear"));
    println!("{}", translate_fin(&dictionary, "paw"));
    println!("{}", translate_fin(&dictionary, "tail"));
    println!("{}", translate_fin(&dictionary, "🐻"));
     println!("{}", translate_fin(&dictionary, "dog")); 
}

