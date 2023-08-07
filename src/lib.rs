use anyhow::Ok;
use rust_bert::pipelines::translation::{Language, TranslationModelBuilder};
use std::fs::File;
use std::io::Read;
use std::io::Write;

//function that reads the file and return string

pub fn read_file(path: &str) -> anyhow::Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content);
    Ok(content)
}

//function that reads the file and return lines as array
pub fn read_file_to_array(path: &str) -> anyhow::Result<Vec<String>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content);
    let array = content.lines().map(|s| s.to_string()).collect();
    Ok(array)
}

//function that takes the file and translates it
pub fn translate(path: &str, saving_path: &str) -> anyhow::Result<()> {
    let model = TranslationModelBuilder::new()
        .with_source_languages(vec![Language::Spanish])
        .with_target_languages(vec![Language::English])
        .create_model()?;
    let text = read_file_to_array(path)?;
    //pass text to model
    let output = model.translate(&text, None, Language::English)?;
    for sentence in &output {
        println!("{}", *sentence);
    }
    let _ = create_file(saving_path, output);
    Ok(())
}

fn create_file(path: &str, content: Vec<String>) -> anyhow::Result<()> {
    let str = content.join(" ");
    let mut file = File::create(path).unwrap();
    write!(file, "{}", str).unwrap();
    // Flush and close file
    file.flush().unwrap();
    Ok(())
}
