use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};

use math::{Scalar, Tensor, Vector3};
use num_traits::{One, Zero};
use path::{GetPosition, Path};
use surface::{BoundedFourSurface, EdgeTensor};

fn main() -> Result<(), anyhow::Error> {
    let left = Path::build()
        .start(SuperPoint {
            point: Vector3 {
                x: -1_f32,
                y: -1_f32,
                z: 0.0,
            },
            side_dir: Vector3::new(0.0, 0.4, 0.1_f32),
        })
        .line_to(SuperPoint {
            point: Vector3 {
                x: -1_f32,
                y: 1.0,
                z: 0.0,
            },
            side_dir: Vector3::new(0.0, 0.0, 1_f32),
        })
        .build();

    let top = Path::build()
        .start(SuperPoint {
            point: Vector3 {
                x: -1_f32,
                y: 1.0,
                z: 0.0,
            },
            side_dir: Vector3::new(0.0, 0.0, 1_f32),
        })
        .line_to(SuperPoint {
            point: Vector3 {
                x: 1.0,
                y: 1.0,
                z: 0.0,
            },
            side_dir: Vector3::new(0.0, 0.0, 1_f32),
        })
        .build();

    let right = Path::build()
        .start(SuperPoint {
            point: Vector3 {
                x: 1.0,
                y: 1.0,
                z: 0.0,
            },
            side_dir: Vector3::new(0.0, 0.0, 1_f32),
        })
        .line_to(SuperPoint {
            point: Vector3 {
                x: 1.0,
                y: -1_f32,
                z: 0.0,
            },
            side_dir: Vector3::new(0.0, 0.0, 1_f32),
        })
        .build();

    let bottom = Path::build()
        .start(SuperPoint {
            point: Vector3 {
                x: 1.0,
                y: -1_f32,
                z: 0.0,
            },
            side_dir: Vector3::new(0.0, 0.0, 1_f32),
        })
        .line_to(SuperPoint {
            point: Vector3 {
                x: -1_f32,
                y: -1_f32,
                z: 0.0,
            },
            side_dir: Vector3::new(0.0, 0.0, 1_f32),
        })
        .build();

    let fs = BoundedFourSurface {
        left,
        top,
        right,
        bottom,
        invert_triangles: false,
        triangle_regularity: 10.0,
    };

    let mut faces = Vec::new();
    let mut pts = Vec::new();
    for f in fs.polygonize() {
        let mut new_face = Vec::new();
        for p in f {
            pts.push(p);
            new_face.push(pts.len() - 1);
        }
        faces.push(format!(
            "[{}, {}, {}]",
            new_face[0], new_face[1], new_face[2]
        ));
    }

    let faces = faces.join(",");
    let pts = pts
        .into_iter()
        .map(|v| format!("[{}, {}, {}]", v.x, v.y, v.z))
        .collect::<Vec<_>>()
        .join(",");

    let scad = format!("polyhedron(points =[{pts}], faces = [{faces}]);");
    let button_hull = format!("translate(v=[0, 0, 0]) {{ {scad} }};");
    let home = std::env::var("HOME").unwrap();
    std::fs::write(format!("{home}/Desktop/example.scad"), button_hull).unwrap();
    Ok(())
}

#[derive(Clone, Copy, Debug)]
pub struct SuperPoint<T: Scalar> {
    pub side_dir: Vector3<T>,
    pub point: Vector3<T>,
}

impl<T> GetPosition for SuperPoint<T>
where
    T: Scalar,
{
    type Position = Vector3<T>;

    fn get_position(&self) -> Self::Position {
        self.point
    }

    fn get_position_mut(&mut self) -> &mut Self::Position {
        &mut self.point
    }
}

impl<T: Scalar + 'static> EdgeTensor for SuperPoint<T> {
    type Vector = Vector3<T>;

    fn get_point(&self) -> Self::Vector {
        self.point
    }

    fn get_edge_dir(&self) -> Self::Vector {
        self.side_dir
    }
}

impl<T: Scalar + 'static> Tensor for SuperPoint<T> {
    type Scalar = T;

    fn magnitude(&self) -> <Self as Tensor>::Scalar {
        let dot = self.point.x.powi(2) + self.point.y.powi(2) + self.point.z.powi(2);
        dot.sqrt()
    }
}

impl<T: Scalar> Div for SuperPoint<T>
where
    Vector3<T>: Div<Vector3<T>, Output = Vector3<T>>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            side_dir: self.side_dir / rhs.side_dir,
            point: self.point / rhs.point,
        }
    }
}

impl<T: Scalar> Div<T> for SuperPoint<T>
where
    Vector3<T>: Div<T, Output = Vector3<T>>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            side_dir: self.side_dir / rhs,
            point: self.point / rhs,
        }
    }
}

impl<T: Scalar> Mul<T> for SuperPoint<T>
where
    Vector3<T>: Mul<T, Output = Vector3<T>>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            side_dir: self.side_dir * rhs,
            point: self.point * rhs,
        }
    }
}

impl<T: Scalar> Mul for SuperPoint<T>
where
    Vector3<T>: Mul<Vector3<T>, Output = Vector3<T>>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            side_dir: self.side_dir * rhs.side_dir,
            point: self.point * rhs.point,
        }
    }
}

impl<T: Scalar> MulAssign for SuperPoint<T>
where
    Vector3<T>: MulAssign,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.point *= rhs.point;
        self.side_dir *= rhs.side_dir;
    }
}
impl<T: Scalar> AddAssign for SuperPoint<T>
where
    Vector3<T>: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.point += rhs.point;
        self.side_dir += rhs.side_dir;
    }
}

impl<T: Scalar> SubAssign for SuperPoint<T>
where
    Vector3<T>: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.point -= rhs.point;
        self.side_dir -= rhs.side_dir;
    }
}

impl<T: Scalar + 'static> Add<Self> for SuperPoint<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            side_dir: self.side_dir + rhs.side_dir,
            point: self.point + rhs.point,
        }
    }
}
impl<T: Scalar + 'static> Sub<Self> for SuperPoint<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            side_dir: self.side_dir - rhs.side_dir,
            point: self.point - rhs.point,
        }
    }
}

impl<T: Scalar + 'static> Zero for SuperPoint<T> {
    fn zero() -> Self {
        Self {
            side_dir: Vector3::zero(),
            point: Vector3::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.point.is_zero() && self.side_dir.is_zero()
    }
}

impl<T: Scalar> One for SuperPoint<T> {
    fn one() -> Self {
        Self {
            side_dir: Vector3::one(),
            point: Vector3::one(),
        }
    }
}
