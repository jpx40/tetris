use bevy::prelude::*;

use crate::{
    board::{Block, BLOCK_LENGTH},
    new_block_sprite,
    piece::{PieceQueue, PieceType},
    piece_shape, shift_piece,
};

// Scoreboard length and width
const STATS_BOARD_LENGTH: f32 = 220.0;
const STATS_BOARD_WIDTH: f32 = 50.0;

// Fraction
#[derive(Resource)]
pub struct Score(pub u32);
#[derive(Component)]
pub struct Scoreboard;

// Eliminate rows
#[derive(Resource)]
pub struct Lines(pub u32);
#[derive(Component)]
pub struct Linesboard;

// show next domino
#[derive(Debug, Resource)]
pub struct NextPieceType(pub Option<PieceType>);

#[derive(Debug, Component)]
pub struct NextPieceBoard;

pub fn setup_stats_boards(mut commands: Commands, windows: Query<&Window>) {
    // Calculate stats position by window size and board size
    let window = windows.single();
    // The position of the upper left corner of the gameboard on the window
    let gameboard_left_corner_pos = (
        window.width() / 2.0 - 5.0 * BLOCK_LENGTH,
        window.height() / 2.0 - 10.0 * BLOCK_LENGTH,
    );
    info!("gameboard_left_corner_pos: {:?}", gameboard_left_corner_pos);
    // Fraction
    commands
        .spawn(
            TextBundle::from_sections([
                TextSection::new(
                    "Score: ",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.5, 0.5, 1.0),
                        ..default()
                    },
                ),
                TextSection::new(
                    "0",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(1.0, 0.5, 0.5),
                        ..default()
                    },
                ),
            ]) // TODO: Need to fix position   https://docs.rs/bevy/latest/bevy/window/struct.WindowResolution.html
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(gameboard_left_corner_pos.1),
                left: Val::Px(gameboard_left_corner_pos.0 - STATS_BOARD_LENGTH),
                ..default()
            }),
        )
        .insert(Scoreboard);

    // Rows
    commands
        .spawn(
            TextBundle::from_sections([
                TextSection::new(
                    "Lines: ",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.5, 0.5, 1.0),
                        ..default()
                    },
                ),
                TextSection::new(
                    "0",
                    TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(1.0, 0.5, 0.5),
                        ..default()
                    },
                ),
            ]) // TODO: Need to fix position issue
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(gameboard_left_corner_pos.1 + STATS_BOARD_WIDTH),
                left: Val::Px(gameboard_left_corner_pos.0 - STATS_BOARD_LENGTH),
                ..default()
            }),
        )
        .insert(Linesboard);
}

pub fn update_scoreboard(score: Res<Score>, mut query: Query<&mut Text, With<Scoreboard>>) {
    let mut text = query.single_mut();
    text.sections[1].value = score.0.to_string();
}

pub fn update_linesboard(lines: Res<Lines>, mut query: Query<&mut Text, With<Linesboard>>) {
    let mut text = query.single_mut();
    text.sections[1].value = lines.0.to_string();
}

pub fn reset_score(mut score: ResMut<Score>) {
    score.0 = 0;
}

pub fn reset_lines(mut lines: ResMut<Lines>) {
    lines.0 = 0;
}

pub fn update_next_piece_board(
    mut commands: Commands,
    piece_queue: Res<PieceQueue>,
    mut next_piece_type: ResMut<NextPieceType>,
    query: Query<Entity, With<NextPieceBoard>>,
) {
    if next_piece_type.0.is_none()
        || piece_queue.0.front().unwrap().piece_type != next_piece_type.0.unwrap()
    {
        next_piece_type.0 = Some(piece_queue.0.front().unwrap().piece_type);
        // Destroy the original board
        for entity in &query {
            commands.entity(entity).despawn();
        }
        let piece_type = piece_queue.0.front().unwrap().piece_type;
        let color = piece_queue.0.front().unwrap().color;
        let blocks = shift_piece(piece_shape(piece_type), Some(8), Some(17));
        spawn_next_piece_board(&mut commands, blocks, color);
    }
}

pub fn spawn_next_piece_board(commands: &mut Commands, blocks: [Block; 4], color: Color) {
    let visibility = Visibility::Visible;
    commands
        .spawn(new_block_sprite(&blocks[0], color, visibility))
        .insert(NextPieceBoard);
    commands
        .spawn(new_block_sprite(&blocks[1], color, visibility))
        .insert(NextPieceBoard);
    commands
        .spawn(new_block_sprite(&blocks[2], color, visibility))
        .insert(NextPieceBoard);
    commands
        .spawn(new_block_sprite(&blocks[3], color, visibility))
        .insert(NextPieceBoard);
}

pub fn clear_next_piece_board(mut commands: Commands, query: Query<Entity, With<NextPieceBoard>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
