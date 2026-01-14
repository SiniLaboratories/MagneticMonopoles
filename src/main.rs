use std::env;
mod write_read_file;
mod structs;
mod computations;


fn main() 
{
    println!("Generation of initial condition started");
    let args: Vec<String> = env::args().collect();

    println!("Example: plain wave propagation");

    println!("Loading configuration file...");
    let json_config = write_read_file::import_configuration_file(&args[1]).expect("Invalid JSON");

    println!("Loading initial condition file: {}",json_config["InitialCondition"]);

    //Load configuration file
    let mut struct_array = write_read_file::read_condition_file(json_config["InitialCondition"].as_str().unwrap()).expect("Could not load initial condition file");

    println!("Initial condition imported successfully! ");

    let mut iteration = 0;

    println!("Starting the computation");

    //Compute 
    for iteration in 0..json_config["ComputationParameters"]["NumberOfIterations"].as_u64().unwrap() 
    {
        struct_array = computations::perform_computation_step(&struct_array, &json_config, iteration).expect("Computation step failed");
        println!("Iteration {} completed", iteration);

        if iteration % json_config["ComputationParameters"]["OutputFrequency"].as_u64().unwrap() == 0 
        {
            let filename_out = format!("{}/{}.in", json_config["StepPaths"].as_str().unwrap(), iteration.to_string());
            println!("Writing down intermediate output {}",filename_out);

            match write_read_file::write_condition_file(filename_out, &struct_array) {
                Ok(_) => println!("File saved successfully"),
                Err(e) => eprintln!("Errore during saving {}", e),
            }
        }
    }

    println!("Computation finished");

    let filename_out = format!("{}//{}.in", json_config["StepPaths"].as_str().unwrap(), iteration.to_string());
    println!("Writing down final output {}",filename_out);


    match write_read_file::write_condition_file(filename_out, &struct_array) {
        Ok(_) => println!("File saved successfully"),
        Err(e) => eprintln!("Errore during saving {}", e),
    }


}