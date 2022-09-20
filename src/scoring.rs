use crate::{WMask, Word};
use rayon::prelude::*;

#[inline]
pub fn info_bits(old: usize, new: usize) -> f64 {
    (old as f64).log2() - (new as f64).log2()
}

#[inline]
pub fn info_bits2(old: usize, new: usize, sols: usize) -> f64 {
    (old as f64).log2() - (new as f64).log2() - (sols as f64).log2()
}

pub fn apply_mask(words: &mut Vec<Word>, mask: WMask) -> f64 {
    let size = words.len();
    words.retain(|&s| mask.matches(s));
    info_bits(size, words.len())
}

pub fn calc_guess_info_iter(words: impl IntoIterator<Item = Word>, mask: WMask) -> f64 {
    let [old, new] = words.into_iter().fold([0, 0], |[old, new], word| {
        [old + 1, new + if mask.matches(word) { 1 } else { 0 }]
    });
    info_bits(old, new)
}

pub fn calc_guess_info(words: &[Word], mask: WMask) -> f64 {
    info_bits(words.len(), words.iter().filter(|&&word| mask.matches(word)).count())
}

pub fn count_mask_hits(words: impl IntoIterator<Item = Word>, mask: WMask) -> usize {
    words.into_iter().filter(|&w| mask.matches(w)).count()
}

pub fn find_best_word(wordlist: &[Word], sollist: &[Word]) -> (Word, f64) {
    wordlist
        .par_iter()
        .map(|&guess| {
            let sum: f64 = sollist
                .par_iter()
                .map(|&answer| {
                    let mask = WMask::compare(guess, answer);
                    let hits = count_mask_hits(wordlist.iter().copied(), mask);
                    info_bits(wordlist.len(), hits)
                })
                .sum();
            (guess, sum)
        })
        .max_by(|(_, s1), (_, s2)| s1.total_cmp(s2))
        .unwrap()
}

pub fn find_best_word_set(wordlist: &[Word], sollist: &[Word]) -> Vec<(Word, f64)> {
    let mut vec = Vec::with_capacity(wordlist.len());
    wordlist
        .par_iter()
        .map(|&guess| {
            let sum: f64 = sollist
                .par_iter()
                .map(|&answer| {
                    let mask = WMask::compare(guess, answer);
                    let hits = count_mask_hits(wordlist.iter().copied(), mask);
                    info_bits(wordlist.len(), hits)
                })
                .sum();
            (guess, sum)
        })
        .collect_into_vec(&mut vec);

    vec.par_sort_by(|(_, s1), (_, s2)| s2.total_cmp(s1));
    vec.truncate(100);
    vec
}
