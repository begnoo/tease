use crate::utils::lines::get_content_from_blob;

use super::diff::{diff_file, DiffLine};
use std::collections::HashMap;

struct MatchIndex {
    a: usize,
    b: usize,
    o: usize,
    a_len: usize,
    b_len: usize,
    o_len: usize
}

pub fn merge_file(a_sha1_blob: String, b_sha1_blob: String, o_sha1_blob: String) {
    let a_diff = diff_file(o_sha1_blob.to_string(), a_sha1_blob.to_string());
    let b_diff = diff_file(o_sha1_blob.to_string(), b_sha1_blob.to_string());

    let a_matches = find_matches(a_diff);
    let b_matches = find_matches(b_diff);

    let a_lines = get_content_from_blob(a_sha1_blob);
    let b_lines = get_content_from_blob(b_sha1_blob);
    let o_lines = get_content_from_blob(o_sha1_blob);

    let mut match_index = MatchIndex {
        a: 0, a_len: a_lines.len(),
        b: 0, b_len: b_lines.len(),
        o: 0, o_len: o_lines.len(),
    };
    handle_mismaches(&a_matches, &b_matches, & mut match_index);
}

fn handle_mismaches(a_matches: &HashMap<usize, usize>, b_matches: &HashMap<usize, usize>, match_index: & mut MatchIndex) {
    let mut prev_a: usize;
    let mut prev_b: usize;

    loop {
        let i = find_next_mismach(&a_matches, &b_matches, &match_index);
        
        prev_a = match_index.a;
        prev_b = match_index.b; 

        if i == 1 {
            // postavlja o, a i b
            find_next_match(a_matches, b_matches, match_index);
            if match_index.a != prev_a && match_index.b != prev_b {
                //emit
            } 
            else { 
                //emit final
                return ;
            }
        } else if i != 0 {
            //emit
        } else {
            //emit final
        }
    }
}

fn find_next_match(a_matches: &HashMap<usize, usize>, b_matches: &HashMap<usize, usize>, match_index: & mut MatchIndex) {
    match_index.o = match_index.o + 1;
    
    while match_index.o < match_index.o_len 
            && a_matches.contains_key(&match_index.a) 
            && b_matches.contains_key(&match_index.b) 
    {
        match_index.o = match_index.o + 1;
    }

    match_index.a = a_matches.get(&match_index.o).unwrap().to_owned();
    match_index.b = b_matches.get(&match_index.o).unwrap().to_owned();
}

fn find_next_mismach(a_matches: &HashMap<usize, usize>, b_matches: &HashMap<usize, usize>, match_index: &MatchIndex) -> usize {
    let mut i = 1;
    while inbounds(i, match_index) 
            && match_line(a_matches, match_index.o, match_index.a, i)
            && match_line(b_matches, match_index.o, match_index.b, i) 
    {
        i = i + 1;
    }   

    if inbounds(i, match_index) {
        return i;
    }

    0
}

fn inbounds(i: usize, match_index: &MatchIndex) -> bool {
    if i < match_index.a_len || i < match_index.b_len || i < match_index.o_len {
        return true;
    }

    false
}

fn match_line(matches: &HashMap<usize, usize>, original: usize, offset: usize, i: usize) -> bool {
    matches.get(&(original + i)).or(Option::Some(&0)).unwrap().to_owned() == offset + i
}

fn find_matches(diff_lines: Vec<DiffLine>) -> HashMap<usize, usize> {
    let mut matches_map: HashMap<usize, usize> = HashMap::new();
    for diff_line in diff_lines.iter() {
        if diff_line.state == "equ" {
            matches_map.insert(diff_line.line.number, diff_line.new_number);
        }
    }

    matches_map
}