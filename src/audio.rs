use crate::actions::Actions;
use crate::bees::flower_collision;
use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_plugins(AudioPlugin)
            .add_audio_channel::<Background>()
            .add_systems(OnEnter(GameState::Playing), play_background_music)
            .add_systems(
                Update,
                play_flower_sound
                    .after(flower_collision)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[derive(Resource)]
struct Background;

#[derive(Resource)]
struct FlowerAudio(Handle<AudioInstance>);

fn play_background_music(
    background_channel: Res<AudioChannel<Background>>,
    audio_assets: Res<AudioAssets>,
) {
    background_channel
        .play(audio_assets.background_music.clone())
        .looped()
        .with_volume(0.3);
}

fn play_flower_sound(
    mut actions: ResMut<Actions>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
) {
    if actions.flower_gotten {
        actions.flower_gotten = false;
        audio.play(audio_assets.flower.clone()).with_volume(0.5);
    }
}
