use std::{
    ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign},
    path::PathBuf,
};

use clap::Parser;
use math::{Scalar, Tensor, Vector3};
use num_traits::{One, Zero};
use path::{GetPosition, Path};
use surface::{EdgeTensor, SurfaceBetweenPathsBuilder};

#[derive(Parser)]
struct Opts {
    #[arg(long)]
    output_path: PathBuf,
}

fn main() -> Result<(), anyhow::Error> {
    let opts = Opts::parse();
    let left = Path::build()
        .start(SuperPoint {
            point: Vector3 {
                x: -1_f32,
                y: -1_f32,
                z: 0.0,
            },
            side_dir: Vector3::new(0.0, 0.0, 1_f32),
        })
        .line_to(SuperPoint {
            point: Vector3 {
                x: -1_f32,
                y: 0.1,
                z: 0.0,
            },
            side_dir: Vector3::new(0.0, 0.0, 1_f32),
        })
        .quad_3_to(
            SuperPoint {
                point: Vector3 {
                    x: -1.2_f32,
                    y: 0.8,
                    z: 0.0,
                },
                side_dir: Vector3::new(0.0, 0.0, 2_f32),
            },
            SuperPoint {
                point: Vector3 {
                    x: -1.0_f32,
                    y: 1.0,
                    z: 0.0,
                },
                side_dir: Vector3::new(0.0, 0.0, 1_f32),
            },
        )
        .build();

    let right = Path::build()
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
                x: 1.0,
                y: 1.0,
                z: 0.0,
            },
            side_dir: Vector3::new(0.0, 0.0, 2_f32),
        })
        .build();

    let fs = SurfaceBetweenPathsBuilder::default()
        .set_leading_path(left)
        .set_subdue_path(right)
        .set_inner_points_t(20)
        .set_inner_points_s(6)
        .set_leading_path_points_per_component(2)
        .set_subdue_path_points(11)
        .set_zero_border_points(8)
        .set_one_border_points(20)
        .set_one_path_padding(0.01)
        .set_zero_path_padding(0.01)
        .build();

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
    let f_path = opts.output_path.join("surface_between_paths.scad");
    println!("Scad file: {f_path:?}");
    std::fs::write(f_path, button_hull).unwrap();
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
