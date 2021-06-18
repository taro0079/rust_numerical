#[macro_use]
extern crate peroxide;
use peroxide::fuga::*;

fn main() {
    let density :f64 = 8.96 * 10f64.pow(6f64);
    let k :f64 = 300.0;
    let c :f64 = 0.3;
    let dx :f64 = 0.005;
    let dt :f64 = 0.01;
    let xnum :usize = 100;
    let tnum :usize = 1000;
    
    let s:f64 = k / density / c / dx.pow(2.0) * dt;

    
    let mut v:Vec<Vec<f64>> = vec![vec![0.0; xnum];xnum];
    for i in 0..xnum {
        for j in 0..xnum {
            if j == i {
                v[i][j] = 1.0 - 2.0 * s;
            }
            else if j == i + 1 {
               v[i][j] = s;
            }
            else if i == j + 1 {
                v[i][j] = s;
            }
        }
    }
    let mat = py_matrix(v);
    
    let xx:Vec<Vec<f64>> = vec![vec![20.0; 1]; xnum];

    let mut xxmat = py_matrix(xx);
    
    let mut c:Vec<Vec<f64>> = vec![vec![0.0; 1]; xnum];
    c[0][0] = 300.0 * s;
    c[xnum-1][0] = 20.0 * s;
    let cmat = py_matrix(c);
    (mat.clone() * xxmat.clone() + cmat.clone()).print();
    println!("{:?}", cmat);
    for _ in 0..(tnum+1) {
        let newmat = mat.clone() * xxmat.clone() + cmat.clone() ;
        xxmat = newmat;
    }
    xxmat.print();
    

}
