use std::time::Duration;

use bevy_action::{Action, ActionEvent};
use bevy_asset::{Assets, Handle};
use bevy_ecs::{
    prelude::{Entity, EventReader, EventWriter},
    system::{Query, Res},
};
use bevy_reflect::TypeUuid;
use bevy_sprite::TextureAtlasSprite;
use bevy_time::Time;
use rand::seq::SliceRandom;

use super::*;

pub(crate) fn queue_animations<T: Action + TypeUuid>(
    maps: Res<Assets<AnimationMap<T>>>,
    mut actions: EventReader<ActionEvent<T>>,
    mut query: Query<(&Handle<AnimationMap<T>>, &mut AnimationQueue<T>)>,
) {
    for action in actions.iter() {
        let Ok((map_handle, mut queue)) = query.get_mut(action.entity) else {
            continue;
        };

        let map = maps.get(map_handle).unwrap();

        let Some(group) = map.get(&action.action) else {
            continue;
        };

        let mut thread_rng = rand::thread_rng();

        match group.ordering.clone().unwrap_or_default() {
            AnimationClipOrder::Sequential => {
                for animation in &group.clips {
                    queue.push_back(animation.clone());
                }
            }
            AnimationClipOrder::Random => {
                let mut clips = group.clips.clone();

                clips.shuffle(&mut thread_rng);

                for animation in &group.clips {
                    queue.push_back(animation.clone());
                }
            }
            AnimationClipOrder::RandomSelect => {
                queue.push_back(group.clips.choose(&mut thread_rng).unwrap().clone());
            }
        }
    }
}

pub(crate) fn process_animations<T: Action + TypeUuid>(
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &ClipMap,
        &mut AnimationState<T>,
        &mut AnimationQueue<T>,
        &mut TextureAtlasSprite,
    )>,
    mut writer: EventWriter<ActionEvent<T>>,
) {
    query.for_each_mut(|(entity, clip_map, mut state, mut queue, mut sprite)| {
        state.timer.tick(time.delta());

        if state.timer.just_finished() {
            sprite.index = if sprite.index == state.clip.1 {
                if state.looping {
                    state.clip.0
                } else {
                    state.clip.1
                }
            } else if state.clip.0 < state.clip.1 {
                sprite.index + 1
            } else {
                sprite.index - 1
            }
        }

        if match state.looping {
            false => sprite.index == state.clip.1,
            true => queue.len() > 0,
        } {
            for trigger in state.triggers.drain(..) {
                writer.send(ActionEvent::<T> {
                    action: trigger,
                    entity,
                });
            }

            // TODO: Improve this
            if queue.len() == 0 {
                return;
            }

            let next_animation = queue.pop_front().unwrap();

            state.clip = clip_map.get(&next_animation.id).unwrap().clone();

            state.looping = next_animation.looping.unwrap_or_default();

            state.timer.reset();
            state
                .timer
                .set_duration(Duration::from_secs_f32(next_animation.frame_speed));

            sprite.index = state.clip.0;

            state.triggers = next_animation.triggers;
        }
    });
}
