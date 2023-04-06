use std::marker::PhantomData;

use bevy_action::Action;
use bevy_asset::{AssetLoader, BoxedFuture, Error, LoadContext, LoadedAsset};
use bevy_reflect::TypeUuid;
use serde::Deserialize;

use super::{AnimationMap, ClipMap};

#[derive(Default)]
pub struct ClipMapLoader;

impl AssetLoader for ClipMapLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), Error>> {
        Box::pin(async move {
            let clip_map: ClipMap = ron::de::from_bytes(bytes)?;

            load_context.set_default_asset(LoadedAsset::new(clip_map));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["clip.ron"]
    }
}

#[derive(Default)]
pub struct AnimationMapLoader<T: Action + for<'a> Deserialize<'a> + TypeUuid>(PhantomData<T>);

impl<T: Action + for<'a> Deserialize<'a> + TypeUuid> AssetLoader for AnimationMapLoader<T> {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), Error>> {
        Box::pin(async move {
            let animation_map: AnimationMap<T> = ron::de::from_bytes(bytes)?;

            load_context.set_default_asset(LoadedAsset::new(animation_map));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["anim.ron"]
    }
}
