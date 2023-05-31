use std::f64::consts::PI;
#[derive(Debug)]
pub struct Circle {
	pub center: Point,
	pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Self {
            center: Point::new(x, y),
            radius,
        }
    }
    
    pub fn diameter(&self) -> f64 { // returns the diameter of the circle.
        self.radius * 2.
    }

    
    pub fn area(&self) -> f64 { // returns the area of the circle.
        PI * self.radius.powf(2.)
    }

    
    pub fn intersect(&self, circle_bis: &Self) -> bool { // which returns true, if 2 circles intersect.
        let dist = self.center.distance(&circle_bis.center);
        let rad_sum = self.radius + circle_bis.radius;
        dist < rad_sum
    }

}

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
        }
    }
    pub fn distance(&self, point_bis: &Point) -> f64 {
        let dist_x = point_bis.x - self.x;
        let dist_y = point_bis.y - self.y;
        (dist_x.powf(2.) + dist_y.powf(2.)).sqrt()
    }
}

