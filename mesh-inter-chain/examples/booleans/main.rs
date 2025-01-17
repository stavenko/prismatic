use core::f32;
use std::{fs, path::PathBuf};

use clap::Parser;
use math::{BaseOrigin, Quaternion, Vector3};
use mesh_inter_chain::{
    geometry::GeometryDyn,
    indexes::{
        aabb::Aabb,
        geo_index::{geo_object::GeoObject, index::GeoIndex},
    },
};

use itertools::Itertools;

use shapes::{Plane, Rect};

#[derive(Parser)]
pub struct Command {
    #[arg(long)]
    pub output_path: PathBuf,
}

fn rib_unification_1(file_root: PathBuf) -> anyhow::Result<String> {
    let mut index = GeoIndex::new(Aabb::from_points(&[
        Vector3::new(-10_f32, -10_f32, -10_f32),
        Vector3::new(15_f32, 14_f32, 10_f32),
    ]))
    .debug_svg_path(file_root.clone())
    .input_polygon_min_rib_length(0.05_f32)
    .points_precision(0.001_f32);

    let zero = BaseOrigin::new();

    let mesh_one = index.new_mesh();

    Plane::centered(zero.clone(), 1.0, 1.0, 1).polygonize(mesh_one.make_mut_ref(&mut index), 0)?;

    let mesh_two = index.new_mesh();

    Plane::centered(zero.clone().offset_x(1.5_f32), 2.0, 2.0, 1)
        .polygonize(mesh_two.make_mut_ref(&mut index), 0)?;

    Ok(index.scad())
}

fn rib_unification_2(file_root: PathBuf) -> anyhow::Result<String> {
    let mut index = GeoIndex::new(Aabb::from_points(&[
        Vector3::new(-10_f32, -10_f32, -10_f32),
        Vector3::new(15_f32, 14_f32, 10_f32),
    ]))
    .debug_svg_path(file_root.clone())
    .input_polygon_min_rib_length(0.05_f32)
    .points_precision(0.001_f32);

    let mesh_one = index.new_mesh();
    let zero = BaseOrigin::new();

    Plane::centered(zero.clone(), 2.0, 2.0, 1).polygonize(mesh_one.make_mut_ref(&mut index), 0)?;

    let mesh_two = index.new_mesh();
    Plane::centered(zero.clone().offset_x(1.5_f32), 1.0, 1.0, 1)
        .polygonize(mesh_two.make_mut_ref(&mut index), 0)?;

    Ok(index.scad())
}

fn cut_planes(file_root: PathBuf) -> anyhow::Result<String> {
    let mut index = GeoIndex::new(Aabb::from_points(&[
        Vector3::new(-10_f32, -10_f32, -10_f32),
        Vector3::new(15_f32, 14_f32, 10_f32),
    ]))
    .debug_svg_path(file_root.clone())
    .input_polygon_min_rib_length(0.05_f32)
    .points_precision(0.001_f32);

    let zero = BaseOrigin::new();
    let rotated = zero
        .clone()
        .rotate_axisangle(Vector3::x() * f32::consts::FRAC_PI_2);

    let mesh_one = index.new_mesh();
    Plane::centered(zero, 1.0, 1.0, 1).polygonize(mesh_one.make_mut_ref(&mut index), 0)?;

    let mesh_two = index.new_mesh();
    Plane::centered(rotated, 2.0, 2.0, 1).polygonize(mesh_two.make_mut_ref(&mut index), 0)?;

    Ok(index.scad())
}

fn overlap_in_center(file_root: PathBuf) -> anyhow::Result<String> {
    let mut index = GeoIndex::new(Aabb::from_points(&[
        Vector3::new(-10_f32, -10_f32, -10_f32),
        Vector3::new(15_f32, 14_f32, 10_f32),
    ]))
    .debug_svg_path(file_root.clone())
    .input_polygon_min_rib_length(0.05_f32)
    .points_precision(0.001_f32);

    let zero = BaseOrigin::new();

    let small_box = index.new_mesh();
    Rect::centered(zero.clone().offset_x(1.0), 1.0, 1.0, 1.0)
        .polygonize(small_box.make_mut_ref(&mut index), 0)?;
    let big_box = index.new_mesh();
    Rect::centered(zero, 2.0, 2.0, 2.0).polygonize(big_box.make_mut_ref(&mut index), 0)?;

    let to_remove = [
        small_box
            .make_ref(&index)
            .front_of(big_box.make_ref(&index)),
        big_box.make_ref(&index).back_of(small_box.make_ref(&index)),
    ]
    .into_iter()
    .flatten()
    .collect_vec();

    for p in to_remove {
        p.make_mut_ref(&mut index).remove();
    }
    Ok(index.scad())
}

fn overlap_touching_edge(file_root: PathBuf) -> anyhow::Result<String> {
    let mut index = GeoIndex::new(Aabb::from_points(&[
        Vector3::new(-10_f32, -10_f32, -10_f32),
        Vector3::new(15_f32, 14_f32, 10_f32),
    ]))
    .debug_svg_path(file_root.clone())
    .input_polygon_min_rib_length(0.05_f32)
    .points_precision(0.001_f32);

    let zero = BaseOrigin::new();

    let smaller_box = Rect::centered(zero.clone().offset_x(1.0).offset_y(0.5), 1.0, 1.0, 1.0);

    let bigger_box = Rect::centered(zero, 2.0, 2.0, 2.0);

    let small = index.new_mesh();
    smaller_box.polygonize(small.make_mut_ref(&mut index), 0)?;
    let big = index.new_mesh();
    bigger_box.polygonize(big.make_mut_ref(&mut index), 0)?;

    let remove = [
        index.select_polygons(
            small,
            big,
            mesh_inter_chain::indexes::geo_index::index::PolygonFilter::Front,
        ),
        index.select_polygons(
            big,
            small,
            mesh_inter_chain::indexes::geo_index::index::PolygonFilter::Back,
        ),
        index.select_polygons(
            big,
            small,
            mesh_inter_chain::indexes::geo_index::index::PolygonFilter::Shared,
        ),
        index.select_polygons(
            small,
            big,
            mesh_inter_chain::indexes::geo_index::index::PolygonFilter::Shared,
        ),
    ]
    .concat();

    for poly in remove {
        poly.make_mut_ref(&mut index).remove();
    }

    index.move_all_polygons(big, small);
    big.make_mut_ref(&mut index).remove();

    Ok(index.scad())
}

fn overlap_touching_edge_with_opposite_polygons(file_root: PathBuf) -> anyhow::Result<String> {
    let mut index = GeoIndex::new(Aabb::from_points(&[
        Vector3::new(-10_f32, -10_f32, -10_f32),
        Vector3::new(15_f32, 14_f32, 10_f32),
    ]))
    .debug_svg_path(file_root.clone())
    .input_polygon_min_rib_length(0.05_f32)
    .points_precision(0.001_f32);

    let zero = BaseOrigin::new();

    let smaller_box = Rect::build()
        .origin(zero.clone().offset_z(-1.0))
        .align_z(shapes::Align::Pos)
        .build();

    let cutter = Plane::centered(zero.clone().offset_z(0.9), 10.0, 10.0, 1);
    let bigger_box = Rect::centered(zero, 2.0, 2.0, 2.0);

    let big = index.new_mesh();
    bigger_box.polygonize(big.make_mut_ref(&mut index), 0)?;
    let cut = index.new_mesh();
    cutter.polygonize(cut.make_mut_ref(&mut index), 0)?;
    for p in [
        cut.make_ref(&index).all_polygons(),
        big.make_ref(&index).front_of(cut.make_ref(&index)),
    ]
    .into_iter()
    .flatten()
    {
        p.make_mut_ref(&mut index).remove();
    }

    let smal = index.new_mesh();
    smaller_box
        .polygonize(smal.make_mut_ref(&mut index), 0)
        .unwrap();

    let shared_of_big = big.make_ref(&index).shared_with(smal.make_ref(&index));
    let shared_of_cut = smal.make_ref(&index).shared_with(big.make_ref(&index));
    for p in shared_of_cut {
        p.make_mut_ref(&mut index).remove();
    }
    for p in shared_of_big {
        p.make_mut_ref(&mut index).remove();
    }

    Ok(index.scad())
}

fn complex_cut(file_root: PathBuf) -> anyhow::Result<String> {
    let mut index = GeoIndex::new(Aabb::from_points(&[
        Vector3::new(-10_f32, -10_f32, -10_f32),
        Vector3::new(15_f32, 14_f32, 10_f32),
    ]))
    .debug_svg_path(file_root.clone())
    .input_polygon_min_rib_length(0.05_f32)
    .points_precision(0.001_f32);

    let zero = BaseOrigin::new();

    let platform = Rect::centered(zero.clone(), 1.0, 1.0, 0.2);

    let cutter = Rect::centered(zero.clone().offset_x(0.2).offset_y(0.2), 1.0, 1.0, 1.0);

    let platform_mesh = index.new_mesh();
    platform.polygonize(platform_mesh.make_mut_ref(&mut index), 0)?;

    let cutter_mesh = index.new_mesh();
    cutter.polygonize(cutter_mesh.make_mut_ref(&mut index), 0)?;

    let remove = [
        index.select_polygons(
            cutter_mesh,
            platform_mesh,
            mesh_inter_chain::indexes::geo_index::index::PolygonFilter::Front,
        ),
        index.select_polygons(
            platform_mesh,
            cutter_mesh,
            mesh_inter_chain::indexes::geo_index::index::PolygonFilter::Back,
        ),
    ]
    .concat();

    for polygon in &remove {
        polygon.make_mut_ref(&mut index).remove();
    }

    for polygon in cutter_mesh.make_ref(&index).all_polygons() {
        polygon.make_mut_ref(&mut index).flip();
    }

    let matter = Rect::centered(zero.offset_x(-0.95), 1.0, 0.6, 0.2);

    index.move_all_polygons(cutter_mesh, platform_mesh);
    let matter_mesh = index.new_mesh();

    matter.polygonize(matter_mesh.make_mut_ref(&mut index), 0)?;

    let remove = [
        index.select_polygons(
            matter_mesh,
            platform_mesh,
            mesh_inter_chain::indexes::geo_index::index::PolygonFilter::Back,
        ),
        index.select_polygons(
            platform_mesh,
            matter_mesh,
            mesh_inter_chain::indexes::geo_index::index::PolygonFilter::Back,
        ),
    ]
    .concat();

    for polygon in &remove {
        polygon.make_mut_ref(&mut index).remove();
    }

    index.move_all_polygons(matter_mesh, platform_mesh);

    Ok(index.scad())
}

/*
fn smaller_by_bigger(file_root: PathBuf) -> anyhow::Result<Vec<String>> {
    let mut index = GeoIndex::new(Aabb::from_points(&[
        Vector3::new(-10_f32, -10_f32, -10_f32),
        Vector3::new(15_f32, 14_f32, 10_f32),
    ]))
    .debug_svg_path(file_root.clone())
    .input_polygon_min_rib_length(0.05_f32)
    .points_precision(0.001_f32);

    let zz = Vector3::z();
    let yy = Vector3::y();
    let xx = yy.cross(&zz).normalize();

    let zero = BaseOrigin::new();

    let smaller_box = Rect::centered(zero.clone(), 1.0, 1.0, 1.0);

    let bigger_box = index.save_mesh(
        shapes::rect(_zero_basis, 2.0, 2.0, 2.0)
            .into_iter()
            .map(Cow::Owned),
    );

    let smaller_box = index.save_mesh(smaller_box.into_iter().map(Cow::Owned));
    let smaller_box_mut = index.get_mutable_mesh(smaller_box);

    let result = smaller_box_mut.boolean_union(bigger_box).remove(0);

    let filename = "smaller_by_bigger.stl";
    let path = file_root.join(filename);
    let mut writer = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .unwrap();

    stl_io::write_stl(&mut writer, result.into_iter())?;
    Ok(vec![filename.into()])
}
*/

fn two_identical_boxes_with_overlapped_side_and_rotated(
    file_root: PathBuf,
) -> anyhow::Result<String> {
    let mut index = GeoIndex::new(Aabb::from_points(&[
        Vector3::new(-10_f32, -10_f32, -10_f32),
        Vector3::new(15_f32, 14_f32, 10_f32),
    ]))
    .debug_svg_path(file_root.clone())
    .input_polygon_min_rib_length(0.05_f32)
    .points_precision(0.001_f32);

    let x_basis_one = BaseOrigin::new();
    let box_one = index.new_mesh();
    let box_two = index.new_mesh();
    Rect::centered(x_basis_one, 1.0, 1.0, 1.0).polygonize(box_one.make_mut_ref(&mut index), 0)?;

    let rot = Quaternion::from_scaled_axis(Vector3::x() * std::f32::consts::FRAC_PI_4);

    let x_basis_two = BaseOrigin::new().offset_x(0.5).rotate(rot);

    Rect::centered(x_basis_two, 1.0, 1.0, 1.0).polygonize(box_two.make_mut_ref(&mut index), 0)?;

    let remove = [
        index.select_polygons(
            box_one,
            box_two,
            mesh_inter_chain::indexes::geo_index::index::PolygonFilter::Back,
        ),
        index.select_polygons(
            box_one,
            box_two,
            mesh_inter_chain::indexes::geo_index::index::PolygonFilter::Back,
        ),
    ]
    .concat();

    for polygon in &remove {
        polygon.make_mut_ref(&mut index).remove();
    }

    index.move_all_polygons(box_two, box_one);

    Ok(index.scad())
}

fn two_identical_boxes_one_with_one_common_side_rotated(
    file_root: PathBuf,
) -> anyhow::Result<String> {
    let mut index = GeoIndex::new(Aabb::from_points(&[
        Vector3::new(-10_f32, -10_f32, -10_f32),
        Vector3::new(15_f32, 14_f32, 10_f32),
    ]))
    .debug_svg_path(file_root.clone())
    .input_polygon_min_rib_length(0.05_f32)
    .points_precision(0.001_f32);

    let x_basis_one = BaseOrigin::new();
    let box_one = index.new_mesh();
    let box_two = index.new_mesh();
    Rect::centered(x_basis_one, 1.0, 1.0, 1.0).polygonize(box_one.make_mut_ref(&mut index), 0)?;

    let rot = Quaternion::from_scaled_axis(Vector3::x() * std::f32::consts::FRAC_PI_4);

    let x_basis_two = BaseOrigin::new().offset_x(1.0).rotate(rot);

    Rect::centered(x_basis_two, 1.0, 1.0, 1.0).polygonize(box_two.make_mut_ref(&mut index), 0)?;

    let remove = [
        index.select_polygons(
            box_one,
            box_two,
            mesh_inter_chain::indexes::geo_index::index::PolygonFilter::Back,
        ),
        index.select_polygons(
            box_one,
            box_two,
            mesh_inter_chain::indexes::geo_index::index::PolygonFilter::Back,
        ),
    ]
    .concat();

    for polygon in &remove {
        polygon.make_mut_ref(&mut index).remove();
    }

    index.move_all_polygons(box_two, box_one);

    Ok(index.scad())
}
fn two_identical_boxes_one_with_one_common_side(file_root: PathBuf) -> anyhow::Result<String> {
    let mut index = GeoIndex::new(Aabb::from_points(&[
        Vector3::new(-10_f32, -10_f32, -10_f32),
        Vector3::new(15_f32, 14_f32, 10_f32),
    ]))
    .debug_svg_path(file_root.clone())
    .input_polygon_min_rib_length(0.05_f32)
    .points_precision(0.001_f32);

    let x_basis_one = BaseOrigin::new();
    let box_one = index.new_mesh();
    let box_two = index.new_mesh();
    Rect::centered(x_basis_one, 1.0, 1.0, 1.0).polygonize(box_one.make_mut_ref(&mut index), 0)?;

    let x_basis_two = BaseOrigin::new().offset_x(1.0);

    Rect::centered(x_basis_two, 1.0, 1.0, 1.0).polygonize(box_two.make_mut_ref(&mut index), 0)?;

    let remove = [
        index.select_polygons(
            box_one,
            box_two,
            mesh_inter_chain::indexes::geo_index::index::PolygonFilter::Back,
        ),
        index.select_polygons(
            box_one,
            box_two,
            mesh_inter_chain::indexes::geo_index::index::PolygonFilter::Back,
        ),
    ]
    .concat();

    for polygon in &remove {
        polygon.make_mut_ref(&mut index).remove();
    }

    index.move_all_polygons(box_two, box_one);

    Ok(index.scad())
}

fn two_identical_boxes_one_with_overlap(file_root: PathBuf) -> anyhow::Result<String> {
    let mut index = GeoIndex::new(Aabb::from_points(&[
        Vector3::new(-10_f32, -10_f32, -10_f32),
        Vector3::new(15_f32, 14_f32, 10_f32),
    ]))
    .debug_svg_path(file_root.clone())
    .input_polygon_min_rib_length(0.05_f32)
    .points_precision(0.001_f32);

    let x_basis_one = BaseOrigin::new();
    let box_one = index.new_mesh();
    let box_two = index.new_mesh();
    Rect::centered(x_basis_one, 1.0, 1.0, 1.0).polygonize(box_one.make_mut_ref(&mut index), 0)?;

    let x_basis_two = BaseOrigin::new().offset_x(0.8);

    Rect::centered(x_basis_two, 1.0, 1.0, 1.0).polygonize(box_two.make_mut_ref(&mut index), 0)?;

    let remove = [
        index.select_polygons(
            box_one,
            box_two,
            mesh_inter_chain::indexes::geo_index::index::PolygonFilter::Back,
        ),
        index.select_polygons(
            box_one,
            box_two,
            mesh_inter_chain::indexes::geo_index::index::PolygonFilter::Back,
        ),
    ]
    .concat();

    for polygon in &remove {
        polygon.make_mut_ref(&mut index).remove();
    }

    index.move_all_polygons(box_two, box_one);

    Ok(index.scad())
}

fn two_identical_boxes_one_shifted_in_plane(file_root: PathBuf) -> anyhow::Result<String> {
    let mut index = GeoIndex::new(Aabb::from_points(&[
        Vector3::new(-10_f32, -10_f32, -10_f32),
        Vector3::new(15_f32, 14_f32, 10_f32),
    ]))
    .debug_svg_path(file_root.clone())
    .input_polygon_min_rib_length(0.05_f32)
    .points_precision(0.001_f32);

    let x_basis_one = BaseOrigin::new();
    let box_one = index.new_mesh();
    let box_two = index.new_mesh();
    Rect::centered(x_basis_one, 1.0, 1.0, 1.0).polygonize(box_one.make_mut_ref(&mut index), 0)?;

    let x_basis_two = BaseOrigin::new().offset_x(0.8).offset_z(0.7);

    Rect::centered(x_basis_two, 1.0, 1.0, 1.0).polygonize(box_two.make_mut_ref(&mut index), 0)?;

    let remove = [
        index.select_polygons(
            box_one,
            box_two,
            mesh_inter_chain::indexes::geo_index::index::PolygonFilter::Back,
        ),
        index.select_polygons(
            box_one,
            box_two,
            mesh_inter_chain::indexes::geo_index::index::PolygonFilter::Back,
        ),
    ]
    .concat();

    for polygon in &remove {
        polygon.make_mut_ref(&mut index).remove();
    }

    index.move_all_polygons(box_two, box_one);

    Ok(index.scad())
}

fn two_identical_boxes_one_shifted_in_space(file_root: PathBuf) -> anyhow::Result<String> {
    let mut index = GeoIndex::new(Aabb::from_points(&[
        Vector3::new(-10_f32, -10_f32, -10_f32),
        Vector3::new(15_f32, 14_f32, 10_f32),
    ]))
    .debug_svg_path(file_root.clone())
    .input_polygon_min_rib_length(0.05_f32)
    .points_precision(0.001_f32);

    let x_basis_one = BaseOrigin::new();
    let box_one = index.new_mesh();
    let box_two = index.new_mesh();
    Rect::centered(x_basis_one, 1.0, 1.0, 1.0).polygonize(box_one.make_mut_ref(&mut index), 0)?;

    let x_basis_two = BaseOrigin::new().offset_x(0.6).offset_z(0.7).offset_y(0.6);

    Rect::centered(x_basis_two, 1.0, 1.0, 1.0).polygonize(box_two.make_mut_ref(&mut index), 0)?;

    let remove = [
        index.select_polygons(
            box_one,
            box_two,
            mesh_inter_chain::indexes::geo_index::index::PolygonFilter::Back,
        ),
        index.select_polygons(
            box_one,
            box_two,
            mesh_inter_chain::indexes::geo_index::index::PolygonFilter::Back,
        ),
    ]
    .concat();

    for polygon in &remove {
        polygon.make_mut_ref(&mut index).remove();
    }

    index.move_all_polygons(box_two, box_one);

    Ok(index.scad())
}

fn smaller_box_cutted_by_bigger(file_root: PathBuf) -> anyhow::Result<String> {
    let mut index = GeoIndex::new(Aabb::from_points(&[
        Vector3::new(-10_f32, -10_f32, -10_f32),
        Vector3::new(15_f32, 14_f32, 10_f32),
    ]))
    .debug_svg_path(file_root.clone())
    .input_polygon_min_rib_length(0.05_f32)
    .points_precision(0.001_f32);

    let x_basis_one = BaseOrigin::new();
    let x_basis_two = BaseOrigin::new().offset_z(0.5);
    let box_one = index.new_mesh();
    let box_two = index.new_mesh();

    Rect::centered(x_basis_one, 1.0, 1.0, 1.0).polygonize(box_one.make_mut_ref(&mut index), 0)?;
    Rect::centered(x_basis_two, 2.5, 2.5, 0.5).polygonize(box_two.make_mut_ref(&mut index), 0)?;

    let remove = [
        index.select_polygons(
            box_one,
            box_two,
            mesh_inter_chain::indexes::geo_index::index::PolygonFilter::Back,
        ),
        index.select_polygons(
            box_two,
            box_one,
            mesh_inter_chain::indexes::geo_index::index::PolygonFilter::Front,
        ),
    ]
    .concat();

    for polygon in box_two.make_ref(&index).all_polygons() {
        polygon.make_mut_ref(&mut index).flip();
    }

    for polygon in &remove {
        polygon.make_mut_ref(&mut index).remove();
    }

    index.move_all_polygons(box_two, box_one);
    Ok(index.scad())
}

fn smaller_box_cutted_by_longer(file_root: PathBuf) -> anyhow::Result<String> {
    let mut index = GeoIndex::new(Aabb::from_points(&[
        Vector3::new(-10_f32, -10_f32, -10_f32),
        Vector3::new(15_f32, 14_f32, 10_f32),
    ]))
    .debug_svg_path(file_root.clone())
    .input_polygon_min_rib_length(0.05_f32)
    .points_precision(0.001_f32);

    let x_basis_one = BaseOrigin::new();
    let x_basis_two = BaseOrigin::new().offset_z(0.5);
    let box_one = index.new_mesh();
    let box_two = index.new_mesh();

    Rect::centered(x_basis_one, 1.0, 1.0, 1.0).polygonize(box_one.make_mut_ref(&mut index), 0)?;
    Rect::centered(x_basis_two, 0.25, 0.25, 3.5).polygonize(box_two.make_mut_ref(&mut index), 0)?;

    let remove = [
        index.select_polygons(
            box_one,
            box_two,
            mesh_inter_chain::indexes::geo_index::index::PolygonFilter::Back,
        ),
        index.select_polygons(
            box_two,
            box_one,
            mesh_inter_chain::indexes::geo_index::index::PolygonFilter::Front,
        ),
    ]
    .concat();

    for polygon in box_two.make_ref(&index).all_polygons() {
        polygon.make_mut_ref(&mut index).flip();
    }

    for polygon in &remove {
        polygon.make_mut_ref(&mut index).remove();
    }

    index.move_all_polygons(box_two, box_one);
    Ok(index.scad())
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Command::parse();

    fs::create_dir_all(cli.output_path.clone())?;
    let meshes = [
        rib_unification_1(cli.output_path.clone())?,
        rib_unification_2(cli.output_path.clone())?,
        cut_planes(cli.output_path.clone())?,
        overlap_in_center(cli.output_path.clone())?,
        overlap_touching_edge(cli.output_path.clone())?,
        overlap_touching_edge_with_opposite_polygons(cli.output_path.clone())?,
        complex_cut(cli.output_path.clone())?,
        two_identical_boxes_with_overlapped_side_and_rotated(cli.output_path.clone())?,
        two_identical_boxes_one_with_one_common_side_rotated(cli.output_path.clone())?,
        two_identical_boxes_one_with_overlap(cli.output_path.clone())?,
        two_identical_boxes_one_with_one_common_side(cli.output_path.clone())?,
        two_identical_boxes_one_shifted_in_plane(cli.output_path.clone())?,
        two_identical_boxes_one_shifted_in_space(cli.output_path.clone())?,
        smaller_box_cutted_by_bigger(cli.output_path.clone())?,
        smaller_box_cutted_by_longer(cli.output_path.clone())?,
    ];

    let grid: i32 = 4;
    let grid_size = 5.0;
    let mut scad = Vec::new();
    'outer: for w in 0..grid {
        let x = grid_size * (w as f32 - (grid as f32 / 2.0));
        for h in 0..grid {
            let y = grid_size * (h as f32 - (grid as f32 / 2.0));
            let i = h + (w * grid);
            if let Some(mesh) = meshes.get(i as usize) {
                scad.push(format!("translate(v=[{}, {}, 0]) {{ {} }};", x, y, mesh));
            } else {
                break 'outer;
            }
        }
    }

    let file_content = scad.join("\n");

    fs::write(cli.output_path.join("tot.scad"), file_content).unwrap();

    Ok(())
}
