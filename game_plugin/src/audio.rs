use crate::loading::AudioAssets;
use crate::AppState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin, AudioSource};
use std::collections::HashMap;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(AudioPlugin)
            .init_resource::<AudioState>()
            .add_event::<AudioEvent>()
            .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_audio.system()))
            .add_system_set(
                SystemSet::on_update(AppState::InGameMap).with_system(control_audio.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGameExplore).with_system(control_audio.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGameBattle).with_system(control_audio.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGameEvent).with_system(control_audio.system()),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::InGameMap).with_system(stop_audio.system()),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::InGameExplore).with_system(stop_audio.system()),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::InGameBattle).with_system(stop_audio.system()),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::InGameEvent).with_system(stop_audio.system()),
            );
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum AudioKind {
    BGMExplore,
    BGMBattle,
    BGMBattleLast,
    BGMWin,
    BGMLose,
    SEAttack,
    SEHeal,
    SETown,
}

#[derive(Debug)]
pub enum AudioEvent {
    Play(AudioKind),
    // Pause(AudioKind),
    Stop(AudioKind),
}

pub struct AudioState {
    pub audio_loaded: bool,
    pub sound_handles: HashMap<AudioKind, Handle<AudioSource>>,
    pub channels: HashMap<String, (AudioChannel, ChannelAudioState)>,
}
impl Default for AudioState {
    fn default() -> Self {
        AudioState {
            audio_loaded: false,
            sound_handles: HashMap::new(),
            channels: HashMap::new(),
        }
    }
}
impl AudioState {
    pub fn get_channel(
        &mut self,
        kind: &AudioKind,
    ) -> Option<&mut (AudioChannel, ChannelAudioState)> {
        match kind {
            AudioKind::SEAttack | AudioKind::SEHeal | AudioKind::SETown => {
                self.channels.get_mut("se")
            }
            _ => self.channels.get_mut("bgm"),
        }
    }
}

pub struct ChannelAudioState {
    pub stopped: bool,
    pub paused: bool,
    pub loop_started: bool,
    pub volume: f32,
}

impl Default for ChannelAudioState {
    fn default() -> Self {
        ChannelAudioState {
            volume: 1.0,
            stopped: true,
            loop_started: false,
            paused: false,
        }
    }
}

fn setup_audio(audio_assets: Res<AudioAssets>, mut audio_state: ResMut<AudioState>) {
    audio_state.channels.insert(
        String::from("bgm"),
        (
            AudioChannel::new("bgm".to_owned()),
            ChannelAudioState::default(),
        ),
    );
    audio_state.channels.insert(
        String::from("se"),
        (
            AudioChannel::new("se".to_owned()),
            ChannelAudioState::default(),
        ),
    );

    // bgm(ループ再生)
    audio_state.sound_handles.insert(
        AudioKind::BGMExplore,
        audio_assets.get_handle_for_audio(AudioKind::BGMExplore),
    );
    audio_state.sound_handles.insert(
        AudioKind::BGMBattle,
        audio_assets.get_handle_for_audio(AudioKind::BGMBattle),
    );
    audio_state.sound_handles.insert(
        AudioKind::BGMLose,
        audio_assets.get_handle_for_audio(AudioKind::BGMLose),
    );
    audio_state.sound_handles.insert(
        AudioKind::BGMWin,
        audio_assets.get_handle_for_audio(AudioKind::BGMWin),
    );
    audio_state.sound_handles.insert(
        AudioKind::BGMBattleLast,
        audio_assets.get_handle_for_audio(AudioKind::BGMBattleLast),
    );
    // se(one-shot)
    audio_state.sound_handles.insert(
        AudioKind::SEAttack,
        audio_assets.get_handle_for_audio(AudioKind::SEAttack),
    );
    audio_state.sound_handles.insert(
        AudioKind::SEHeal,
        audio_assets.get_handle_for_audio(AudioKind::SEHeal),
    );
    audio_state.sound_handles.insert(
        AudioKind::SETown,
        audio_assets.get_handle_for_audio(AudioKind::SETown),
    );
}

fn stop_audio(audio: Res<Audio>, mut audio_state: ResMut<AudioState>) {
    for (_name, (channel, state)) in audio_state.channels.iter_mut() {
        audio.stop_channel(channel);
        state.paused = false;
        state.stopped = false;
        state.loop_started = false;
    }
}

fn control_audio(
    audio: Res<Audio>,
    mut audio_state: ResMut<AudioState>,
    mut events_reader: EventReader<AudioEvent>,
) {
    for event in events_reader.iter() {
        match event {
            AudioEvent::Play(kind) => {
                let audio_source = audio_state.sound_handles[kind].clone();
                let (channel, channel_audio_state) = audio_state.get_channel(kind).unwrap();
                match kind {
                    AudioKind::SEAttack | AudioKind::SEHeal | AudioKind::SETown => {
                        channel_audio_state.paused = false;
                        channel_audio_state.stopped = false;
                        audio.play_in_channel(audio_source, channel);
                    }
                    _ => {
                        if channel_audio_state.loop_started {
                            continue;
                        }
                        channel_audio_state.loop_started = true;
                        channel_audio_state.stopped = false;
                        audio.play_looped_in_channel(audio_source, channel);
                    }
                }
            }
            AudioEvent::Stop(kind) => {
                let (channel, channel_audio_state) = audio_state.get_channel(kind).unwrap();
                if channel_audio_state.stopped {
                    continue;
                }
                audio.stop_channel(channel);
                *channel_audio_state = ChannelAudioState::default();
            }
        }
    }
}
