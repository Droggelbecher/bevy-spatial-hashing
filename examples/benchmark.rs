use std::hint::black_box;

use clap::Parser;
use rand::{rngs::StdRng, Rng, SeedableRng};

use bevy::app::AppExit;
use bevy::{app::ScheduleRunnerSettings, log::LogPlugin, math::vec2, prelude::*, utils::Duration};
use bevy_spatial_hashing::spatial_hashmap::{SpatialHashmap, SquareQuery};

const N_CIRCLES: i32 = 10000;
const BOTTOM_LEFT: Vec2 = vec2(-950., -500.);
const TOP_RIGHT: Vec2 = vec2(950., 500.);
const MIN_SPEED: Vec2 = vec2(-20., -20.);
const MAX_SPEED: Vec2 = vec2(20., 20.);

const FPS_INTERVAL: f32 = 1.0_f32;
const N_FPS_ROUNDS: i32 = 10;

#[derive(Debug, Resource, Parser)]
#[command(author, version, about, long_about = None)]
struct Experiment {
    #[arg(short, long)]
    collision_radius: f32,
    #[arg(short, long)]
    grid_size: Option<f32>,
    #[arg(short, long)]
    use_shm: bool,
}

fn main() {
    let experiment = Experiment::parse();

    let mut app = &mut App::new();
    app = app
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            0.0,
        )))
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin {
            filter: "info".into(),
            level: bevy::log::Level::DEBUG,
        })
        .add_startup_system(startup)
        .add_system(update_locations);

    if experiment.use_shm {
        app = app.add_system(collide_shm)
            .insert_resource(SHM {
                shm: SpatialHashmap::new(experiment.grid_size.unwrap())
            })
    } else {
        app = app.add_system(collide_naive);
    }

    app.add_system(fps)
        .insert_resource(ExperimentState::default())
        .insert_resource(experiment)
        .run();
}

#[derive(Debug, Component, Clone)]
pub struct Speed {
    pub v: Vec2,
}

#[derive(Debug, Resource)]
pub struct SHM {
    shm: SpatialHashmap,
}

#[derive(Component, Deref, DerefMut)]
pub struct FPSTimer(Timer);

#[derive(Resource)]
struct ExperimentState {
    frame_count: i32,
    rounds_left: i32,
}

impl Default for ExperimentState {
    fn default() -> Self {
        ExperimentState {
            frame_count: 0,
            rounds_left: N_FPS_ROUNDS,
        }
    }
}

fn startup(mut commands: Commands, mut shm: Option<ResMut<SHM>>) {
    let mut rng = StdRng::seed_from_u64(123);

    for _ in 0..N_CIRCLES {
        let x = rng.gen_range(BOTTOM_LEFT.x..TOP_RIGHT.x);
        let y = rng.gen_range(BOTTOM_LEFT.y..TOP_RIGHT.y);
        let vx = rng.gen_range(MIN_SPEED.x..MAX_SPEED.x);
        let vy = rng.gen_range(MIN_SPEED.y..MAX_SPEED.y);

        let entity = commands.spawn((
            Transform::from_translation(Vec3::new(x, y, 0.)),
            Speed { v: vec2(vx, vy) },
        ));

        // Insert into hashmap
        if let Some(ref mut s) = shm {
            s.shm.insert(vec2(x, y), entity.id());
        }
    }

    commands.spawn(FPSTimer(Timer::from_seconds(FPS_INTERVAL, TimerMode::Repeating)));
}

fn fps(
    time: Res<Time>,
    mut timers: Query<&mut FPSTimer>,
    mut state: ResMut<ExperimentState>,
    experiment: Res<Experiment>,
    mut exit: EventWriter<AppExit>,
) {
    state.frame_count += 1;

    for mut timer in &mut timers {
        if timer.tick(time.delta()).just_finished() {
            {
                println!(
                    "{}, {}, {}",
                    if experiment.use_shm { experiment.grid_size.unwrap() } else { -1.0_f32 },
                    experiment.collision_radius,
                    FPS_INTERVAL / (state.frame_count as f32)
                );
            }
            state.frame_count = 0;
            state.rounds_left -= 1;

            if state.rounds_left <= 0 {
                exit.send(AppExit)
            }
        }
    }
}

fn update_locations(
    mut moving: Query<(Entity, &mut Transform, &mut Speed)>,
    time: Res<Time>,
    mut shm: Option<ResMut<SHM>>,
    experiment: Res<Experiment>,
) {
    for (entity, mut transform, mut speed) in &mut moving {
        let new_translation = transform.translation + speed.v.extend(0.0) * time.delta_seconds();

        if experiment.use_shm {
            shm.as_mut().unwrap().shm.update(
                entity,
                transform.translation.truncate(),
                new_translation.truncate(),
            );
        }

        transform.translation = new_translation;

        if transform.translation.y < BOTTOM_LEFT.y || transform.translation.y > TOP_RIGHT.y {
            speed.v.y *= -1.;
        }
        if transform.translation.x < BOTTOM_LEFT.x || transform.translation.x > TOP_RIGHT.x {
            speed.v.x *= -1.;
        }
    }
}

fn collide_naive(moving: Query<&Transform>, experiment: Res<Experiment>) {
    // Naive Collide
    for transform in moving.iter() {
        for transform2 in moving.iter() {
            if (transform.translation.x - transform2.translation.x).abs()
                < experiment.collision_radius
                && (transform.translation.y - transform2.translation.y).abs()
                    < experiment.collision_radius
            {
                // Pretend to do something with the neighbor position so this doesnt get optimized
                // out
                black_box(transform2.translation);
            }
        }
    }
}

fn collide_shm(moving: Query<&Transform>, shm: ResMut<SHM>, experiment: Res<Experiment>) {
    for transform in moving.iter() {
        let query = SquareQuery::new(
            transform.translation.truncate(),
            experiment.collision_radius,
        );

        // Count neighbors
        for (_entity, _position) in shm.shm.query(query) {
            // Pretend to do something with the neighbor position so this doesnt get optimized
            // out
            black_box(_position);
        }
    }
}
