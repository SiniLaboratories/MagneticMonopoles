pub fn perform_computation_step(struct_array: &Vec<super::structs::Point>, json_config: &serde_json::Value, iteration: u64
) -> Result<Vec<super::structs::Point>, Box<dyn std::error::Error>> 
{
    let nx = json_config["GridParameters"]["nx"].as_u64().unwrap() as usize;
    let ny = json_config["GridParameters"]["ny"].as_u64().unwrap() as usize;
    let nz = json_config["GridParameters"]["nz"].as_u64().unwrap() as usize;

    let ntot = nx*ny*nz;

    let dt = json_config["ComputationParameters"]["dt"].as_f64().unwrap();

    let mut out_array = vec![crate::structs::Point {..Default::default()}; ntot];
    
    let f1 = f(&struct_array,&json_config)?;

    //TODO: parallelize this for loop, it can be run parallelized without problems
    for i in 0..ntot 
    {
        out_array[i] = struct_array[i] + f1[i]*dt;
    }

    //TODO: Implement Runge Kutta 4

    Ok(out_array)
}



pub fn f(struct_array: &Vec<super::structs::Point>, json_config: &serde_json::Value
) -> Result<Vec<super::structs::DtPoint>, Box<dyn std::error::Error>>
{
    let nx = json_config["GridParameters"]["nx"].as_u64().unwrap() as usize;
    let ny = json_config["GridParameters"]["ny"].as_u64().unwrap() as usize;
    let nz = json_config["GridParameters"]["nz"].as_u64().unwrap() as usize;

    let ntot = nx*ny*nz;

    let dx = json_config["GridParameters"]["dx"].as_f64().unwrap();
    let dy = json_config["GridParameters"]["dy"].as_f64().unwrap();
    let dz = json_config["GridParameters"]["dz"].as_f64().unwrap();

    let c = json_config["PhysicalParameters"]["c"].as_f64().unwrap();
    let m_ep = json_config["PhysicalParameters"]["m_ep"].as_f64().unwrap();
    let m_em = json_config["PhysicalParameters"]["m_em"].as_f64().unwrap();
    let m_mp = json_config["PhysicalParameters"]["m_mp"].as_f64().unwrap();
    let m_mm = json_config["PhysicalParameters"]["m_mm"].as_f64().unwrap();

    let relativistic = json_config["ComputationParameters"]["relativistic"].as_bool().unwrap();

    let mut out_array = vec![crate::structs::DtPoint {..Default::default()}; ntot];

    //TODO: parallelize this for loop. It should be able to run flawlessly parallelized
    for i in 0..ntot 
    {
        let (ix,iy,iz) = crate::index_conversion::index_to_3d(i,nx,ny,nz);

        let i_xp1 = crate::index_conversion::index_from_3d(ix+1,iy,iz,nx,ny,nz);
        let i_xp2 = crate::index_conversion::index_from_3d(ix+2,iy,iz,nx,ny,nz);
        let i_xm1 = crate::index_conversion::index_from_3d(ix-1,iy,iz,nx,ny,nz);
        let i_xm2 = crate::index_conversion::index_from_3d(ix-2,iy,iz,nx,ny,nz);

        let i_yp1 = crate::index_conversion::index_from_3d(ix,iy+1,iz,nx,ny,nz);
        let i_yp2 = crate::index_conversion::index_from_3d(ix,iy+2,iz,nx,ny,nz);
        let i_ym1 = crate::index_conversion::index_from_3d(ix,iy-1,iz,nx,ny,nz);
        let i_ym2 = crate::index_conversion::index_from_3d(ix,iy-1,iz,nx,ny,nz);

        let i_zp1 = crate::index_conversion::index_from_3d(ix,iy,iz+1,nx,ny,nz);
        let i_zp2 = crate::index_conversion::index_from_3d(ix,iy,iz+2,nx,ny,nz);
        let i_zm1 = crate::index_conversion::index_from_3d(ix,iy,iz-1,nx,ny,nz);
        let i_zm2 = crate::index_conversion::index_from_3d(ix,iy,iz-2,nx,ny,nz);
        

        //dE/dt --> rotor of B & j_b
        let dBz_dy = 0.0;
        let dBy_dz = 0.0;
        let dBz_dx = 0.0;
        let dBx_dz = 0.0;
        let dBy_dx = 0.0;
        let dBx_dy = 0.0;
        let rotB_x = dBz_dy - dBy_dz;
        let rotB_y = dBz_dx - dBx_dz;
        let rotB_z = dBy_dx - dBx_dy;

        //NOTE: j_e/m_xyz could be implemented inside a struct as function megas, would be easyer. 
        //the total current is determined by two different sus types of flux

        let j_e_x = 0.0;
        let j_e_y = 0.0;
        let j_e_z = 0.0;

        out_array[i].E[0] = c * (rotB_x) - 4 * pi * j_e_x; //dEx/dt = +- rotBx/c +- j_m_x/c
        out_array[i].E[1] = c * (rotB_y) - 4 * pi * j_e_y; //dEy/dt = +- rotBy/c +- j_m_y/c
        out_array[i].E[2] = c * (rotB_z) - 4 * pi * j_e_z; //dEz/dt = +- rotBz/c +- j_m_z/c

        //dB/dt --> rotor of E & j_e
        let dEz_dy = 0.0;
        let dEy_dz = 0.0;
        let dEz_dx = 0.0;
        let dEx_dz = 0.0;
        let dEy_dx = 0.0;
        let dEx_dy = 0.0;
        let rotE_x = dEz_dy - dEy_dz;
        let rotE_y = dEz_dx - dEx_dz;
        let rotE_z = dEy_dx - dEx_dy;

        let j_m_x = 0.0; 
        let j_m_y = 0.0;
        let j_m_z = 0.0;

        out_array[i].B[0] = c * (rotE_x) - 4 * pi * j_m_x; //dEx/dt = +- rotBx/c +- j_m_x/c
        out_array[i].B[1] = c * (rotE_y) - 4 * pi * j_m_y; //dEy/dt = +- rotBy/c +- j_m_y/c
        out_array[i].B[2] = c * (rotE_z) - 4 * pi * j_m_z; //dEz/dt = +- rotBz/c +- j_m_z/c
        

        //divegence of rho_em
        let div_j_em = 0.0;
        out_array[i].rho_em = -1*div_j_em;

        //divegence of j_ep
        let div_j_ep = 0.0;
        out_array[i].rho_ep = -1*div_j_ep;

        //divegence of j_mm
        let div_j_mm = 0.0;
        out_array[i].rho_mm = -1*div_j_mm;

        //divegence of j_mp
        let div_j_mp = 0.0;
        out_array[i].rho_mp = -1*div_j_mp;


        //lorentz force for em

        //lorentz force for ep

        //lorentz force for mm

        //lorentz force for mp

    }

    
    Ok(out_array)
}


/*
fn rot_B()

fn rot_E()

fn div_j_ep()
fn div_j_em()
fn div_j_mp()
fn div_j_mm()

fn vector_product()

fn lorentzian(velocity_vector,c)

*/