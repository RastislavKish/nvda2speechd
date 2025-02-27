/*
* Copyright (C) 2022 Rastislav Kish
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, version 3.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

use std::{net::TcpListener, thread::spawn};

use serde::{Serialize, Deserialize};

use tungstenite::{accept, Message};

use url::Url;

#[derive(Serialize, Deserialize, Debug)]
enum Request {
    SpeakText(String),
    CancelSpeech,
    BrailleMessage(String),
    }

fn main() {
    let server=TcpListener::bind(&get_host()).unwrap();

    for stream in server.incoming() {
        spawn(move || {
            let mut websocket=accept(stream.unwrap()).unwrap();

            let spd_connection=speech_dispatcher::Connection::open("nvda2speechd", "", "", speech_dispatcher::Mode::Threaded).unwrap();

            println!("New connection established");

            while let Ok(msg)=websocket.read() {
                if let Message::Binary(data) = msg {
                    if let Ok(request)=rmp_serde::from_slice::<Request>(&data[..]) {
                        match request {
                            Request::SpeakText(text) => {
                                spd_connection.say(speech_dispatcher::Priority::Text, &text);
                                },
                            Request::CancelSpeech => {
                                spd_connection.cancel().unwrap();
                                },
                            Request::BrailleMessage(_) => {

                                },
                            }
                        }
                    }
                }
            println!("A connection closed");
            });
        }
    }

fn get_host() -> String {
    if let Ok(host)=std::env::var("NVDA2SPEECHD_HOST") {
        if let Ok(url)=Url::parse(&host) {
            if let Some(port)=url.port() {
                return format!("127.0.0.1:{}", port);
                }
            }
        }

    "127.0.0.1:3457".to_string()
    }

