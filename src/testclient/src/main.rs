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

use serde::{Serialize, Deserialize};
use tungstenite::{connect, Message};
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
enum Request {
    SpeakText(String),
    CancelSpeech,
    BrailleMessage(String),
    }

fn main() {
    if let Ok((mut socket, _))=connect(get_host().to_string()) {
        let stdin=std::io::stdin();

        loop {
            let mut input=String::new();
            stdin.read_line(&mut input).unwrap();
            input=input.trim().to_string();

            if input.starts_with("speakText") && input.len()>10 {
                let request=Request::SpeakText((input[10..]).to_string());
                socket.send(Message::Binary(rmp_serde::to_vec(&request).unwrap().into())).unwrap();
                }
            else if input=="cancel" {
                let request=Request::CancelSpeech;
                socket.send(Message::Binary(rmp_serde::to_vec(&request).unwrap().into())).unwrap();
                }
            else if input=="quit" {
                break;
                }
            }

        }
    }

fn get_host() -> Url {
    if let Ok(host)=std::env::var("NVDA2SPEECHD_HOST") {
        if let Ok(url)=Url::parse(&host) {
            return url;
            }
        }

    Url::parse("ws://localhost:3457").unwrap()
    }

