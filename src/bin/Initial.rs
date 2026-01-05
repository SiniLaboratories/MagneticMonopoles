mod write_read_file;

fn main() 
{
    println!("Generation of initial condition started");
    let args: Vec<String> = env::args().collect();

    println!("Example: plain wave propagation")

    println!("Loading configuration file...");
    println!("{}",args[0]);
    let json_config = write_read_file::import_configuration_file(args[0]).expect("Invalid JSON");

    println!(json_config["nx"]);

    //Load configuration file

}