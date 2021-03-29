mod transform;
use transform::Vec2 as V2;

fn main() {
	
	let v1: V2<i32> = V2::new(6,2);
	println!("({}).mag() = {}", v1, v1.mag2());
	
}
