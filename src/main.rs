use crate::message::Message;
use std::io::{Read, Write};
use std::net::TcpListener;
// use std::sync::{Arc, Mutex};
use std::thread;
// use std::thread::sleep;
// use std::time::Duration;

mod core;
mod message;

fn main() {
    let bitmap = core::BitMap::new();

    // 保持连接状态
    let listener = TcpListener::bind("0.0.0.0:3666").unwrap();

    for connect in listener.incoming() {
        if let Ok(mut stream) = connect {
            let mut bitmap = bitmap.clone();
            thread::spawn(move || {
                let mut buffer = [0; 1];
                let mut content = String::new();

                loop {
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            // 读取完毕一个完整的包
                            if buffer[0] == 10 {
                                let cmd = content.clone();

                                // 清空
                                content = "".to_string();

                                match Message::parse(cmd.clone()) {
                                    Message::SetBit { key, offset, value } => {
                                        bitmap.set(key, offset, value);
                                        stream.write("OK\n".as_bytes()).unwrap();
                                        println!("{:#?}", bitmap);
                                    }
                                    Message::GetBit { key, offset } => {
                                        let value = bitmap.get(key.clone(), offset.clone());
                                        let resp = value.to_string();
                                        stream.write(resp.as_bytes()).unwrap();

                                        println!(
                                            "getbit {:#?} {} value={:#?}\n",
                                            key, offset, resp
                                        );
                                    }
                                    Message::UnSupport => {
                                        stream
                                            .write("目前只支持setbit/getbit\n".as_bytes())
                                            .unwrap();
                                    }
                                    Message::Error(mut error) => {
                                        error.push_str("\n");
                                        stream.write(error.as_bytes()).unwrap();
                                    }
                                }

                                // 就不退出了!
                                // break;
                            } else {
                                content.push_str(&String::from_utf8_lossy(&buffer));
                            }
                        }
                        Err(e) => {
                            println!("{:#?}", e);
                        }
                    }
                }
            });
        }
    }
}
