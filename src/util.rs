use bevy::prelude::Vec2;

pub fn lerp (a : f32, b : f32, t : f32) -> f32 {
	a + (b - a) * t
}

pub fn aabb (a_min : Vec2, a_max : Vec2, b_min : Vec2, b_max : Vec2) -> bool {
	   a_min.x < b_max.x
	&& a_max.x > b_min.x
	&& a_min.y < b_max.y
	&& a_max.y > b_min.y
}

pub fn swept_aabb (a_min : Vec2, a_max : Vec2, b_min : Vec2, b_max : Vec2, v : Vec2) -> (usize, f32) {
	let entry_x;
	let entry_y;
	let exit_x;
	let exit_y;
	
	if v.x == 0. {
		if a_min.x < b_max.x && b_min.x < a_max.x {
			entry_x = -f32::INFINITY;
			exit_x = f32::INFINITY;
		} else {
			return (0, 0.);
		}
	} else {
		let entry_dist_x = if v.x > 0. {
			b_min.x - a_max.x
		} else {
			a_min.x - b_max.x
		};
		
		entry_x = entry_dist_x / v.x.abs();
		
		let exit_dist_x = if v.x > 0. {
			b_max.x - a_min.x
		} else {
			a_max.x - b_min.x
		};
		
		exit_x = exit_dist_x / v.x.abs();
	}
	
	if v.y == 0. {
		if a_min.y < b_max.y && b_min.y < a_max.y {
			entry_y = -f32::INFINITY;
			exit_y = f32::INFINITY;
		} else {
			return (0, 0.);
		}
	} else {
		let entry_dist_y = if v.y > 0. {
			b_min.y - a_max.y
		} else {
			a_min.y - b_max.y
		};
		
		entry_y = entry_dist_y / v.y.abs();
		
		let exit_dist_y = if v.y > 0. {
			b_max.y - a_min.y
		} else {
			a_max.y - b_min.y
		};
		
		exit_y = exit_dist_y / v.y.abs();
	}
	
	if entry_x > exit_y || entry_y > exit_x {
		return (0, 0.);
	}
	
	return (
		if entry_x > entry_y { 1 } else { 2 }, // 1: sides, 2: top/btm
		f32::max(entry_x, entry_y),
	);
}
