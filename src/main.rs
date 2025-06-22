use std::env;
use std::fs;
use std::fs::{File,
    DirEntry
};
use std::path::{Path, PathBuf};
use std::io::Read;

fn main() {
    // PRAISE GOD, PRAISE GOD
    // Hail to Vega

    let current_dir = env::current_dir().unwrap();
    let formatted_dir = current_dir.into_os_string().into_string().unwrap();

    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        let vec_extensions = sort_vec_by_extensions(formatted_dir);
        let mut newlines: u32 = 0;

        for file in vec_extensions {
            let file_result = File::open(file);
            let mut file = match file_result {
                Ok(file) => file,
                Err(error) => {
                    continue;
                }
            };
            let mut data: Vec<u8> = vec![];

            match file.read_to_end(&mut data) {
                Ok(data) => data,
                Err(error) => {
                    continue;
                }
            };

            newlines += check_newlines(data);
        }

        println!("Total lines count in the directory: {newlines}");
    } else {
        let second_arg: &String = &args[1];
        // println!("Second argument (filename): {second_arg}");



        let file_result = File::open(second_arg);
        let mut file = match file_result {
            Ok(file) => file,
            Err(error) => {
                use std::panic;

                panic::set_hook(Box::new(|error| {
                    println!("{error}");
                }));
                panic!("{error}");
            }
        };

        let mut data = vec![];
        match file.read_to_end(&mut data) {
            Ok(data) => data,
            Err(error) => {
                use std::panic;

                panic::set_hook(Box::new(|error| {
                    println!("{error}");
                }));
                panic!("{error}");
            }
        };

        let newlines: u32 = check_newlines(data);
        println!("Total lines count in {second_arg}: {newlines}");
    }
}

fn check_newlines(vec: Vec<u8>) -> u32 {
    let mut counter = 1;
    for num in vec {
        if num == 10 {
            counter += 1;
        } else {
            continue;
        }
    }
    return counter;
}

fn sort_vec_by_extensions<P: AsRef<Path>>(dir: P) -> Vec<String> {
    let extensions_checked: &[&str; 6] = &[".rs", ".txt", ".h", ".c", ".bat", ".sh"];

    let mut collected = Vec::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                // Рекурсивный вызов
                collected.extend(sort_vec_by_extensions(path));
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if extensions_checked.iter().any(|&x| x.trim_start_matches('.') == ext) {
                    if let Some(path_str) = path.to_str() {
                        collected.push(path_str.to_string());
                    }
                }
            }
        }
    }

    return collected;
}
