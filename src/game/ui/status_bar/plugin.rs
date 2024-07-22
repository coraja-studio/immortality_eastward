use super::material::StatusBarMaterial;
use crate::game::ui::percentage::AsPercentage;
use crate::game::ui::status_bar::definition::StatusBarDefinition;
use bevy::prelude::*;
use bevy::sprite::{Material2dPlugin, MaterialMesh2dBundle};
use std::marker::PhantomData;

pub trait PercentageComponent: Component + AsPercentage {}
impl<T: Component + AsPercentage> PercentageComponent for T {}

pub struct StatusBarPlugin<T: PercentageComponent>(PhantomData<T>);

impl<T: PercentageComponent> Default for StatusBarPlugin<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T: PercentageComponent> Plugin for StatusBarPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<StatusBarMaterial>::default())
            .add_systems(Update, despawn::<T>)
            .add_systems(
                PostUpdate,
                ((spawn::<T>), (follow_owner::<T>, update::<T>)).chain(),
            );
    }
}

#[derive(Component)]
pub struct StatusBarOwner(Entity);

#[derive(Bundle)]
pub struct StatusBarBundle {
    material_mesh_bundle: MaterialMesh2dBundle<StatusBarMaterial>,
    owner: StatusBarOwner,
}

fn spawn<T: PercentageComponent>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut status_bar_materials: ResMut<Assets<StatusBarMaterial>>,
    stat_bar_query: Query<(Ref<StatusBarDefinition<T>>, &T, &Transform, Entity)>,
) {
    for (status_bar_definition, percentage_component, transform, entity) in stat_bar_query.iter() {
        if !status_bar_definition.is_added() {
            continue;
        }
        println!("Spawning Health Bar!");
        commands.spawn(StatusBarBundle {
            material_mesh_bundle: MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::default()).into(),
                material: status_bar_materials.add(StatusBarMaterial {
                    foreground_color: status_bar_definition.foreground_color.into(),
                    background_color: status_bar_definition.background_color.into(),
                    percent: percentage_component.percentage().value(),
                }),
                transform: Transform {
                    translation: transform.translation + status_bar_definition.offset,
                    scale: Vec2::new(
                        status_bar_definition.size.width(),
                        status_bar_definition.size.height(),
                    )
                    .extend(1.0),
                    ..default()
                },
                ..default()
            },
            owner: StatusBarOwner(entity),
        });
    }
}

fn update<T: PercentageComponent>(
    mut status_bar_materials: ResMut<Assets<StatusBarMaterial>>,
    status_bar_query: Query<
        (&Handle<StatusBarMaterial>, &StatusBarOwner),
        Without<StatusBarDefinition<T>>,
    >,
    owner_percentage_component_query: Query<&T>,
) {
    for (material_handle, &StatusBarOwner(owner_entity)) in status_bar_query.iter() {
        let material = status_bar_materials
            .get_mut(material_handle)
            .expect("StatusBarMaterial missing");
        let Ok(health) = owner_percentage_component_query.get(owner_entity) else {
            continue;
        };
        material.percent = health.percentage().value();
    }
}

fn follow_owner<T: PercentageComponent>(
    mut bar_query: Query<(&mut Transform, &StatusBarOwner), Without<StatusBarDefinition<T>>>,
    owner_query: Query<(&Transform, &StatusBarDefinition<T>)>,
) {
    for (mut transform, &StatusBarOwner(owner_entity)) in bar_query.iter_mut() {
        let Ok((owner_transform, owner_bar_definition)) = owner_query.get(owner_entity) else {
            continue;
        };

        transform.translation = owner_transform.translation + owner_bar_definition.offset;
    }
}

fn despawn<T: PercentageComponent>(
    mut commands: Commands,
    bar_query: Query<(Entity, &StatusBarOwner), Without<StatusBarDefinition<T>>>,
    owner_query: Query<&StatusBarDefinition<T>>,
) {
    for (bar_entity, &StatusBarOwner(owner_entity)) in bar_query.iter() {
        if owner_query.get(owner_entity).is_err() {
            commands.entity(bar_entity).despawn();
        };
    }
}
