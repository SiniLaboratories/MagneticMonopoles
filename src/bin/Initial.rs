use std::env;
use magnetic_monopoles::write_read_file;
use magnetic_monopoles::index_conversion;
use magnetic_monopoles::structs;


fn main() 
{
    println!("Generation of initial condition started");
    let args: Vec<String> = env::args().collect();

    //println!("Example: Single charge at center of the domain");

    println!("Loading configuration file...");
    println!("{}",args[1]);
    let json_config = write_read_file::import_configuration_file(&args[1]).expect("Invalid JSON");

    println!("Generating initial condition...");

    let nx = json_config["GridParameters"]["nx"].as_u64().unwrap() as usize;
    let ny = json_config["GridParameters"]["ny"].as_u64().unwrap() as usize;
    let nz = json_config["GridParameters"]["nz"].as_u64().unwrap() as usize;

    let ntot = nx*ny*nz;
    let mut struct_array = vec![structs::Point {..Default::default()}; ntot];

    for i in 0..ntot
    {
        //For loop where the initial condituion is set
        let (ix, iy, iz) = index_conversion::index_to_3d(i, nx, ny, nz);
        //println!("i: {} -> ix: {}, iy: {}, iz: {}", i, ix, iy, iz);

        if ix == 50 && iy == 50 && iz == 0
        {
            struct_array[i].rhoep = 1.0;
        }
    }

    println!("Writing the initial condition file: {}",json_config["InitialCondition"]);

    // file
    let filename = json_config["InitialCondition"].as_str().unwrap();

    //write_read_file::write_condition_file(filename, &structArray).expect("Unable to write file");
    
    match write_read_file::write_condition_file(filename, &struct_array) {
        Ok(_) => println!("File saved successfully"),
        Err(e) => eprintln!("Errore during saving {}", e),
    }


    println!("Initial condition file written: {}",filename);
    println!("Initial condition generation completed.");
}