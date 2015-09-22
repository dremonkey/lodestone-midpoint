/// The main crate for lodestone-midpoint
/// 
/// ## Overview
/// 
/// Calculates the midpoint along a great circle path between two points.
/// 
/// # Arguments
/// * `point1` - FeaturePoint
/// * `point2` - FeaturePoint

use std::f64::consts::PI;

// Third party crates
extern crate lodestone_point;

use lodestone_point::FeaturePoint;

pub extern fn midpoint(
    point1: &FeaturePoint,
    point2: &FeaturePoint) -> FeaturePoint {

  let coords1 = point1.coordinates();
  let coords2 = point2.coordinates();

  let lat1 = coords1[1].to_radians();
  let lng1 = coords1[0].to_radians();
  let lat2 = coords2[1].to_radians();
  let lng2 = coords2[0].to_radians();

  let delta_lng = lng2 - lng1;
  let bx = lat2.cos() * delta_lng.cos();
  let by = lat2.cos() * delta_lng.sin();

  // calculate the midpoint

  let dlat = (lat1.sin() + lat2.sin()).atan2(
              ((lat1.cos() + bx) * (lat1.cos() + bx) + by * by).sqrt()
             );
           
  let mut dlng = lng1 + by.atan2(lat1.cos() + bx);

  dlng = (dlng + 3.0 * PI) % (2.0 * PI) - PI; // normalise to -180..+180°

  FeaturePoint::new(vec![dlng.to_degrees(), dlat.to_degrees()])
}

#[cfg(test)]
mod tests {
  use lodestone_point::FeaturePoint;
  use super::midpoint;
  
  #[test]
  fn test_midpoint() {
    let pt1 = FeaturePoint::new(vec![0.0, 0.0]);
    let pt2 = FeaturePoint::new(vec![1.0, 0.0]);
    let expected = FeaturePoint::new(vec![0.5, 0.0]);
    let midpoint = midpoint(&pt1, &pt2);

    assert_eq!(expected, midpoint);
  }
}