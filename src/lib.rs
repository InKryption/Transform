pub mod transform;


mod tests {
	
	use super::transform;
	pub use transform::Vector2;
	
	mod comparisons {
		
		#[test]
		fn equality() {
			use super::Vector2 as V2;
			
			assert!( V2::<u32>		{ x: 9u32, y: 7u32 } == V2{ x: 9u32, y: 7u32 } );
			assert!( V2::<f32>		{ x: 9f32, y: 7f32 } == V2{ x: 9f32, y: 7f32 } );
			assert!( V2::<f64,f32>	{ x: 9f64, y: 7f32 } == V2{ x: 9f64, y: 7f32 } );
			assert!( V2::<i16,i64>	{ x: 2i16, y: 5i64 } == V2{ x: 2i16, y: 5i64 } );
			
		}
		
	}
	
	mod constructors {
		
		#[test]
		fn construction_equivalencies() {
			
			use super::Vector2 as V2;
			
			let v1: V2<i32> = Default::default();
			assert_eq!( v1 , V2{ x: 0, y:0 } );
			
			let v2: V2<i32> = V2::iso(7);
			assert_eq!( v2 , V2{ x: 7, y: 7 } );
			
			let v3: V2 = V2::new(3,8);
			assert_eq!( v3 , V2{ x: 3, y: 8 } );
		}
	}

	mod logic {}
	
}
