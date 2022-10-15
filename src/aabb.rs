use bevy::prelude::*;
use bevy::sprite::Anchor;

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct AABB {
    bottom_left: Vec2,
    upper_right: Vec2,
}

impl AABB {
    pub fn inside(&self, point: Vec2) -> bool {
        self.bottom_left.x <= point.x
            && point.x <= self.upper_right.x
            && self.bottom_left.y <= point.y
            && point.y <= self.upper_right.y
    }
}

pub fn aabb_update(
    mut query: Query<(&mut AABB, &GlobalTransform, &Handle<Image>, &Sprite)>,
    images: Res<Assets<Image>>,
) {
    for (mut aabb, transform, image_handle, sprite) in &mut query {
        let size = match sprite.custom_size {
            None => match images.get(image_handle) {
                None => continue,
                Some(image) => image.size(),
            },
            Some(size) => size,
        };

        let position = transform.translation().truncate();
        match sprite.anchor {
            Anchor::Center => {
                let radius_x = size.x / 2.0;
                let radius_y = size.y / 2.0;
                aabb.bottom_left = Vec2::new(position.x - radius_x, position.y - radius_y);
                aabb.upper_right = Vec2::new(position.x + radius_x, position.y + radius_y);
            }
            Anchor::BottomLeft => {
                aabb.bottom_left = position;
                aabb.upper_right = position + size;
            }
            _ => error!("entity with unsupported anchor"),
        }
    }
}
