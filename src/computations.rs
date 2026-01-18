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
        

        //dB/dt --> rotor of E & j_e
        let rotB_x = 0.0;

        //dE/dt --> rotor of B & j_b

        //divegence of j_ep

        //divegence of j_em

        //divegence of j_mp

        //divegence of j_mm


        //lorentz force for ep

        //lorentz force for em

        //lorentz force for mp

        //lorentz force for mm

    }

    
    Ok(out_array)
}