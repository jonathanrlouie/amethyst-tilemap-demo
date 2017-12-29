use amethyst::ecs::{Component, DenseVecStorage};
use amethyst::renderer::PosTex;
use genmesh::generators::{Plane, SharedVertex, IndexedPolygon};
use genmesh::{Triangulate, Vertices};
use tiled;

pub fn generate_tilemap_plane(tilesize: u32, tilemap_width: u32, tilemap_height: u32) -> Vec<PosTex> {
    let plane = Plane::subdivide(tilemap_width as usize, tilemap_height as usize);

    let half_width = (tilesize * tilemap_width) as f32 / 2.0 ;
    let half_height = (tilesize * tilemap_height) as f32 / 2.0;

    let vertex_data: Vec<PosTex> = plane.shared_vertex_iter().map(|(raw_x, raw_y)| {

        let vertex_x = (half_width * raw_x).round();
        let vertex_y = (half_height * raw_y).round();

        let u_pos = (1.0 + raw_x) / 2.0;
        let v_pos = (1.0 + raw_y) / 2.0;

        let tilemap_x = (u_pos * tilemap_width as f32).round();
        let tilemap_y = (v_pos * tilemap_height as f32).round();

        PosTex {
            position: [vertex_x, vertex_y, 0.0],
            tex_coord: [tilemap_x, tilemap_height as f32 - tilemap_y]
        }
    }).collect();

    let indexed_vertex_data: Vec<PosTex> = plane.indexed_polygon_iter()
        .triangulate()
        .vertices()
        .map(|i| *vertex_data.get(i as usize).unwrap_or(&PosTex{position: [0., 0., 0.], tex_coord: [0., 0.]}))
        .collect();

    indexed_vertex_data
}

pub fn generate_tile_data(map: &tiled::Map, tileset_width: u32, tileset_height: u32) -> Vec<[f32; 4]> {
    let mut tiles = Vec::new();
    let layers = &map.layers;
    for layer in layers {
        for rows in &layer.tiles {
            for tile in rows {
                if *tile != 0 {
                    // subtract 1.0 from the x coordinate because the first gid of the tileset is 1
                    // this could be made cleaner
                    tiles.push([(*tile - 1) as f32 % tileset_width as f32, (tileset_height - 1)  as f32 - (((*tile - 1) / tileset_width) as f32), 0.0, 0.0]);
                } else {
                    tiles.push([0.0, 0.0, 0.0, 0.0]);
                }

            }
        }
    }
    tiles
}

#[derive(Clone)]
pub struct TilemapDimensions {
    pub width: u32,
    pub height: u32
}

impl Component for TilemapDimensions {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone)]
pub struct TilesheetDimensions {
    pub width: u32,
    pub height: u32
}

impl Component for TilesheetDimensions {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone)]
pub struct TilemapTiles {
    pub tiles: Vec<[f32; 4]>,
}

impl Component for TilemapTiles {
    type Storage = DenseVecStorage<Self>;
}