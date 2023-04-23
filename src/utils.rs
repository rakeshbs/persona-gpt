use std::time::{SystemTime, UNIX_EPOCH};
use tiktoken_rs::cl100k_base;

pub fn read_and_sort_dir(path: &str) -> std::io::Result<Vec<std::fs::DirEntry>> {
    let dir_path = std::path::Path::new(path);
    let mut entries: Vec<_> = std::fs::read_dir(dir_path)?
        .map(|res| res.map(|e| e))
        .collect::<Result<Vec<_>, std::io::Error>>()?;

    entries.sort_by_key(|entry| std::cmp::Reverse(entry.file_name()));

    Ok(entries)
}

pub fn get_epoch_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub fn token_length(text: &str) -> usize {
    let bpe = cl100k_base().unwrap();
    let tokens = bpe.encode_with_special_tokens(text);
    return tokens.len();
}
