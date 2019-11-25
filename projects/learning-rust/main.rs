// use std::fmt;

struct PointVector{
    x: f64,
    y: f64,
    z: f64
}

impl PointVector {
    fn distanceFromOrigin(&self) -> f64 {
        let PointVector { x, y, z } = self;
        let xyDistance: f64 = ((x.powi(2)) + y.powi(2)).sqrt();
        let xyzDistance: f64 = (xyDistance.powi(2) + z.powi(2)).sqrt();
        xyzDistance
    }

    fn distanceFromOtherPoint(&self, other: &PointVector) -> f64 {
        let PointVector { x: x0, y: y0, z: z0 } = self;
        let PointVector { x: x1, y: y1, z: z1 } = other;
        let xyDelta = ((x1 - x0).powi(2) + (y1 - y0).powi(2)).sqrt();
        let xyzDelta = (xyDelta.powi(2) + (z1 - z0).powi(2)).sqrt();
        xyzDelta
    }
}

struct PointVectorPlane(Vec<PointVector>);

impl PointVectorPlane {
    fn add(&mut self, p: PointVector) {
        let vec = &mut self.0;
        vec.push(p)
    }

    fn planeArea(&self) -> f64 {
        let vec = &self.0;
        if vec.len() < 3 {
            ()
        }
        // 1/2 base * height
        // Have base be distance between first and second points
        let base = vec[0].distanceFromOtherPoint(&vec[1]);
        let hypotenuse = vec[0].distanceFromOtherPoint(&vec[2]);
        let height = (hypotenuse.powi(2) - (base / 2.0).powi(2)).sqrt();
        0.5 * ( base * height )
    }

    fn print(&self) {
        for (index, pointVector) in self.0.iter().enumerate() {
            let PointVector { x, y, z } = pointVector;
            match index {
                0 => println!("{} {} {}: {}", x, y, z, pointVector.distanceFromOrigin()),
                _ => println!("{} {} {}: {}", x, y, z, pointVector.distanceFromOtherPoint(&self.0[index - 1]))
            }
        }
    }
}

fn main() {
    let mut p: PointVectorPlane = PointVectorPlane(vec!{});
    p.add(PointVector{ x: 0.0, y: 0.0, z: 0.0 });
    p.add(PointVector{ x: 0.0, y: 1.0, z: 1.0 });
    p.add(PointVector{ x: 1.0, y: 0.0, z: 1.0 });
    println!("Plane: {}", p.planeArea());
    p.print();
}
