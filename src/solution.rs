use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let words: Vec<&str> = words.iter().map(|s| s.as_str()).collect();
        let res: Vec<usize> = find_substrings(&s, &words[..]);
        res.iter().map(|x| *x as i32).collect()
    }
}

// Returns the indices of all substring of s that are a concatenation of all words.
// The words are not necessarily distinct.
// The order of the words in the substring does not matter.
//
// For example:
// s = "barfoothefoobarman"
// words = ["foo", "bar"]
// returns [0, 9]
pub fn find_substrings(s: &str, words: &[&str]) -> Vec<usize> {
    let sum_length: usize = words.iter().map(|s| s.len()).sum();
    if sum_length > s.len() {
        return Vec::new();
    }
    let mut result: Vec<usize> = Vec::new();
    for idx_begin_substring in 0..(s.len() - sum_length + 1) {
        let substring: &str = &s[idx_begin_substring..(idx_begin_substring + sum_length)];
        let occurrences: Vec<Vec<usize>> = find_all_occurrences(substring, words);
        let mut indices_of_unused_words: HashSet<usize> = (0..words.len()).collect();
        if is_concatenation(&mut indices_of_unused_words, substring, words, &occurrences) {
            result.push(idx_begin_substring);
        }
    }
    result
}

fn is_concatenation(indices_of_unused_words: &mut HashSet<usize>, substring: &str, words: &[&str], occurrences: &[Vec<usize>]) -> bool {
    assert!(substring.len() == occurrences.len(), "substring and occurrences must have the same length. substring = {:?}, occurrences = {:?}", substring, occurrences);
    // If we have used all words, we have found a concatenation.
    if indices_of_unused_words.is_empty() {
        assert!(substring.is_empty(), "substring must be empty, otherwise sum_length != substring which violates the premise of the arguments. substring = {:?}", substring);
        return true;
    }
    if substring.is_empty() {
        return false;
    }
    let word_indices: &[usize] = &occurrences[0];
    for word_idx in word_indices {
        if !indices_of_unused_words.contains(word_idx) {
            continue;
        }
        indices_of_unused_words.remove(word_idx);
        let word: &str = words[*word_idx];
        let is_concat: bool = is_concatenation(indices_of_unused_words, &substring[word.len()..], words, &occurrences[word.len()..]);
        // Restore the set to its original state
        indices_of_unused_words.insert(*word_idx);
        if is_concat {
            return true;
        }
    }
    false
}

// Returns a vec where each element represents a char index in s.
// Each char index contains a vec of all indices of words that start at that char index.
// For example:
// s = "foobar"
// words = ["foo", "bar", "o", "ba"]
// result =
// "  f    o    o      b     a   r "  
//  [{0}, {2}, {2}, {1, 3}, {}, {}]
fn find_all_occurrences(s: &str, words: &[&str]) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = vec![vec![]; s.len()];
    for (index_word, word) in words.iter().enumerate() {
        let occurrences_of_word: Vec<usize> = find_occurrences(s, word);
        for occ in occurrences_of_word {
            result[occ].push(index_word);
        }
    }
    result
}

// Searches through s and returns a Vec of all indices in s where "word" starts.
fn find_occurrences(s: &str, word: &str) -> Vec<usize> {
    let mut result = Vec::new();
    let mut start = 0;
    while let Some(index) = s[start..].find(word) {
        result.push(start + index);
        start += index + 1;
    }
    result
}
