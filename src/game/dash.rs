use std::time::Duration;

use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::AppSet;

use super::{
    input::PlayerAction,
    movement::{Movement, MovementController},
    spawn::player::Player,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<DashController>();
    app.add_systems(Update, record_dash_controller.in_set(AppSet::RecordInput));
    app.add_systems(
        Update,
        update_dash_controller_direction.in_set(AppSet::RecordInput),
    );

    app.register_type::<Dash>();
    app.add_systems(Update, apply_dash.in_set(AppSet::Update));
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct DashController {
    pub last_direction: Vec2,
    pub intent: bool,
}

impl DashController {
    pub fn new() -> DashController {
        DashController {
            last_direction: Vec2::X,
            intent: false,
        }
    }
}

fn record_dash_controller(
    action_state: Res<ActionState<PlayerAction>>,
    mut query: Query<&mut DashController, With<Player>>,
) {
    for mut dash_controller in &mut query {
        dash_controller.intent = action_state.just_pressed(&PlayerAction::Dash);
    }
}

fn update_dash_controller_direction(
    mut query: Query<(&mut DashController, &MovementController), With<Player>>,
) {
    for (mut dash_controller, movement_controller) in query.iter_mut() {
        if movement_controller.0.length_squared() > 0.05 {
            dash_controller.last_direction = movement_controller.0.normalize_or_zero();
        }
    }
}

#[derive(Reflect)]
pub enum DashState {
    Ready,
    Dashing(Duration),
    Landing(Duration),
    OnCooldown(Duration),
}

impl Default for DashState {
    fn default() -> Self {
        DashState::Ready
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Dash {
    pub state: DashState,
    pub speed: f32,
    pub duration: Duration,
    pub landing_duration: Duration,
    pub cooldown: Duration,
}

impl Dash {
    pub fn new(
        speed: f32,
        duration: Duration,
        landing_duration: Duration,
        cooldown: Duration,
    ) -> Dash {
        Dash {
            state: DashState::Ready,
            speed,
            duration,
            landing_duration,
            cooldown,
        }
    }

    fn apply_delta_time(&mut self, delta_time: Duration) {
        match &mut self.state {
            DashState::Dashing(duration) => {
                *duration = duration.saturating_sub(delta_time);
                if *duration <= Duration::ZERO {
                    self.state = DashState::Landing(self.landing_duration);
                }
            }
            DashState::Landing(duration) => {
                *duration = duration.saturating_sub(delta_time);
                if *duration <= Duration::ZERO {
                    self.state = DashState::OnCooldown(self.cooldown);
                }
            }
            DashState::OnCooldown(duration) => {
                *duration = duration.saturating_sub(delta_time);
                if *duration <= Duration::ZERO {
                    self.state = DashState::Ready;
                }
            }
            DashState::Ready => (),
        }
    }

    fn request_dash(&mut self) {
        if let DashState::Ready = self.state {
            self.state = DashState::Dashing(self.duration);
        }
    }
}

fn apply_dash(
    time: Res<Time>,
    mut query: Query<(
        &DashController,
        &mut Dash,
        &mut Movement,
        &mut LinearVelocity,
    )>,
) {
    for (controller, mut dash, mut movement, mut linear_velocity) in &mut query {
        dash.apply_delta_time(time.delta());

        if controller.intent {
            dash.request_dash()
        }

        match dash.state {
            DashState::Dashing(_) => {
                movement.toggle_control(false);
                linear_velocity.0 = dash.speed * controller.last_direction;
            }
            DashState::Landing(_) => {
                movement.toggle_control(false);
                linear_velocity.0 = Vec2::ZERO;
            }
            DashState::OnCooldown(_) => movement.toggle_control(true),
            DashState::Ready => movement.toggle_control(true),
        }
    }
}
