use std::{collections::HashMap, sync::Mutex};

use pyo3::{prelude::*, pybacked::PyBackedStr};
use rayon::prelude::*;

const CHUNK_SIZE: usize = 1000;

fn preprocess_line(line: &str) -> String {
    line.chars()
        .filter(|c| c.is_ascii_alphabetic() || c.is_ascii_whitespace())
        .map(|c| c.to_ascii_lowercase())
        .collect::<String>()
}

fn update_word_count(line: &str, words: &mut HashMap<String, u32>) {
    for word in preprocess_line(line).split(' ') {
        match words.get_mut(word) {
            Some(count) => *count += 1,
            None => {
                words.insert(word.to_string(), 1);
            }
        }
    }
}

/// Counts words in given lines
#[pyfunction]
fn get_word_counter_dict_rs(lines: Vec<PyBackedStr>) -> HashMap<String, u32> {
    let mut words: HashMap<String, u32> = HashMap::new();

    for line in lines {
        update_word_count(&line, &mut words);
    }

    words
}

/// Counts words in given lines in parallel
#[pyfunction]
fn get_word_counter_dict_parallel_rs(lines: Vec<PyBackedStr>) -> HashMap<String, u32> {
    let words: Mutex<HashMap<String, u32>> = Mutex::new(HashMap::new());

    lines.par_chunks(CHUNK_SIZE).for_each(|lines| {
        let mut local_words: HashMap<String, u32> = HashMap::new();
        for line in lines {
            update_word_count(line, &mut local_words);
        }
        let mut words = words.lock().unwrap();
        for (word, count) in local_words {
            *words.entry(word).or_default() += count;
        }
    });

    words.into_inner().unwrap()
}

/// A Python module implemented in Rust.
#[pymodule]
fn python_heart_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_word_counter_dict_rs, m)?)?;
    m.add_function(wrap_pyfunction!(get_word_counter_dict_parallel_rs, m)?)?;
    Ok(())
}
