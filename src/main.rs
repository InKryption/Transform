mod transform;
use crate::transform::Vec2 as V2;

fn main() {
	
	let v1: V2 = V2::new(4,8);
	let v2: V2 = V2::iso(3);
	let n1 = 5;
	println!("({}) - ({}) = ({})", v1, v2, v1 - v2);
	println!("({}) + {} = ({})", v1 - v2, n1, (v1 - v2) + n1);
	
}
