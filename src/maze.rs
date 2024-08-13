use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::Rng;

pub struct MazePlugin;

impl Plugin for MazePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup).add_plugins(TilemapPlugin);
    }
}
fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle: Handle<Image> = asset_server.load("tileset.png");

    let map_size = TilemapSize { x: 32, y: 32 };

    // Create a tilemap entity a little early.
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    // Eventually, we will insert the `TilemapBundle` bundle on the entity, which
    // will contain various necessary components, such as `TileStorage`.
    let tilemap_entity = commands.spawn_empty().id();

    // To begin creating the map we will need a `TileStorage` component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world. If you have multiple layers of tiles you would have a tilemap entity
    // per layer, each with their own `TileStorage` component.
    let mut tile_storage = TileStorage::empty(map_size);

    let wall_tile = TileTextureIndex(17);
    let floor_tile = TileTextureIndex(92);
    let mut maze = vec![wall_tile; map_size.count()];
    let mut floor_tiles: Vec<usize> = Vec::new();

    // let start = 1 + map_size.x as usize;
    // let end = ((map_size.x - 1) * map_size.y) as usize - 1;

    let end = map_size.count() - map_size.x as usize - 2;
    let start = map_size.x as usize + 1;

    let mut rng = rand::thread_rng();

    let max_depth = 10000;
    let mut depth = 0;

    floor_tiles.push(start);
    let mut current_pos = start;
    // let mut previous_pos = start;

    // Generate critical path
    while current_pos != end {
        let next_pos = match rng.gen_range(0..4) {
            0 => {
                // Left
                current_pos.saturating_sub(1)
            }
            1 => {
                // Up
                current_pos + map_size.x as usize
            }
            2 => {
                // Right
                current_pos + 1
            }
            3 => {
                // Down
                current_pos.saturating_sub(map_size.x as usize)
            }
            _ => unreachable!(),
        };

        if next_pos % map_size.x as usize == 0
            || next_pos / map_size.x as usize == 0
            || next_pos % (map_size.x as usize - 1) == 0
            || next_pos > (map_size.count() - map_size.y as usize)
        {
            continue;
        }

        // No path on outside edge
        // if next_pos >= map_size.count()
        //     || next_pos % map_size.x as usize == 0
        //     || next_pos % (map_size.x as usize - 1) == 0
        //     || next_pos >= (map_size.x * (map_size.y - 1)) as usize
        //     || next_pos <= map_size.x as usize
        // {
        //     continue;
        // }

        if let Some(start_of_loop) =
            floor_tiles.iter().enumerate().find_map(
                |(i, t)| {
                    if *t == next_pos {
                        Some(i)
                    } else {
                        None
                    }
                },
            )
        {
            // info!(
            //     "found loop, draining. len: {}, start_of_loop: {}",
            //     floor_tiles.len(),
            //     start_of_loop
            // );
            floor_tiles = floor_tiles.drain(..=start_of_loop).collect();
            current_pos = floor_tiles[start_of_loop];
            // previous_pos = floor_tiles[start_of_loop.saturating_sub(1)]
        } else {
            // let offset = map_size.x as usize;
            // #[rustfmt::skip]
            // let mut points_around_next_pos = vec![
            //     offset + next_pos - 1, offset + next_pos, offset + next_pos + 1,
            //     next_pos - 1,                                      next_pos + 1,
            //     offset - next_pos - 1, offset - next_pos, offset - next_pos + 1,
            // ];
            // points_around_next_pos.retain(|p| *p != current_pos && *p != previous_pos);
            //
            // if points_around_next_pos
            //     .iter()
            //     .any(|p| floor_tiles.contains(p))
            // {
            //     continue;
            // } else {
            // previous_pos = current_pos;
            current_pos = next_pos;
            floor_tiles.push(current_pos);
            // }
        }

        if depth == max_depth {
            info!("max depth critical path");
            depth = 0;
            floor_tiles.clear();
            current_pos = start;
            // previous_pos = start;
        } else {
            depth += 1;
        }
    }
    floor_tiles.push(end);

    info!("branching");

    // let num_branches = 40;
    // for _ in 0..num_branches {
    //     let width = map_size.x as usize;
    //     let height = map_size.y as usize;
    //
    //     let mut start = rng.gen_range(width + 1..width * (height - 1) - 1);
    //     let offset = map_size.x as usize;
    //     #[rustfmt::skip]
    //     let mut points_around_pos = vec![
    //         offset + start - 1, offset + start, offset + start + 1,
    //         start - 1,                   start,          start + 1,
    //         start - offset - 1, start - offset, start - offset + 1,
    //     ];
    //
    //     let branch_start_depth = 1000;
    //     let mut bsd = 0;
    //     while points_around_pos.iter().any(|p| {
    //         floor_tiles.contains(p)
    //             || start >= map_size.count()
    //             || start % map_size.x as usize == 0
    //             || start % (map_size.x as usize - 1) == 0
    //             || start > (map_size.x * (map_size.y - 1)) as usize
    //             || start < map_size.x as usize
    //     }) {
    //         if bsd == branch_start_depth {
    //             info!("maximum depth branch start");
    //             break;
    //         }
    //
    //         start = rng.gen_range(width..width * (height - 1) - 1);
    //         points_around_pos = vec![
    //             offset + start - 1,
    //             offset + start,
    //             offset + start + 1,
    //             start - 1,
    //             start,
    //             start + 1,
    //             start - offset - 1,
    //             start - offset,
    //             start - offset + 1,
    //         ];
    //         bsd += 1;
    //     }
    //
    //     if bsd == branch_start_depth {
    //         continue;
    //     }
    //
    //     let mut branch_floor_tiles = Vec::new();
    //     let max_depth = 10000;
    //     let mut depth = 0;
    //
    //     let mut current_pos = start;
    //     let mut previous_pos = start;
    //     branch_floor_tiles.push(start);
    //
    //     // loop {
    //     //     let next_pos = match rng.gen_range(0..4) {
    //     //         0 => {
    //     //             let left = current_pos.saturating_sub(1);
    //     //             if left % map_size.x as usize - 1 == 0 {
    //     //                 continue;
    //     //             } else {
    //     //                 left
    //     //             }
    //     //         }
    //     //         1 => {
    //     //             // Up
    //     //             current_pos + map_size.x as usize
    //     //         }
    //     //         2 => {
    //     //             // Right
    //     //             let left = current_pos + 1;
    //     //             if left % map_size.x as usize == 0 {
    //     //                 continue;
    //     //             } else {
    //     //                 left
    //     //             }
    //     //         }
    //     //         3 => {
    //     //             // Down
    //     //             current_pos.saturating_sub(map_size.x as usize)
    //     //         }
    //     //         _ => unreachable!(),
    //     //     };
    //     //
    //     //     if floor_tiles.contains(&next_pos) {
    //     //         break;
    //     //     }
    //     //
    //     //     // No path on outside edge
    //     //     if next_pos >= map_size.count()
    //     //         || next_pos % map_size.x as usize == 0
    //     //         || next_pos % map_size.x as usize - 1 == 0
    //     //         || next_pos > (map_size.x * (map_size.y - 1)) as usize
    //     //         || next_pos < map_size.x as usize
    //     //     {
    //     //         continue;
    //     //     }
    //     //
    //     //     if let Some(start_of_loop) = branch_floor_tiles.iter().enumerate().find_map(|(i, t)| {
    //     //         if *t == next_pos {
    //     //             Some(i)
    //     //         } else {
    //     //             None
    //     //         }
    //     //     }) {
    //     //         info!(
    //     //             "found loop, draining. len: {}, start_of_loop: {}",
    //     //             floor_tiles.len(),
    //     //             start_of_loop
    //     //         );
    //     //         branch_floor_tiles = branch_floor_tiles.drain(..=start_of_loop).collect();
    //     //         current_pos = branch_floor_tiles[start_of_loop];
    //     //         previous_pos = branch_floor_tiles[start_of_loop.saturating_sub(1)]
    //     //     } else {
    //     //         let offset = map_size.x as usize;
    //     //         #[rustfmt::skip]
    //     //         let mut points_around_next_pos = vec![
    //     //             offset + current_pos - 1, offset + current_pos, offset + current_pos + 1,
    //     //             current_pos - 1,                                         current_pos + 1,
    //     //             offset - current_pos - 1, offset - current_pos, offset - current_pos + 1,
    //     //         ];
    //     //         points_around_next_pos.retain(|p| *p != current_pos && *p != previous_pos);
    //     //
    //     //         if points_around_next_pos
    //     //             .iter()
    //     //             .any(|p| branch_floor_tiles.contains(p) || floor_tiles.contains(p))
    //     //         {
    //     //             continue;
    //     //         } else {
    //     //             previous_pos = current_pos;
    //     //             current_pos = next_pos;
    //     //             branch_floor_tiles.push(current_pos);
    //     //         }
    //     //     }
    //     //
    //     //     if depth == max_depth {
    //     //         branch_floor_tiles.clear();
    //     //         current_pos = start;
    //     //         previous_pos = start;
    //     //     } else {
    //     //         depth += 1;
    //     //     }
    //     // }
    //     // branch_floor_tiles.push(current_pos);
    //
    //     for index in branch_floor_tiles.into_iter() {
    //         floor_tiles.push(index);
    //     }
    // }

    for index in floor_tiles.into_iter() {
        maze[index as usize] = floor_tile;
    }

    // Spawn the elements of the tilemap.
    // Alternatively, you can use helpers::filling::fill_tilemap.
    for i in 0..maze.len() {
        let tile_pos = TilePos {
            x: i as u32 % map_size.x as u32,
            y: i as u32 / map_size.x as u32,
        };
        let tile = TileBundle {
            position: tile_pos,
            visible: TileVisible(true),
            tilemap_id: TilemapId(tilemap_entity),
            texture_index: {
                if i == start {
                    TileTextureIndex(147)
                } else if i == end {
                    TileTextureIndex(146)
                } else {
                    maze[i]
                }
            },
            ..Default::default()
        };
        // info!("{tile:#?}");
        let tile_entity = commands.spawn(tile).id();
        tile_storage.set(&tile_pos, tile_entity);
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        visibility: Visibility::Visible,
        ..Default::default()
    });
}

// fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let texture_handle: Handle<Image> = asset_server.load("tileset.png");
//
//     let map_size = TilemapSize { x: 32, y: 32 };
//
//     // Create a tilemap entity a little early.
//     // We want this entity early because we need to tell each tile which tilemap entity
//     // it is associated with. This is done with the TilemapId component on each tile.
//     // Eventually, we will insert the `TilemapBundle` bundle on the entity, which
//     // will contain various necessary components, such as `TileStorage`.
//     let tilemap_entity = commands.spawn_empty().id();
//
//     // To begin creating the map we will need a `TileStorage` component.
//     // This component is a grid of tile entities and is used to help keep track of individual
//     // tiles in the world. If you have multiple layers of tiles you would have a tilemap entity
//     // per layer, each with their own `TileStorage` component.
//     let mut tile_storage = TileStorage::empty(map_size);
//
//     let wall_tile = TileTextureIndex(17);
//     let floor_tile = TileTextureIndex(92);
//     let mut maze = vec![wall_tile; map_size.count()];
//     let mut floor_tiles: Vec<usize> = Vec::new();
//
//     // let start = 1 + map_size.x as usize;
//     // let end = ((map_size.x - 1) * map_size.y) as usize - 1;
//
//     let end = 990;
//     let start = 33;
//
//     let mut rng = rand::thread_rng();
//
//     let max_depth = 10000;
//     let mut depth = 0;
//
//     floor_tiles.push(start);
//     let mut current_pos = start;
//     let mut previous_pos = start;
//
//     // Generate critical path
//     while current_pos != end {
//         let next_pos = match rng.gen_range(0..4) {
//             0 => {
//                 let left = current_pos.saturating_sub(1);
//                 if left % map_size.x as usize - 1 == 0 {
//                     continue;
//                 } else {
//                     left
//                 }
//             }
//             1 => {
//                 // Up
//                 current_pos + map_size.x as usize
//             }
//             2 => {
//                 // Right
//                 let left = current_pos + 1;
//                 if left % map_size.x as usize == 0 {
//                     continue;
//                 } else {
//                     left
//                 }
//             }
//             3 => {
//                 // Down
//                 current_pos.saturating_sub(map_size.x as usize)
//             }
//             _ => unreachable!(),
//         };
//
//         // No path on outside edge
//         if next_pos >= map_size.count()
//             || next_pos % map_size.x as usize == 0
//             || next_pos % (map_size.x as usize - 1) == 0
//             || next_pos >= (map_size.x * (map_size.y - 1)) as usize
//             || next_pos <= map_size.x as usize
//         {
//             continue;
//         }
//
//         if let Some(start_of_loop) =
//             floor_tiles.iter().enumerate().find_map(
//                 |(i, t)| {
//                     if *t == next_pos {
//                         Some(i)
//                     } else {
//                         None
//                     }
//                 },
//             )
//         {
//             // info!(
//             //     "found loop, draining. len: {}, start_of_loop: {}",
//             //     floor_tiles.len(),
//             //     start_of_loop
//             // );
//             floor_tiles = floor_tiles.drain(..=start_of_loop).collect();
//             current_pos = floor_tiles[start_of_loop];
//             previous_pos = floor_tiles[start_of_loop.saturating_sub(1)]
//         } else {
//             let offset = map_size.x as usize;
//             #[rustfmt::skip]
//             let mut points_around_next_pos = vec![
//                 offset + next_pos - 1, offset + next_pos, offset + next_pos + 1,
//                 next_pos - 1,                                      next_pos + 1,
//                 offset - next_pos - 1, offset - next_pos, offset - next_pos + 1,
//             ];
//             points_around_next_pos.retain(|p| *p != current_pos && *p != previous_pos);
//
//             if points_around_next_pos
//                 .iter()
//                 .any(|p| floor_tiles.contains(p))
//             {
//                 continue;
//             } else {
//                 previous_pos = current_pos;
//                 current_pos = next_pos;
//                 floor_tiles.push(current_pos);
//             }
//         }
//
//         if depth == max_depth {
//             floor_tiles.clear();
//             current_pos = start;
//             previous_pos = start;
//         } else {
//             depth += 1;
//         }
//     }
//     floor_tiles.push(end);
//
//     info!("branching");
//
//     let num_branches = 40;
//     for _ in 0..num_branches {
//         let width = map_size.x as usize;
//         let height = map_size.y as usize;
//
//         let mut start = rng.gen_range(width + 1..width * (height - 1) - 1);
//         let offset = map_size.x as usize;
//         #[rustfmt::skip]
//         let mut points_around_pos = vec![
//             offset + start - 1, offset + start, offset + start + 1,
//             start - 1,                   start,          start + 1,
//             start - offset - 1, start - offset, start - offset + 1,
//         ];
//
//         let branch_start_depth = 1000;
//         let mut bsd = 0;
//         while points_around_pos.iter().any(|p| {
//             floor_tiles.contains(p)
//                 || start >= map_size.count()
//                 || start % map_size.x as usize == 0
//                 || start % (map_size.x as usize - 1) == 0
//                 || start > (map_size.x * (map_size.y - 1)) as usize
//                 || start < map_size.x as usize
//         }) {
//             if bsd == branch_start_depth {
//                 info!("maximum depth branch start");
//                 break;
//             }
//
//             start = rng.gen_range(width..width * (height - 1) - 1);
//             points_around_pos = vec![
//                 offset + start - 1,
//                 offset + start,
//                 offset + start + 1,
//                 start - 1,
//                 start,
//                 start + 1,
//                 start - offset - 1,
//                 start - offset,
//                 start - offset + 1,
//             ];
//             bsd += 1;
//         }
//
//         if bsd == branch_start_depth {
//             continue;
//         }
//
//         let mut branch_floor_tiles = Vec::new();
//         let max_depth = 10000;
//         let mut depth = 0;
//
//         let mut current_pos = start;
//         let mut previous_pos = start;
//         branch_floor_tiles.push(start);
//
//         // loop {
//         //     let next_pos = match rng.gen_range(0..4) {
//         //         0 => {
//         //             let left = current_pos.saturating_sub(1);
//         //             if left % map_size.x as usize - 1 == 0 {
//         //                 continue;
//         //             } else {
//         //                 left
//         //             }
//         //         }
//         //         1 => {
//         //             // Up
//         //             current_pos + map_size.x as usize
//         //         }
//         //         2 => {
//         //             // Right
//         //             let left = current_pos + 1;
//         //             if left % map_size.x as usize == 0 {
//         //                 continue;
//         //             } else {
//         //                 left
//         //             }
//         //         }
//         //         3 => {
//         //             // Down
//         //             current_pos.saturating_sub(map_size.x as usize)
//         //         }
//         //         _ => unreachable!(),
//         //     };
//         //
//         //     if floor_tiles.contains(&next_pos) {
//         //         break;
//         //     }
//         //
//         //     // No path on outside edge
//         //     if next_pos >= map_size.count()
//         //         || next_pos % map_size.x as usize == 0
//         //         || next_pos % map_size.x as usize - 1 == 0
//         //         || next_pos > (map_size.x * (map_size.y - 1)) as usize
//         //         || next_pos < map_size.x as usize
//         //     {
//         //         continue;
//         //     }
//         //
//         //     if let Some(start_of_loop) = branch_floor_tiles.iter().enumerate().find_map(|(i, t)| {
//         //         if *t == next_pos {
//         //             Some(i)
//         //         } else {
//         //             None
//         //         }
//         //     }) {
//         //         info!(
//         //             "found loop, draining. len: {}, start_of_loop: {}",
//         //             floor_tiles.len(),
//         //             start_of_loop
//         //         );
//         //         branch_floor_tiles = branch_floor_tiles.drain(..=start_of_loop).collect();
//         //         current_pos = branch_floor_tiles[start_of_loop];
//         //         previous_pos = branch_floor_tiles[start_of_loop.saturating_sub(1)]
//         //     } else {
//         //         let offset = map_size.x as usize;
//         //         #[rustfmt::skip]
//         //         let mut points_around_next_pos = vec![
//         //             offset + current_pos - 1, offset + current_pos, offset + current_pos + 1,
//         //             current_pos - 1,                                         current_pos + 1,
//         //             offset - current_pos - 1, offset - current_pos, offset - current_pos + 1,
//         //         ];
//         //         points_around_next_pos.retain(|p| *p != current_pos && *p != previous_pos);
//         //
//         //         if points_around_next_pos
//         //             .iter()
//         //             .any(|p| branch_floor_tiles.contains(p) || floor_tiles.contains(p))
//         //         {
//         //             continue;
//         //         } else {
//         //             previous_pos = current_pos;
//         //             current_pos = next_pos;
//         //             branch_floor_tiles.push(current_pos);
//         //         }
//         //     }
//         //
//         //     if depth == max_depth {
//         //         branch_floor_tiles.clear();
//         //         current_pos = start;
//         //         previous_pos = start;
//         //     } else {
//         //         depth += 1;
//         //     }
//         // }
//         // branch_floor_tiles.push(current_pos);
//
//         for index in branch_floor_tiles.into_iter() {
//             floor_tiles.push(index);
//         }
//     }
//
//     for index in floor_tiles.into_iter() {
//         maze[index as usize] = floor_tile;
//     }
//
//     // Spawn the elements of the tilemap.
//     // Alternatively, you can use helpers::filling::fill_tilemap.
//     for x in 0..map_size.x {
//         for y in 0..map_size.y {
//             let tile_pos = TilePos { x, y };
//             let tile = TileBundle {
//                 position: tile_pos,
//                 visible: TileVisible(true),
//                 tilemap_id: TilemapId(tilemap_entity),
//                 texture_index: {
//                     let index = (y * map_size.x + x % map_size.x) as usize;
//                     if index == start {
//                         TileTextureIndex(147)
//                     } else if index == end {
//                         TileTextureIndex(146)
//                     } else {
//                         maze[index]
//                     }
//                 },
//                 ..Default::default()
//             };
//             // info!("{tile:#?}");
//             let tile_entity = commands.spawn(tile).id();
//             tile_storage.set(&tile_pos, tile_entity);
//         }
//     }
//
//     let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
//     let grid_size = tile_size.into();
//     let map_type = TilemapType::default();
//
//     commands.entity(tilemap_entity).insert(TilemapBundle {
//         grid_size,
//         map_type,
//         size: map_size,
//         storage: tile_storage,
//         texture: TilemapTexture::Single(texture_handle),
//         tile_size,
//         transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
//         visibility: Visibility::Visible,
//         ..Default::default()
//     });
// }
