use crate::index_structs::index::remove_index_row;

pub fn reset_index_row(filename: String) {
    remove_index_row(filename).expect("{:?} isn't added yet.");
}