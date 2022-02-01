mod unstructured_json;

fn main() {
    if let Err(e) = unstructured_json::serialize_and_deserialize_unstructured_json() {
        println!("{}", e);
    }
}
