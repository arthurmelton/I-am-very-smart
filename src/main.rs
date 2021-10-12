use clap::*;
use curl::easy::Easy;
use serde_json::Value;
use serde_json::value::Value::Null;

fn main() {
    let matches = App::new("I am very smart")
        .version("1.0")
        .about("translate something into smart talk")
        .arg(
            Arg::with_name("INPUT")
                .help("the text to convert")
                .required(false)
                .index(1),
        )
        .get_matches();
    let mut dst = Vec::new();
    let mut easy = Easy::new();
    easy.url("https://honk.moe/tools/thesaurus.json").unwrap();
    let _output = easy.custom_request("GET");
    let mut transfer = easy.transfer();
    transfer
        .write_function(|data| {
            dst.extend_from_slice(data);
            Ok(data.len())
        })
        .unwrap();
    transfer.perform().unwrap();
    drop(transfer);

    let v: Value = serde_json::from_str(&dst.iter().map(|&c| c as char).collect::<String>().as_str()).unwrap();
    let mut text = matches.value_of("INPUT").unwrap().to_ascii_lowercase().trim().to_string();
    
    for texts in text.clone().split(" ") {
        let v_texts = &v[texts];
        if v_texts != &Null {
            let v_texts = v_texts.as_str().unwrap();
            text = text.replace(&texts, v_texts);
        }
    }
    
    println!("{}", text);
}
