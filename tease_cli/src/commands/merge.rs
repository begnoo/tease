use crate::{utils::{lines::{get_content_from_sha1, Line}, blob_writer::{tease_file_exists, read_tree_from_commit, trail_commit_history, read_head_commit, create_tease_file, create_index_file}}, commands::read::read_object, index_structs::index::{read_index, Index, IndexRow, save_index}};

use super::{diff::{diff_file, DiffLine}, add::add_file, goback::delete_all};
use std::{collections::HashMap, fmt::{Display, Formatter, Result}, fs::{read_to_string, create_dir_all}, path::Path};

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

#[derive(Default)]
pub struct Chunk {
    o_lines: Vec<String>,
    a_lines: Vec<String>,
    b_lines: Vec<String>,
    resolve_type: ResolveType
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.resolve_type {
            ResolveType::Same => write!(f, "{}\n", self.o_lines.join("\n")),
            ResolveType::NewA => write!(f, "{}\n", self.a_lines.join("\n")),
            ResolveType::NewB => write!(f, "{}", self.b_lines.join("\n")),
            ResolveType::Conflict => write!(f, "\n>>>>>>>>(incoming)>>>>>>>>\n{} \
                                                \n===========================\n{} \
                                                \n<<<<<<<<<(current)<<<<<<<<", self.a_lines.join("\n"), self.b_lines.join("\n"))
        }
    }
}

enum ResolveType {
    Same,
    NewA,
    NewB,
    Conflict
}

impl Default for ResolveType {
    fn default() -> Self { ResolveType::Same }
}

#[derive(Default, Debug)]
struct IndexObject {
    sha1: String,
    path: String,
}

// TODO: sredi log da prikazuje commitove po redosledu a ne po roditeljima (mozda bitno samo za front)
pub fn merge(branch_name: String) {
    let branch_head = format!("refs/heads/{}", branch_name.to_string());

    if !tease_file_exists(branch_head.to_string()) {
        println!("Branch named {}, does not exist", branch_name.to_string());
        return ;
    }

    let branch_head_commit = read_to_string(Path::new(".tease").join(branch_head.to_string()))
        .expect(&format!("Couldn't read {}", branch_head));
    let mut branch_index = extract_index_from_commit(branch_head_commit.to_string());

    let current_head_commit = read_head_commit();
    let common_commit = find_common_commit(current_head_commit, branch_head_commit);
    let mut common_index = extract_index_from_commit(common_commit.to_string());
    
    let mut index = read_index();

    delete_all();
    create_index_file(Path::new(".tease").join("index").as_path());
    handle_index_diff(&mut index, &mut common_index, &mut branch_index)
}

fn handle_index_diff(old_index: &mut Index, common_index: &mut Vec<IndexObject>, branch_index: &mut Vec<IndexObject>) {
    let mut to_delete: Vec<IndexObject> = vec![];

    for common in common_index.iter() {

        let old_position = old_index.rows.iter().position(|current| current.file_name == common.path);
        let branch_position = branch_index.iter().position(|branch| branch.path == common.path);

        if old_position.is_some() && branch_position.is_some() {
            let mut old_row = old_index.rows.get_mut(old_position.unwrap()).unwrap();
            let branch_row = branch_index.get_mut(branch_position.unwrap()).unwrap();

            let chunks = merge_file(old_row.blob_hash.to_string(), branch_row.sha1.to_string(), common.sha1.to_string());
            if chunks.iter().find(|chunk| matches!(chunk.resolve_type, ResolveType::Conflict)).is_some() {
                old_row.staging = 1;
            } else {
                old_row.staging = 0;
            }
            
            let content: Vec<String> = chunks.iter().map(|chunk| chunk.to_string()).collect();
            create_missing_folders_and_file(old_row.file_name.to_string(), content.join(""));
            add_file(old_row.file_name.to_string())
                .expect(&format!("Couldn't merge file {}", old_row.file_name.to_string()));
            branch_index.remove(branch_position.unwrap());
            old_index.rows.remove(old_position.unwrap());
        } else if old_position.is_some() && branch_position.is_none() {
            to_delete.push(IndexObject { sha1: common.sha1.to_string(), path: common.path.to_string() });
            old_index.rows.remove(old_position.unwrap());
        }
    }

    handle_residual_branch_rows(branch_index);
    handle_residual_current_rows(old_index, &common_index);
    handle_rows_to_remove(&to_delete)
}

fn handle_residual_current_rows(old_index: & Index, common_index: & Vec<IndexObject>) {

    let mut added: Vec<String> = vec![];

    for old_row in old_index.rows.iter() {
        println!("{:?}", old_row);
        if common_index.iter().find(|row| row.path == old_row.blob_hash).is_some() {
            continue;
        } 
        
        let lines = get_content_from_sha1(old_row.blob_hash.to_string());
        let content: Vec<String> = lines.iter().map(|line| line.to_string()).collect();
        create_missing_folders_and_file(old_row.file_name.to_string(), content.join(""));
        add_file(old_row.file_name.to_string()).expect(&format!("Couldn't merge file {}", old_row.file_name.to_string()));
        added.push(old_row.file_name.to_string());
    }
    
    let mut new_index = read_index();
    
    for branch_row in added.iter() {
        let new_row_position = new_index.rows.iter().position(|new_row| new_row.file_name == branch_row.to_string());
        if new_row_position.is_some() {
            let mut new_row = new_index.rows.get_mut(new_row_position.unwrap()).unwrap();
            new_row.staging = 2;
        }
    }
}

fn handle_residual_branch_rows(branch_index: & Vec<IndexObject>) {
    for branch_row in branch_index.iter() {
        let lines = get_content_from_sha1(branch_row.sha1.to_string());
        let content: Vec<String> = lines.iter().map(|line| line.to_string()).collect();
        create_missing_folders_and_file(branch_row.path.to_string(), content.join(""));
        add_file(branch_row.path.to_string()).expect(&format!("Couldn't merge file {}", branch_row.path.to_string()));
    }
    
    let mut new_index = read_index();
    
    for branch_row in branch_index.iter() {
        let new_row_position = new_index.rows.iter().position(|new_row| new_row.file_name == branch_row.path);
        if new_row_position.is_some() {
            let mut new_row = new_index.rows.get_mut(new_row_position.unwrap()).unwrap();
            new_row.staging = 2;
        }
    }
}

fn handle_rows_to_remove(to_remove: & Vec<IndexObject>) {
    let mut new_index = read_index();

    for row_to_remove in to_remove.iter() {
        let new_row = new_index.rows.iter().find(|new_row| new_row.file_name == row_to_remove.path);
        if new_row.is_none() {
            new_index.rows.push( IndexRow { file_name: row_to_remove.path.to_string(), blob_hash: row_to_remove.sha1.to_string(), staging: 2, ..Default::default()} );
        }
    }

    save_index(new_index).expect("Couldn't update rows to remove while merging");
}

fn create_missing_folders_and_file(filepath: String, content: String) {
    let parts: Vec<&str> = filepath.split("/").collect();
    if parts.len() > 1 {
        let folder_path = parts[0..parts.len() - 1].join("/");
        let path = Path::new(&folder_path);
        if !path.exists() {
            create_dir_all(path).expect(&format!("Couldn't create folder {}", path.to_str().unwrap()));
        }
    }
    create_tease_file(Path::new(&filepath), content);
}

fn extract_index_from_commit(commit: String) -> Vec<IndexObject> {
    let root_tree = read_tree_from_commit(&commit);
    let mut temp_index: Vec<IndexObject> = vec![];
    collect_from_branch(root_tree, "".to_string(), &mut temp_index);

    temp_index
}

fn find_common_commit(current: String, incoming: String) -> String {
    let mut current_history: Vec<String> = vec![current.to_string()];
    trail_commit_history(&current, &mut current_history);

    let mut incoming_history: Vec<String> = vec![incoming.to_string()];
    trail_commit_history(&incoming, &mut incoming_history);

    for current_sha1 in current_history.iter() {
       for incoming_sha1 in incoming_history.iter() {
            if current_sha1 == incoming_sha1 {
                return current_sha1.to_string();
            }
       }   
    }

    "".to_string()
}

fn collect_from_branch(root_tree: String, prev_path: String, temp_index: & mut Vec<IndexObject>) {    
    let tree_content = read_object(&root_tree);
    let lines: Vec<&str> = tree_content.split("\n").collect();

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        
        if parts[0] == "blob" {
            let new_file = if prev_path.is_empty() { parts[1].to_string() } else { vec![prev_path.to_string(), parts[1].to_string()].join("/") };
            temp_index.push(IndexObject { sha1: parts[2].to_string(), path: new_file });
        }

        if parts[0] == "tree" {
            let new_folder = if prev_path.is_empty() { parts[1].to_string() } else { vec![prev_path.to_string(), parts[1].to_string()].join("/") };
            collect_from_branch(parts[2].to_string(), new_folder.to_string(), temp_index);
        }
    }
}

pub fn merge_file(a_sha1: String, b_sha1: String, o_sha1: String) -> Vec<Chunk> {
    let a_diff = diff_file(o_sha1.to_string(), a_sha1.to_string());
    let b_diff = diff_file(o_sha1.to_string(), b_sha1.to_string());

    let a_matches = find_matches(a_diff);
    let b_matches = find_matches(b_diff);

    let a_lines = get_content_from_sha1(a_sha1);
    let b_lines = get_content_from_sha1(b_sha1);
    let o_lines = get_content_from_sha1(o_sha1);

    let mut match_index = MatchIndex {
        a: 0, a_len: a_lines.len(), a_lines,
        b: 0, b_len: b_lines.len(), b_lines,
        o: 0, o_len: o_lines.len(), o_lines
    };
    generate_chunks(&a_matches, &b_matches, & mut match_index)
}

fn generate_chunks(a_matches: &HashMap<usize, usize>, b_matches: &HashMap<usize, usize>, match_index: & mut MatchIndex) -> Vec<Chunk> {
    let mut chunks: Vec<Chunk> = vec![];
    loop {
        let i = find_next_mismatch(&a_matches, &b_matches, &match_index);

        if i == 1 {
            // postavlja o, a i b
            let new_values = find_next_match(a_matches, b_matches, match_index);
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
    }

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