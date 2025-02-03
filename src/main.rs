use uniswap_math::Q64x96;

fn main() {
    let q = Q64x96::from_float(1.0);
    println!("Q number: {:?}", q);
    println!("Q number as float: {:?}", q.to_float());
}
