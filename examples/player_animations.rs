/// NOTE: I did not make the art used in this example, credits to Red_Voxel.
/// https://opengameart.org/content/simple-platformer-character
///
/// This example shows using actions for a few player animations.
use bevy::{prelude::*, reflect::TypeUuid};
use bevy_action::{Action, ActionEvent, ActionPlugin};
use bevy_action_animation::{AnimationBundle, AnimationMap, AnimationPlugin};
use bevy_asset::{AssetLoader, LoadedAsset};
use serde::{Deserialize, Serialize};

/// Marker component for player.
#[derive(Component)]
struct Player;

/// Player action enum (represents all actions a player can do).
/// This is also being used here as a component to track player state (for input handling).
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Default,
    TypeUuid,
    Action,
    Component,
    Serialize,
    Deserialize,
)]
#[uuid = "255554ef-9787-45ce-a7ca-105f6c819d5a"]
enum PlayerAction {
    #[default]
    Idle,
    Run,
    Swing,
    // Used to demonstrate how animations can trigger actions.
    SwingFinished,
}

struct AnimLoader;

impl AssetLoader for AnimLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy_asset::LoadContext,
    ) -> bevy_asset::BoxedFuture<'a, Result<(), bevy_asset::Error>> {
        Box::pin(async move {
            let animation_map: AnimationMap<PlayerAction> = ron::de::from_bytes(bytes).unwrap();

            load_context.set_default_asset(LoadedAsset::new(animation_map));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["anim.ron"]
    }
}

#[bevy_main]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // Register the `ActionPlugin` for our action type.
        .add_plugin(ActionPlugin::<PlayerAction>::default())
        // Register the animation plugin for our action type.
        .add_plugin(AnimationPlugin::<PlayerAction>::default())
        .add_asset_loader(AnimLoader)
        .add_system(setup.in_schedule(CoreSchedule::Startup))
        .add_systems((input, action_sender).chain())
        .add_system(swing_finished)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut action_events: EventWriter<ActionEvent<PlayerAction>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Load our texture atlas normally
    let texture_handle = asset_server.load("player_sheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(48.0, 48.0), 8, 3, None, None);
    let texture_atlas = texture_atlases.add(texture_atlas);

    // AnimationMaps must currently be stored as a `.anim.ron` file. Custom formats will be supported in the future.
    let animation_map: Handle<AnimationMap<PlayerAction>> = asset_server.load("player.anim.ron");

    let player = commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas,
                transform: Transform {
                    scale: Vec3::splat(2.0),
                    ..default()
                },
                ..default()
            },
            // Add the `AnimationBundle` to our entity, providing our ClipMap and AnimationMap. The rest should just be given the defaults.
            AnimationBundle {
                animation_map,
                ..default()
            },
            Player,
            // Player is idling initially.
            PlayerAction::Idle,
        ))
        .id();

    action_events.send(ActionEvent {
        action: PlayerAction::Idle,
        entity: player,
    });
}

fn input(keyboard: Res<Input<KeyCode>>, mut player: Query<&mut PlayerAction, With<Player>>) {
    let mut player_action = player.get_single_mut().unwrap();

    if *player_action == PlayerAction::Idle {
        if keyboard.just_pressed(KeyCode::Space) {
            *player_action = PlayerAction::Swing;
        } else if keyboard.pressed(KeyCode::Right) {
            *player_action = PlayerAction::Run;
        }
    } else if *player_action == PlayerAction::Run {
        if !keyboard.pressed(KeyCode::Right) {
            *player_action = PlayerAction::Idle;
        }
    }
}

fn action_sender(
    mut action_events: EventWriter<ActionEvent<PlayerAction>>,
    player: Query<(Entity, &PlayerAction), (With<Player>, Changed<PlayerAction>)>,
) {
    if let Ok((player, player_action)) = player.get_single() {
        action_events.send(ActionEvent {
            action: *player_action,
            entity: player,
        });
    }
}

fn swing_finished(
    mut player_action: Query<&mut PlayerAction, With<Player>>,
    mut action_events: EventReader<ActionEvent<PlayerAction>>,
) {
    for ActionEvent { action, entity } in action_events.iter() {
        if let PlayerAction::SwingFinished = action {
            let mut player_action = player_action.get_mut(*entity).unwrap();

            *player_action = PlayerAction::Idle;
        }
    }
}
