use crate::index_structs::index::reset_index_row;


pub fn reset(file_path: String) -> () {
    let reset_res = reset_index_row(file_path.to_string());
    if reset_res.is_err() {
        println!("File {:?} is not present in the index.", file_path);
        return ;
    }

    println!("Index row successfuly reset.");
}