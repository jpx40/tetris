use std::collections::HashMap;
use std::collections::HashSet;

use bevy::prelude::*;

use crate::common::*;
use crate::piece::*;
use crate::stats::*;

// game board Width Height
pub const COL_COUNT: u8 = 10;
pub const ROW_COUNT: u8 = 20;
// Side length of square block
pub const BLOCK_LENGTH: f32 = 30.0;
// TODO sticker rounded corners
// Square square sticker side length
pub const BLOCK_STICKER_LENGTH: f32 = 28.0;

// game board Boundary thickness
pub const BORDER_THICKNESS: f32 = 10.0;
pub const BORDER_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

// 方块
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Block {
    pub x: i32,
    pub y: i32,
}

impl Block {
    pub fn translation(&self) -> Vec3 {
        // The xy origin of the box is the lower left corner
        // Block x ranges from 0-9, block y ranges from 0-19
        // 10*20 blocks
        Vec3 {
            x: (self.x as f32 - (COL_COUNT as f32 / 2.0) + 0.5) * BLOCK_LENGTH,
            y: (self.y as f32 - (ROW_COUNT as f32 / 2.0) + 0.5) * BLOCK_LENGTH,
            z: 0.0,
        }
    }
}

impl From<[i32; 2]> for Block {
    fn from([x, y]: [i32; 2]) -> Self {
        Block { x, y }
    }
}

#[derive(Debug, Resource)]
pub struct RemovePieceComponentTimer(pub Timer);

pub fn setup_game_board(mut commands: Commands) {
    // The origin of the three-dimensional coordinates is at the center of the board
    // left border
    let half_col_count = COL_COUNT as f32 / 2.0;
    let half_raw_count = ROW_COUNT as f32 / 2.0;
    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3 {
                x: -half_col_count * BLOCK_LENGTH - BORDER_THICKNESS / 2.0,
                ..default()
            },
            scale: Vec3 {
                x: BORDER_THICKNESS,
                y: ROW_COUNT as f32 * BLOCK_LENGTH + 2.0 * BORDER_THICKNESS,
                z: 0.0,
            },
            ..default()
        },
        sprite: Sprite {
            color: BORDER_COLOR,
            ..default()
        },
        ..default()
    });
    // right border
    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3 {
                x: half_col_count * BLOCK_LENGTH + BORDER_THICKNESS / 2.0,
                ..default()
            },
            scale: Vec3 {
                x: BORDER_THICKNESS,
                y: ROW_COUNT as f32 * BLOCK_LENGTH + 2.0 * BORDER_THICKNESS,
                z: 0.0,
            },
            ..default()
        },
        sprite: Sprite {
            color: BORDER_COLOR,
            ..default()
        },
        ..default()
    });
    // upper border
    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3 {
                y: half_raw_count * BLOCK_LENGTH + BORDER_THICKNESS / 2.0,
                ..default()
            },
            scale: Vec3 {
                x: COL_COUNT as f32 * BLOCK_LENGTH,
                y: BORDER_THICKNESS,
                z: 0.0,
            },
            ..default()
        },
        sprite: Sprite {
            color: BORDER_COLOR,
            ..default()
        },
        ..default()
    });
    // lower border
    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3 {
                y: -half_raw_count * BLOCK_LENGTH - BORDER_THICKNESS / 2.0,
                ..default()
            },
            scale: Vec3 {
                x: COL_COUNT as f32 * BLOCK_LENGTH,
                y: BORDER_THICKNESS,
                z: 0.0,
            },
            ..default()
        },
        sprite: Sprite {
            color: BORDER_COLOR,
            ..default()
        },
        ..default()
    });
}

// When the piece moves to the bottom, remove the piece component
pub fn remove_piece_component(
    mut commands: Commands,
    q_piece_blocks: Query<(Entity, &Movable), With<PieceType>>,
    mut timer: ResMut<RemovePieceComponentTimer>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if !q_piece_blocks.is_empty() && !q_piece_blocks.iter().last().unwrap().1.can_down {
        if !q_piece_blocks.iter().last().unwrap().1.can_down {
            timer.0.tick(time.delta());
        } else {
            // When unable to move down, gain the ability to move down again by moving left and right.
            timer.0.reset();
        }
    }
    let mut reset_timer = false;
    for (entity, movable) in &q_piece_blocks {
        // After reaching the bottom, you can still move left and right in a short time
        if !movable.can_down {
            // When you reach the bottom, press the down key to skip the timer and start a new piece directly.
            if timer.0.just_finished() || keyboard_input.pressed(KeyCode::Down) {
                commands.entity(entity).remove::<PieceType>();
                reset_timer = true;
            }
        }
    }
    if reset_timer {
        timer.0.reset();
    }
}

// Check if there is a successful row
pub fn check_full_line(
    mut commands: Commands,
    game_audios: Res<GameAudios>,
    mut score: ResMut<Score>,
    mut lines: ResMut<Lines>,
    mut query: Query<(Entity, &mut Block, &mut Transform), Without<PieceType>>,
) {
    let mut y_to_x_set_map: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (_, block, _) in &query {
        if y_to_x_set_map.contains_key(&block.y) {
            let x_set = y_to_x_set_map.get_mut(&block.y).unwrap();
            x_set.insert(block.x);
        } else {
            let mut x_set = HashSet::new();
            x_set.insert(block.x);
            y_to_x_set_map.insert(block.y, x_set);
        }
    }
    let mut full_lines = Vec::new();
    for (y, x_set) in y_to_x_set_map.iter() {
        if x_set.len() == COL_COUNT as usize {
            full_lines.push(y);
        }
    }
    if full_lines.len() > 0 {
        dbg!(full_lines.len());
        commands.spawn(AudioBundle {
            source: game_audios.line_clear.clone(),
            ..default()
        });
    }
    // The number of rows increases
    lines.0 += full_lines.len() as u32;
    // Score increases
    score.0 += match full_lines.len() {
        0 => 0,
        1 => 100,
        2 => 200,
        3 => 400,
        4 => 800,
        _ => 1000,
    };

    // Eliminate rows
    let mut despawn_entities = Vec::new();
    for line_no in full_lines.iter() {
        let line_no = **line_no;
        for (entity, block, _) in &mut query {
            if block.y == line_no {
                despawn_entities.push(entity);
                commands.entity(entity).despawn();
            }
        }
    }
    // Eliminate the block above the row and move it down as a whole
    full_lines.sort();
    full_lines.reverse();
    for line_no in full_lines.iter() {
        for (entity, mut block, mut transform) in &mut query {
            if !despawn_entities.contains(&entity) && block.y > **line_no {
                info!("down block: {:?}, line_no: {}", block, line_no);
                block.y -= 1;
                transform.translation = block.translation();
            }
        }
    }
}

// Eliminate the block above the row and move it down as a whole
pub fn check_game_over(
    mut commands: Commands,
    game_audios: Res<GameAudios>,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
    query: Query<&Block, Without<PieceType>>,
) {
    let mut max_block_y = 0;
    for block in &query {
        if block.y > max_block_y {
            max_block_y = block.y;
        }
    }
    // info!("max_block_y: {}", max_block_y);
    if max_block_y >= 19 {
        commands.spawn(AudioBundle {
            source: game_audios.gameover.clone(),
            ..default()
        });
        app_state.set(AppState::GameOver);
        game_state.set(GameState::GameQuited);
    }
}

pub fn clear_game_board(mut commands: Commands, query: Query<Entity, With<Block>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
