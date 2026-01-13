
//Function that converts index i to ix iy iz
pub fn index_to_3d(i:usize, nx:usize, ny:usize, _nz:usize) -> (usize, usize, usize)
{
    let iz = i / (nx*ny);
    let iy = (i - iz*nx*ny) / nx;
    let ix = i - iz*nx*ny - iy*nx;

    (ix, iy, iz)
}


//Function that converts ix iy iz to index i
pub fn index_from_3d(ix:usize, iy:usize, iz:usize, nx:usize, ny:usize, nz:usize) -> usize
{
    let ix2 = ix % nx;
    let iy2 = iy % ny;  
    let iz2 = iz % nz;
    let i = iz2*nx*ny + iy2*nx + ix2;

    i
}