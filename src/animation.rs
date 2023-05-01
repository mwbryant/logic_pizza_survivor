use crate::prelude::*;

pub struct GameAnimationPlugin;

impl Plugin for GameAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(two_frame_animation);
    }
}

pub fn two_frame_animation(
    mut animated: Query<(&mut TwoFrameAnimation, &mut Handle<Image>)>,
    time: Res<Time>,
) {
    for (mut animation, mut image) in &mut animated {
        animation.timer.tick(time.delta());
        if animation.current_frame {
            *image = animation.frame_2.clone();
        } else {
            *image = animation.frame_1.clone();
        }

        if animation.timer.just_finished() {
            if animation.current_frame {
                animation.current_frame = false;
            } else {
                animation.current_frame = true;
            }
        }
    }
}
