use rusty_engine::prelude::*;
use rand::prelude::*;

struct GameState {
    high_score: u32,
    score: u32,
    ferris_index: i32,
    spawn_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            high_score: 0,
            score: 0,
            ferris_index: 0,
            spawn_timer: Timer::from_seconds(2.0, true),
        }
    }
}

fn main() {
    // initialize the game
    let mut game = Game::new();

    game.window_settings(WindowDescriptor {
        title: "Hit the Ferrises!".to_string(),
        ..Default::default()
    });

    game.audio_manager.play_music(MusicPreset::Classy8Bit, 0.2);

    let player = game.add_sprite("player", SpritePreset::RacingCarGreen);
    player.translation = Vec2::new(0.0, 0.0);
    player.rotation = SOUTH_WEST;
    player.scale = 1.0;
    player.collision = true;

    let score = game.add_text("score", "Score: 0");
    score.translation = Vec2::new(520.0, 320.0);

    let high_score = game.add_text("high_score", "High Score: 0");
    high_score.translation = Vec2::new(-520.0, 320.0);

    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {

    // Quitting game
    if engine.keyboard_state.just_pressed(KeyCode::Q) {
        engine.should_exit = true;
    }

    // Making the two score display static at the corner
    let score = engine.texts.get_mut("score").unwrap();
    score.translation.x = engine.window_dimensions.x / 2.0 - 80.0;
    score.translation.y = engine.window_dimensions.y / 2.0 - 30.0;
    let high_score = engine.texts.get_mut("high_score").unwrap();
    high_score.translation.x = -engine.window_dimensions.x / 2.0 + 110.0;
    high_score.translation.y = engine.window_dimensions.y / 2.0 - 30.0;

    // Handling collisions
    for event in engine.collision_events.drain(..) {
        if event.state == CollisionState::Begin && event.pair.one_starts_with("player") {
            for label in [event.pair.0, event.pair.1] {
                if label != "player" {
                    engine.sprites.remove(&label);
                }
            }
            game_state.score += 1;
            let score = engine.texts.get_mut("score").unwrap();
            score.value = format!("Score: {}", game_state.score);

            if game_state.score > game_state.high_score {
                let high_score = engine.texts.get_mut("high_score").unwrap();
                game_state.high_score = game_state.score;
                high_score.value = format!("High Score: {}", game_state.high_score);
            }
        }
        engine.audio_manager.play_sfx(SfxPreset::Minimize1, 0.2);
    }

    // Handle player movement
    let player = engine.sprites.get_mut("player").unwrap();
    const MOVEMENT_SPEED: f32 = 200.0;
    
    if engine.keyboard_state.pressed(KeyCode::Up) {
        player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine.keyboard_state.pressed(KeyCode::Down) {
        player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine.keyboard_state.pressed(KeyCode::Left) {
        player.translation.x -= MOVEMENT_SPEED * engine.delta_f32;
    }
    if engine.keyboard_state.pressed(KeyCode::Right) {
        player.translation.x += MOVEMENT_SPEED * engine.delta_f32;
    }

    // handle mouse input
    if engine.mouse_state.just_pressed(MouseButton::Left) {
        if let Some(mouse_location) = engine.mouse_state.location() {
            let label = format!("ferris{}", game_state.ferris_index);
            game_state.ferris_index += 1;
            let cute_ferris = engine.add_sprite(label.clone(), "cute-ferris.png");
            cute_ferris.translation = mouse_location;
            cute_ferris.collision = true;
        }
    }

    // Randomly generates ferris
    if game_state.spawn_timer.tick(engine.delta).just_finished() {
        let label = format!("ferris{}", game_state.ferris_index);
        game_state.ferris_index += 1;
        let cute_ferris = engine.add_sprite(label.clone(), "cute-ferris.png");
        cute_ferris.translation.x = thread_rng().gen_range(-550.0..550.0);
        cute_ferris.translation.y = thread_rng().gen_range(-325.0..325.0);
        cute_ferris.collision = true;
    }

    // reset score
    if engine.keyboard_state.just_pressed(KeyCode::R) {
        game_state.score = 0;
        let score = engine.texts.get_mut("score").unwrap();
        score.value = "Score: 0".to_string();
    }
}
