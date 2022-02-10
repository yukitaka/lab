use nalgebra::DMatrix;
use serde_json;

pub fn deserialize_a_matrix() -> Result<(), std::io::Error> {
    let row_slice: Vec<i32> = (1..5001).collect();
    let matrix = DMatrix::from_row_slice(50, 100, &row_slice);

    let serialized_matrix = serde_json::to_string(&matrix)?;
    let deserialized_matrix: DMatrix<i32> = serde_json::from_str(&serialized_matrix)?;

    assert_eq!(deserialized_matrix, matrix);
    Ok(())
}
