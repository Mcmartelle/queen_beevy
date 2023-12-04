use crate::actions::{set_movement_actions, Actions};
use crate::bees::death_collisions;
use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_systems(OnEnter(GameState::Playing), start_audio)
            .add_systems(
                Update,
                // (
                    // control_flying_sound
                //     .after(set_movement_actions)
                //     .run_if(in_state(GameState::Playing)),
                play_death_sound
                    .after(death_collisions)
                    .run_if(in_state(GameState::Playing)
                // ),
            ));
    }
}

#[derive(Resource)]
struct FlyingAudio(Handle<AudioInstance>);

#[derive(Resource)]
struct DeathAudio(Handle<AudioInstance>);

fn start_audio(mut commands: Commands, audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.pause();
    let handle = audio
        .play(audio_assets.flying.clone())
        .looped()
        .with_volume(0.3)
        .handle();
    commands.insert_resource(FlyingAudio(handle));
}

fn play_death_sound(
    actions: Res<Actions>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    if actions.worker_bee_died {
        audio.play(audio_assets.death.clone())
            .with_volume(0.3);
    }
}

fn control_flying_sound(
    actions: Res<Actions>,
    audio: Res<FlyingAudio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Some(instance) = audio_instances.get_mut(&audio.0) {
        match instance.state() {
            PlaybackState::Paused { .. } => {
                if actions.p1_movement.is_some() {
                    instance.resume(AudioTween::default());
                }
            }
            PlaybackState::Playing { .. } => {
                if actions.p1_movement.is_none() {
                    instance.pause(AudioTween::default());
                }
            }
            _ => {}
        }
    }
}
