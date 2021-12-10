use crate::message::Message;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

mod core;
mod message;

fn main() {
    let mut bitmap = core::BitMap::new();

    bitmap.set("xxx".to_string(), 3, 1);

    sleep(Duration::from_secs(3600));

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
                        Ok(size) => {
                            // 读取完毕一个完整的包
                            if buffer[0] == 10 {
                                let cmd = content.clone();

                                match Message::parse(cmd.clone()) {
                                    Message::SetBit { key, offset, value } => {
                                        println!("{:#?}", "开始setbit");
                                        let size = bitmap.set(key, offset, value);
                                        println!("{:#?}", size);
                                        stream.write("Ok\n".as_bytes()).unwrap();
                                    }
                                    Message::GetBit { key, offset } => {
                                        let value = bitmap.get(key.clone(), offset.clone());
                                        let resp = value.to_string() + "\n";
                                        stream.write(resp.as_bytes()).unwrap();

                                        println!("getbit {:#?} {} value={:#?}", key, offset, resp);
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

                                break;
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
