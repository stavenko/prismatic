# mesh-inter-chain 

This is a pure Rust library to find intersections of several meshes. Each mesh
represented as list of polygons. Mesh could be closed (like `sphere` or `cube`),
or it may be not closed (like `plane` of `surface`. 

Intersecting meshes will have some command line, and in some cases we can make
filtering of polygons:
1. Polygons of some mesh, which are INSIDE other mesh
2. Polygons of some mesh, which are OUTSIDE other mesh.
3. Polygons, which share same `faces`, with same, or opposing normals.

After finding proper polygons, we may perform boolean operations.

## Usage:

There are several examples, when this repo downloaded, we can run:
```
cargo run --example booleans -- --output-path ~/projects/scad-files/booleans
```
This example will create scad file in given folder with some interesting
operations, which are possible with this library.

## Terminology

**Mesh** - collection of polygons. Could be closed, or none-closed.
**Polygon** - 2D object in space, which have **Face** and direction.
**Face** - ordered list of **segments**. Closed chain of segments.
**Segment** - Rib with direction
**Rib** - Item, consisting of two points.
**Point** - Indexed vector in 3D space, 
