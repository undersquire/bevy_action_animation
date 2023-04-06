use std::time::Duration;

use bevy_action::{Action, ActionEvent};
use bevy_asset::{Assets, Handle};
use bevy_ecs::{
    prelude::{Entity, EventReader, EventWriter},
    system::{Query, Res},
};
use bevy_sprite::TextureAtlasSprite;
use bevy_time::Time;

use super::{
    group::{AnimationAttribute, AnimationGroupOrderMode, AnimationMode},
    AnimationClip, AnimationMap, AnimationQueue, AnimationTimer, AnimationTriggers, ClipMap,
};

pub(crate) fn queue_animations<T: Action>(
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
            AnimationGroupOrderMode::Random => {}
            AnimationGroupOrderMode::RandomSelect => {}
        }
    }
}

pub(crate) fn process_animations<T: Action>(
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &Handle<ClipMap>,
        &mut AnimationClip,
        &mut AnimationTimer,
        &mut AnimationQueue<T>,
        &mut TextureAtlasSprite,
        &mut AnimationTriggers<T>,
    )>,
    clip_maps: Res<Assets<ClipMap>>,
    mut writer: EventWriter<ActionEvent<T>>,
) {
    query.for_each_mut(
        |(entity, clip_map, mut clip, mut timer, mut queue, mut sprite, mut triggers)| {
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

                let Some(clip_map) = clip_maps.get(clip_map) else {
                    return;
                };

                let next_animation = queue.pop_front().unwrap();

                clip.0 = clip_map.clips.get(next_animation.clip).unwrap().clone();

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
