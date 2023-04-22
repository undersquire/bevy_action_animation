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

use super::{
    group::{AnimationAttribute, AnimationGroupOrderMode, AnimationMode},
    AnimationClip, AnimationMap, AnimationQueue, AnimationTimer, AnimationTriggers,
};

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

        let Some(group) = map.animations.get(&action.action) else {
            continue;
        };

        let mut thread_rng = rand::thread_rng();

        match group
            .ordering
            .clone()
            .unwrap_or(AnimationGroupOrderMode::Sequential)
        {
            AnimationGroupOrderMode::Sequential => {
                for animation in &group.clips {
                    queue.push_back(animation.clone());
                }
            }
            AnimationGroupOrderMode::Random => {
                let mut clips = group.clips.clone();

                clips.shuffle(&mut thread_rng);

                for animation in &group.clips {
                    queue.push_back(animation.clone());
                }
            }
            AnimationGroupOrderMode::RandomSelect => {
                queue.push_back(group.clips.choose(&mut thread_rng).unwrap().clone());
            }
        }
    }
}

pub(crate) fn process_animations<T: Action + TypeUuid>(
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &Handle<AnimationMap<T>>,
        &mut AnimationClip,
        &mut AnimationTimer,
        &mut AnimationQueue<T>,
        &mut TextureAtlasSprite,
        &mut AnimationTriggers<T>,
    )>,
    animation_maps: Res<Assets<AnimationMap<T>>>,
    mut writer: EventWriter<ActionEvent<T>>,
) {
    query.for_each_mut(
        |(entity, map, mut clip, mut timer, mut queue, mut sprite, mut triggers)| {
            timer.timer.tick(time.delta());

            if timer.timer.just_finished() {
                sprite.index = if sprite.index == clip.last {
                    if timer.mode == AnimationMode::Repeating {
                        clip.first
                    } else {
                        clip.last
                    }
                } else {
                    if clip.first < clip.last {
                        sprite.index + 1
                    } else {
                        sprite.index - 1
                    }
                }
            }

            if match timer.mode {
                AnimationMode::Once => sprite.index == clip.last,
                AnimationMode::Repeating => queue.len() > 0,
            } {
                for trigger in triggers.drain(..) {
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

                let animation_map = animation_maps.get(map).unwrap();

                clip.0 = animation_map
                    .clips
                    .get(next_animation.clip)
                    .unwrap()
                    .clone();

                timer.mode = next_animation.mode;

                timer.timer.reset();
                timer
                    .timer
                    .set_duration(Duration::from_secs_f32(next_animation.rate));

                sprite.index = clip.first;
                sprite.flip_x = false;
                sprite.flip_y = false;

                for attribute in next_animation.attributes {
                    match attribute {
                        AnimationAttribute::FlipX => sprite.flip_x = true,
                        AnimationAttribute::FlipY => sprite.flip_y = true,
                        AnimationAttribute::Trigger(action) => {
                            triggers.push(action);
                        }
                    }
                }
            }
        },
    );
}
