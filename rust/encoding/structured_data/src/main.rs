mod toml_file;
mod unstructured_json;

fn main() {
    if let Err(e) = unstructured_json::serialize_and_deserialize_unstructured_json() {
        println!("{}", e);
    }
    if let Err(e) = toml_file::deserialize_a_toml_configuration_file() {
        println!("{}", e);
    }
    if let Err(e) = toml_file::toml_using_serde() {
        println!("{}", e);
    }
}
