use indicatif::ProgressBar;
use std::path::PathBuf;
use walkdir::WalkDir;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub fn calculate(project_dir: &PathBuf, pbar: ProgressBar) -> Result<(f32)> {
    let mut comments = 0;
    let mut code = 0;
    let ratio: f32;
    for entry in WalkDir::new(project_dir) {
        let entry = entry?;
        if entry.metadata()?.is_dir() {
            continue;
        }

        let filename = entry.path();       
        //check if this is an .rs file
        let re = Regex::new(r".+\.rs$").unwrap();
        if !re.is_match(&filename.display().to_string()) {
            continue;
        }
        let file = File::open(filename)?;
        for line in BufReader::new(file).lines() {
           let comment = Regex::new(r"//.*$").unwrap();
           let tline = line?;
           if comment.is_match(&tline.to_string()){
               comments+=1;
           }
           else if tline.is_empty() {
               //don't count empty lines
           }
           else {
               code+=1;
           }
        }
    }
    pbar.finish_and_clear();
    //comment
    ratio = comments as f32/code as f32;
    println!("code {}, comment {}, ratio {}", code, comments, ratio);
   
    Ok(ratio)
}
