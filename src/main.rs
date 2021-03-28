mod transform;
use crate::transform::Vec2 as V2;

fn main() {
	
	let v1 = V2::new(4,8);
	
	println!("{}", v1 * V2::new(2, 2));
	
}
