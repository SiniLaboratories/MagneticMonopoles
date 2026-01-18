pub fn perform_computation_step(struct_array: &Vec<super::structs::Point>, json_config: &serde_json::Value, iteration: u64
) -> Result<Vec<super::structs::Point>, Box<dyn std::error::Error>> 
{
    let nx = json_config["GridParameters"]["nx"].as_u64().unwrap() as usize;
    let ny = json_config["GridParameters"]["ny"].as_u64().unwrap() as usize;
    let nz = json_config["GridParameters"]["nz"].as_u64().unwrap() as usize;

    let ntot = nx*ny*nz;

    let mut out_array = vec![crate::structs::Point {..Default::default()}; ntot];
    /*
    let f1 = f(&struct_array,&json_config);
    for i in 0..ntot {
        out_array[i] = struct_array[i] + f1.as_ref().unwrap()[i].clone()*json_config["ComputationParameters"]["dt"].as_f64().unwrap();
    }
    */

    //out_array = struct_array + f(&struct_array,&json_config)*json_config["ComputationParameters"]["dt"];

    //Now we start doing the fancy tricks

    //Eh eh eh

    //

    Ok(out_array)
}



fn f(struct_array: &Vec<super::structs::Point>, json_config: &serde_json::Value
) -> Result<Vec<super::structs::DtPoint>, Box<dyn std::error::Error>> 
{
    let nx = json_config["GridParameters"]["nx"].as_u64().unwrap() as usize;
    let ny = json_config["GridParameters"]["ny"].as_u64().unwrap() as usize;
    let nz = json_config["GridParameters"]["nz"].as_u64().unwrap() as usize;

    let ntot = nx*ny*nz;
    
    //rotor of E
    //rotor of B

    //divegence of ----
    //divegence of ----
    //divegence of ----
    //divegence of ----


    //lorentz force for ---
    //lorentz force for ---
    //lorentz force for ---
    //lorentz force for ---

    let mut out_array = vec![crate::structs::DtPoint {..Default::default()}; ntot];
    Ok(out_array)
}