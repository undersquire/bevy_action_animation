# bevy_action_animation

Action-based animation system for Bevy.

## Introduction

This plugin provides users of the [Bevy game engine](https://www.bevyengine.org/) with an action/trigger-based animation system.
This kind of animation system makes for a more scalable framework for adding animations into a game,
by providing the necessary tools to decouple animation logic from game logic.

This plugin is based on [bevy_action](https://www.github.com/undersquire/bevy_action).

**WARNING:** This plugin is still in beta state. Please report bugs on the issues page!

## How It Works

### Actions

Based on [bevy_action](https://www.github.com/undersquire/bevy_action), animations are configured tightly
with specific action types defined by the user, and are triggered when the `AnimationPlugin`'s internal systems read an action
that corresponds to one of its defined animations.

### Animation Queue

`bevy_action_animation` is a fairly simple plugin. It provides the action-based animation system
by using an *animation queue*.

This queue *queues* up animations as they are triggered, and plays them in order as they pop off the queue (LIFO).

There are two types of animation-play modes: `Once`, and `Repeating`.

The `Once` mode plays the animation clip entirely, once, and cannot be interrupted.

The `Repeating` mode plays in a cycle, infinitely, until another animation is added to the animation queue.
This means that if a `Repeating` animation was added, and then another was added the next frame, the original
`Repeating` animation would most likely only play one frame before being cancelled and overwritten by the next animation.

### Animation Attributes

Another feature of `bevy_action_animation` is the ability to give certain animations certain attributes. These are hard-defined
by the plugin itself, and include the following:

- FlipX (flips sprite horizontally)
- FlipY (flips sprite vertically)
- Trigger(Action) (triggers an action upon completion) [1]

Side-notes:

1. `Once` animations will trigger a `Trigger` attribute's action upon full completion. `Repeating` animations will trigger the attribute's action upon cancellation.

Duplicating these attributes is possible, but may have no affect for some:

- FlipX - No affect
- FlipY - No affect
- Trigger - Adds another trigger

### Animation Definition File

Animations are defined in [RON](https://www.github.com/ron-rs/ron) files.

**NOTE:** Custom formats will be supported in the future.

An example of the format structure of the RON files is available in the [assets](https://www.github.com/undersquire/bevy_action_animation/tree/main/assets) directory.

## Examples

See the [examples](https://github.com/undersquire/bevy_action_animation/tree/main/examples) directory.

## Future Work

- [ ] Better documentation
- [ ] Support custom formats
- [ ] Decouple animation files from the animation system
- [ ] Benchmark?
