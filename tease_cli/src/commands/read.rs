use tease_common::read::blob_reader::read_object as common_read;

pub fn read_object(object_name: &String) -> String {
    common_read(&".tease".to_string(), &object_name.to_string())
}