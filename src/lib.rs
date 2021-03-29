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
		
		#[test]
		fn greater_lesser() {
			use super::Vector2 as V2;
			
			assert!( V2::<u32>		{ x: 9u32, y: 7u32 }  > V2{ x: 9u32, y: 7u32 } );
			assert!( V2::<u32>		{ x: 9u32, y: 7u32 }  < V2{ x: 9u32, y: 7u32 } );
			assert!( V2::<f32>		{ x: 9f32, y: 7f32 } == V2{ x: 9f32, y: 7f32 } );
			assert!( V2::<f64,f32>	{ x: 9f64, y: 7f32 } == V2{ x: 9f64, y: 7f32 } );
			assert!( V2::<i16,i64>	{ x: 2i16, y: 5i64 } == V2{ x: 2i16, y: 5i64 } );
		}
		
	}
	
	mod constructors {
		#[test]
		fn construction_equivalencies() {
			
			use super::Vector2 as V2;
			
			assert_eq!( V2::new(3,8)	, V2{ x: 3, y: 8 } );
			assert_eq!( V2::from(7)		, V2{ x: 7, y: 7 } );
			assert_eq!( V2::default()	, V2{ x: 0, y: 0 } );
			
		}
	}

	mod logic {}
	
}
