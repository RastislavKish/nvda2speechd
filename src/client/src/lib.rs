/*
* Copyright (C) 2022 Rastislav Kish
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU Lesser General Public License as published by
* the Free Software Foundation, version 2.1.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
* GNU Lesser General Public License for more details.
*
* You should have received a copy of the GNU Lesser General Public License
* along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

use std::net::TcpStream;
use std::sync::{LazyLock, Mutex};

use widestring::U16CString;

use serde::{Serialize, Deserialize};
use tungstenite::{connect, Message, stream::MaybeTlsStream, WebSocket};
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
enum Request {
    SpeakText(String),
    CancelSpeech,
    BrailleMessage(String),
    }

struct Core {
    socket: Option<WebSocket<MaybeTlsStream<TcpStream>>>,
    }
impl Core {

    pub fn new() -> Core {
        if let Ok((socket, _))=connect(Core::get_host()) {
            return Core { socket: Some(socket) };
            }

        Core { socket: None }
        }

    pub fn test_if_running(&mut self) -> i64 {
        if self.socket.is_none() {
            self.reconnect();
            }

        if self.socket.is_some() {
            return 0;
            }

        -1
        }
    pub fn speak_text(&mut self, text: String) -> i64 {
        if self.socket.is_none() {
            self.reconnect();
            }

        if let Some(socket)=&mut self.socket {
            let request=Request::SpeakText(text);
            socket.write_message(Message::Binary(rmp_serde::to_vec(&request).unwrap())).unwrap();
            return 0;
            }

        -1
        }
    pub fn cancel_speech(&mut self) -> i64 {
        if self.socket.is_none() {
            self.reconnect();
            }

        if let Some(socket)=&mut self.socket {
            let request=Request::CancelSpeech;
            socket.write_message(Message::Binary(rmp_serde::to_vec(&request).unwrap())).unwrap();
            return 0;
            }

        -1
        }
    pub fn braille_message(&mut self, text: String) -> i64 {
        if self.socket.is_none() {
            self.reconnect();
            }

        if let Some(socket)=&mut self.socket {
            let request=Request::BrailleMessage(text);
            socket.write_message(Message::Binary(rmp_serde::to_vec(&request).unwrap())).unwrap();
            return 0;
            }

        -1
        }

    fn reconnect(&mut self) {
        if let Ok((socket, _))=connect(Core::get_host()) {
            self.socket=Some(socket);
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
    }

static CORE: LazyLock<Mutex<Core>> = LazyLock::new(|| Mutex::new(Core::new()));

#[no_mangle]
pub extern "stdcall" fn nvdaController_testIfRunning() -> i64 {
    let mut core=CORE.lock().unwrap();

    core.test_if_running()
    }

#[no_mangle]
pub extern "stdcall" fn nvdaController_speakText(text_ptr: *const u16) -> i64 {
    let text=unsafe {
        U16CString::from_ptr_str(text_ptr).to_string().unwrap()
        };

    let mut core=CORE.lock().unwrap();

    core.speak_text(text)
    }

#[no_mangle]
pub extern "stdcall" fn nvdaController_cancelSpeech() -> i64 {
    let mut core=CORE.lock().unwrap();

    core.cancel_speech()
    }

#[no_mangle]
pub extern "stdcall" fn nvdaController_brailleMessage(text_ptr: *const u16) -> i64 {
    let text=unsafe {
        U16CString::from_ptr_str(text_ptr).to_string().unwrap()
        };

    let mut core=CORE.lock().unwrap();

    core.braille_message(text)
    }

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        }

    }
