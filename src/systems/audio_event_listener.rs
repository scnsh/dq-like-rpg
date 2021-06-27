use crate::components::*;

use bevy::prelude::*;
use bevy_kira_audio::Audio;

pub fn audio_event_listener(
    audio: Res<Audio>,
    mut audio_state: ResMut<AudioState>,
    mut events_reader: EventReader<AudioEvent>,
) {
    if !audio_state.audio_loaded {
        return;
    }
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
            // AudioEvent::Pause(kind) => {
            //     let (channel, channel_audio_state) = audio_state.get_channel(kind).unwrap();
            //     // 既に停止していればSkip
            //     if channel_audio_state.stopped {
            //         continue;
            //     }
            //     if channel_audio_state.paused {
            //         // pauseしていればplay
            //         audio.resume_channel(channel);
            //     } else {
            //         // playしていればpause
            //         audio.pause_channel(channel);
            //     }
            //     channel_audio_state.paused = !channel_audio_state.paused;
            // }
            AudioEvent::Stop(kind) => {
                let (channel, channel_audio_state) = audio_state.get_channel(kind).unwrap();
                // 既に停止していればSkip
                if channel_audio_state.stopped {
                    continue;
                }
                audio.stop_channel(channel);
                *channel_audio_state = ChannelAudioState::default();
            }
        }
    }
}
