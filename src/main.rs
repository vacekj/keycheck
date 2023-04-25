use std::fs;
use std::path::Path;
use ignore::{self, WalkBuilder};
use regex::Regex;

fn main() {
    let mut builder = WalkBuilder::new("./");
    builder
        .standard_filters(false)
        .hidden(false)
        .parents(false)
        .git_ignore(true);

    let private_keys = find_private_keys(builder);

    if !private_keys.is_empty() {
        println!("Warning: private keys found in the following files:");
        for (path, line_number) in private_keys {
            println!("{}:{}", path.display(), line_number);
        }
    }
}

fn find_private_keys(builder: WalkBuilder) -> Vec<(std::path::PathBuf, usize)> {
    let mut private_keys = Vec::new();

    for entry in builder.build().flatten() {
        let path = entry.path();
        if path.is_file() {
            if let Some(line_number) = find_private_key(path) {
                private_keys.push((path.to_path_buf(), line_number));
            }
        }
    }

    private_keys
}

fn find_private_key(path: &Path) -> Option<usize> {
    let file_contents = fs::read_to_string(path).ok()?;
    let re = Regex::new(r"0x([A-Fa-f0-9]{64})").unwrap();
    re.find(&file_contents).map(|key| count_newlines(&file_contents[..key.start()]) + 1)
}

fn count_newlines(s: &str) -> usize {
    s.chars().filter(|&c| c == '\n').count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use ethers::prelude::H256;
    use tempfile::tempdir;
    use rand::Rng;

    #[test]
    fn test_find_private_keys() {
        let temp_dir = tempdir().unwrap();

        let mut rng = rand::thread_rng();

        let private_key = H256::from_slice(&rng.gen::<[u8; 32]>());

        let mut files_with_keys = Vec::new();

        for i in 0..10 {
            let mut file_path = PathBuf::from(temp_dir.path());
            file_path.push(format!("file{}.txt", i));

            let mut file = File::create(&file_path).unwrap();

            let mut file_contents = String::new();

            for i in 0..100 {
                let contents = random_string(&mut rng, 10);
                file_contents.push_str(&format!("{}\n", contents));

                if i == 50 {
                    let key_str = format!("0x{:064x}", private_key);
                    let key_start = file_contents.len();
                    let random_padding = random_string(&mut rng, 10);

                    file_contents = format!("{}{}{}\n", &file_contents, &key_str, random_padding);
                    files_with_keys.push((file_path.clone(), count_newlines(&file_contents[..key_start]) + 1));
                }
            }

            file.write_all(file_contents.as_bytes()).unwrap();
        }

        let mut builder = WalkBuilder::new(temp_dir.path());
        builder
            .standard_filters(false)
            .hidden(false)
            .parents(false)
            .git_ignore(true);

        let private_keys = find_private_keys(builder);

        assert_eq!(private_keys.len(), files_with_keys.len(), "Private keys count mismatch");

        for (path, line) in files_with_keys {
            let expected_output = (path.clone(), line);
            assert!(
                private_keys.contains(&expected_output),
                "Missing private key in file: {}:{}",
                path.display(),
                line
            );
        }
    }

    fn random_string(rng: &mut impl Rng, len: usize) -> String {
        (0..len)
            .map(|_| rng.gen_range(b'a'..b'z' + 1) as char)
            .collect()
    }
}
