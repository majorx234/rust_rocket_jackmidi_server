extern crate jack;
extern crate wmidi;
use crate::midi_note::{MidiNote, NoteType};
use std::{thread, time::Duration};

pub fn start_jack_thread(rx_midi_note: flume::Receiver<MidiNote>) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        let (client, _status) =
            jack::Client::new("rocket_midi_server", jack::ClientOptions::NO_START_SERVER).unwrap();
        let _sample_rate = client.sample_rate();
        let mut midi_out = client
            .register_port("rocket_midi_out", jack::MidiOut::default())
            .unwrap();
        // get frame size
        let _frame_size = client.buffer_size();
        let process_callback =
            move |client: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
                let mut midi_port = midi_out.writer(ps);
                match rx_midi_note.try_recv() {
                    Ok(midi_note) => {
                        match midi_note.note_type {
                            NoteType::NoteOn => midi_port
                                .write(&jack::RawMidi {
                                    time: 0,
                                    bytes: &[
                                        0b10010000,         /* Note On, channel 1 */
                                        midi_note.pitch,    /* key number */
                                        midi_note.velocity, /* velocity */
                                    ],
                                })
                                .unwrap(),
                            NoteType::NoteOff => midi_port
                                .write(&jack::RawMidi {
                                    time: 0,
                                    bytes: &[
                                        0b10000000,         /* Note Off, channel 1 */
                                        midi_note.pitch,    /* key number */
                                        midi_note.velocity, /* velocity */
                                    ],
                                })
                                .unwrap(),
                        }
                    }
                    Err(_) => {}
                }
                jack::Control::Continue
            };

        let process = jack::ClosureProcessHandler::new(process_callback);
        let active_client = client.activate_async((), process).unwrap();
        let mut run: bool = true;
        while run {
            thread::sleep(Duration::from_millis(100));
        }
        println!("exit audio thread\n");
        active_client.deactivate().unwrap();
    })
}
