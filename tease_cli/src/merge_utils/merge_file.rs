use std::{collections::HashMap, fmt::{Display, Formatter, Result}};

use tease_common::diff::{diff_file, DiffLine};

use crate::{utils::lines::{Line, get_content_from_sha1}};

#[derive(Debug)]
struct MatchIndex {
    a: usize,
    b: usize,
    o: usize,
    a_len: usize,
    b_len: usize,
    o_len: usize,
    a_lines: Vec<Line>,
    b_lines: Vec<Line>,
    o_lines: Vec<Line>,
}

#[derive(Default, Debug)]
pub struct Chunk {
    pub o_lines: Vec<String>,
    pub a_lines: Vec<String>,
    pub b_lines: Vec<String>,
    pub resolve_type: ResolveType
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter) -> Result {

        if self.o_lines.is_empty() && self.a_lines.is_empty() && self.b_lines.is_empty() {
            return write!(f, "");
        }

        match self.resolve_type {
            ResolveType::Same => write!(f, "{}", self.o_lines.join("\n")),
            ResolveType::NewA => write!(f, "{}", self.a_lines.join("\n")),
            ResolveType::NewB => write!(f, "{}", self.b_lines.join("\n")),
            ResolveType::Conflict => write!(f,
                               "\n>>>>>>>>(incoming)>>>>>>>>\n\
                                {} \
                                \n==========================\n\
                                {} \
                                \n<<<<<<<<<(current)<<<<<<<<\n", self.a_lines.join("\n"), self.b_lines.join("\n"))
}
    }
}

#[derive(Debug)]
pub enum ResolveType {
    Same,
    NewA,
    NewB,
    Conflict
}

impl Default for ResolveType {
    fn default() -> Self { ResolveType::Same }
}


pub fn merge_file(a_sha1: String, b_sha1: String, o_sha1: String) -> Vec<Chunk> {
    let a_diff = diff_file(".tease".to_string(), o_sha1.to_string(), a_sha1.to_string());
    let b_diff = diff_file(".tease".to_string(), o_sha1.to_string(), b_sha1.to_string());

    let a_matches = find_matches(a_diff);
    let b_matches = find_matches(b_diff);

    let a_lines = get_content_from_sha1(a_sha1);
    let b_lines = get_content_from_sha1(b_sha1);
    let o_lines = get_content_from_sha1(o_sha1);

    // println!("len [] a: {} b: {} o: {}", a_lines.len(), b_lines.len(), o_lines.len());

    let mut match_index = MatchIndex {
        a: 0, a_len: a_lines.len(), a_lines,
        b: 0, b_len: b_lines.len(), b_lines,
        o: 0, o_len: o_lines.len(), o_lines
    };
    let mut chunks = generate_chunks(&a_matches, &b_matches, & mut match_index);
    chunks.retain(|chunk| !(chunk.a_lines.is_empty() && chunk.b_lines.is_empty() && chunk.o_lines.is_empty()));
    chunks
}

fn generate_chunks(a_matches: &HashMap<usize, usize>, b_matches: &HashMap<usize, usize>, match_index: & mut MatchIndex) -> Vec<Chunk> {
    let mut chunks: Vec<Chunk> = vec![];
    loop {
        let i = find_next_mismatch(&a_matches, &b_matches, &match_index);
        // println!("i: {}", i);

        if i == 1 {
            // postavlja o, a i b
            let new_values = find_next_match(a_matches, b_matches, match_index);
            // println!("{:?}", new_values);   
            if new_values[1] != match_index.a && new_values[2] != match_index.b {
                chunks.push(emit(match_index, new_values));
            } 
            else { 
                chunks.push(emit_final(match_index));
                break;
            }
        } else if i != 0 {
            chunks.push(emit(match_index, vec![i]));
        } else {
            chunks.push(emit_final(match_index));
            break; 
        }
        // println!("o: {} a: {} b: {}", match_index.o, match_index.a, match_index.b);
    }
    // println!("{}", chunks.len());
    chunks
}

fn find_next_mismatch(a_matches: &HashMap<usize, usize>, b_matches: &HashMap<usize, usize>, match_index: &MatchIndex) -> usize {
    
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

fn find_next_match(a_matches: &HashMap<usize, usize>, b_matches: &HashMap<usize, usize>, match_index: & mut MatchIndex) -> Vec<usize> {
    let mut o = match_index.o + 1;
    
    while o < match_index.o_len 
        && !(a_matches.contains_key(&o) && b_matches.contains_key(&o)) 
    {
        o = o + 1;
    }

    vec![o, a_matches.get(&o).unwrap_or(&match_index.a).to_owned(), b_matches.get(&o).unwrap_or(&match_index.b).to_owned()]
}

fn inbounds(i: usize, match_index: &MatchIndex) -> bool {
    if i < match_index.a_len || i < match_index.b_len || i < match_index.o_len {
        return true;
    }

    false
}

fn match_line(matches: &HashMap<usize, usize>, original: usize, offset: usize, i: usize) -> bool {
    matches.contains_key(&(original + i)) && matches.get(&(original + i)).unwrap().to_owned() == offset + i
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

fn emit(match_index: & mut MatchIndex, offsets: Vec<usize>) -> Chunk {

    let o_offset = if offsets.len() != 1 { offsets.get(0).unwrap().to_owned() - match_index.o } else { offsets[0] };
    let a_offset = if offsets.len() != 1 { offsets.get(1).unwrap().to_owned() - match_index.a } else { offsets[0] };
    let b_offset = if offsets.len() != 1 { offsets.get(2).unwrap().to_owned() - match_index.b } else { offsets[0] };

    let o_chunk = map_chunk(&match_index.o_lines, match_index.o, o_offset);
    let a_chunk = map_chunk(&match_index.a_lines, match_index.a, a_offset);
    let b_chunk = map_chunk(&match_index.b_lines, match_index.b, b_offset);

    let chunk = handle_chunk(o_chunk, a_chunk, b_chunk);

    match_index.o = match_index.o + o_offset - 1;
    match_index.a = match_index.a + a_offset - 1;
    match_index.b = match_index.b + b_offset - 1;

    chunk
}

fn emit_final(match_index: & mut MatchIndex) -> Chunk {

    let o_chunk = map_chunk(&match_index.o_lines, match_index.o, match_index.o_len - match_index.o + 1);
    let a_chunk = map_chunk(&match_index.a_lines, match_index.a, match_index.a_len - match_index.a + 1);
    let b_chunk = map_chunk(&match_index.b_lines, match_index.b, match_index.b_len - match_index.b + 1);

    handle_chunk(o_chunk, a_chunk, b_chunk)
}

fn handle_chunk(o_chunk: Vec<String>, a_chunk: Vec<String>, b_chunk: Vec<String>) -> Chunk {
    if o_chunk == a_chunk && o_chunk == b_chunk {
        return Chunk {o_lines: o_chunk, resolve_type: ResolveType::Same, ..Default::default()}
    } else if o_chunk == a_chunk {
        return Chunk {b_lines: b_chunk, resolve_type: ResolveType::NewB, ..Default::default()}
    } else if o_chunk == b_chunk {
        return Chunk {a_lines: a_chunk, resolve_type: ResolveType::NewA, ..Default::default()}
    } else {
        return Chunk {a_lines: a_chunk, b_lines: b_chunk, resolve_type: ResolveType::Conflict, ..Default::default()}
    }
}

fn map_chunk(lines: &Vec<Line>, start: usize, offset: usize) -> Vec<String> {
    (start..start + offset - 1)
        .map(|index| (&lines[index].content).to_string())
        .collect()
}