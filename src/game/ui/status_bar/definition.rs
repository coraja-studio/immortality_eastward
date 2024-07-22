use crate::game::ui::status_bar::plugin::PercentageComponent;
use bevy::prelude::*;
use std::marker::PhantomData;

pub struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Size {
            width: f32::max(width, 0.0),
            height: f32::max(height, 0.0),
        }
    }

    pub fn width(&self) -> f32 {
        self.width
    }
    pub fn height(&self) -> f32 {
        self.height
    }
}

#[derive(Component)]
pub struct StatusBarDefinition<T: PercentageComponent> {
    pub size: Size,
    pub offset: Vec3,
    pub foreground_color: Color,
    pub background_color: Color,
    pub phantom_data: PhantomData<T>,
}

impl<T: PercentageComponent> Default for StatusBarDefinition<T> {
    fn default() -> Self {
        Self {
            size: Size::new(40.0, 4.0),
            offset: Vec3::new(0.0, 24.0, 10.0),
            foreground_color: Color::srgb(1.0, 0.0, 0.0),
            background_color: Color::srgb(0.0, 0.0, 0.0),
            phantom_data: PhantomData,
        }
    }
}
