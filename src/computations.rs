pub fn perform_computation_step(struct_array: &Vec<super::structs::Point>, json_config: &serde_json::Value, iteration: u64
) -> Result<Vec<super::structs::Point>, Box<dyn std::error::Error>> 
{
    let nx = json_config["GridParameters"]["nx"].as_u64().unwrap() as usize;
    let ny = json_config["GridParameters"]["ny"].as_u64().unwrap() as usize;
    let nz = json_config["GridParameters"]["nz"].as_u64().unwrap() as usize;

    let ntot = nx*ny*nz;

    let mut out_array = vec![crate::structs::Point {..Default::default()}; ntot];

    Ok(out_array)
}