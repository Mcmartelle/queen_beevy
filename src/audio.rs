use crate::actions::{
    // set_movement_actions,
    Actions,
};
use crate::bees::flower_collision;
use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_plugins(AudioPlugin)
            .add_systems(OnEnter(GameState::Playing), start_audio)
            .add_systems(
                Update,
                play_flower_sound
                    .after(flower_collision)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Resource)]
struct FlyingAudio(Handle<AudioInstance>);

#[derive(Resource)]
struct FlowerAudio(Handle<AudioInstance>);

fn start_audio(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    audio.pause();
    let handle = audio
        .play(audio_assets.flower.clone())
        .with_volume(0.5)
        .handle();
    audio.stop();
    commands.insert_resource(FlowerAudio(handle));
}

fn play_flower_sound(
    mut commands: Commands,
    mut actions: ResMut<Actions>,
    flower_audio: Res<FlowerAudio>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Some(instance) = audio_instances.get_mut(&flower_audio.0) {
        match instance.state() {
            PlaybackState::Paused { .. } => {
                instance.resume(AudioTween::default());
            }
            PlaybackState::Playing { .. } | PlaybackState::Stopping { .. } => {
                if actions.flower_gotten {
                    actions.flower_gotten = false;
                    instance.seek_to(0.0);
                    // instance.pause(AudioTween::default());
                }
            }
            PlaybackState::Stopped => {
                if actions.flower_gotten {
                    actions.flower_gotten = false;
                    let handle = audio
                        .play(audio_assets.flower.clone())
                        .with_volume(0.5)
                        .handle();
                    commands.insert_resource(FlowerAudio(handle));
                }
            }
            _ => {}
        }
    }
}
