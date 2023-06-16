// Given two strings s and t, return true if t is an anagram of s, and false otherwise.
//
// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase,
// typically using all the original letters exactly once.
//
//
//
// Example 1:
//
// Input: s = "anagram", t = "nagaram"
// Output: true
// Example 2:
//
// Input: s = "rat", t = "car"
// Output: false
//
//
// Constraints:
//
// 1 <= s.length, t.length <= 5 * 104
// s and t consist of lowercase English letters.

use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    let mut m = HashMap::new();
    s.chars().for_each(|c| *m.entry(c).or_insert(0) += 1);
    t.chars().for_each(|c| *m.entry(c).or_insert(0) -= 1);
    m.into_values().all(|v|v == 0)
}