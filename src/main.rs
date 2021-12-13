use crate::client::Client;
use crate::message::Message;
use std::collections::VecDeque;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::thread;

mod client;
mod core;
mod message;

fn main() {
    let bitmap = core::BitMap::new();

    let address = "0.0.0.0:3666";
    // 保持连接状态
    let listener = TcpListener::bind(address).unwrap();

    println!("process.id {:#?} listener {}", std::process::id(), address);
    for connect in listener.incoming() {
        if let Ok(mut stream) = connect {
            // 转成reader，这个可以一次读取一行
            let mut reader = BufReader::new(stream.try_clone().unwrap());

            let mut bitmap = bitmap.clone();
            thread::spawn(move || {
                let socket_id = bitmap.make_socket_id();
                let client = Client::new(socket_id);

                // 保存连接信息
                bitmap.push_client(socket_id, client.clone());

                println!(
                    "new connect {:?} socket_id {} ",
                    stream.peer_addr().unwrap(),
                    socket_id
                );
                stream.write("hello bitmap server\n".as_bytes()).unwrap();

                loop {
                    // 转成一个reader
                    let mut buffer = String::new();

                    match reader.read_line(&mut buffer) {
                        Ok(_) => {
                            // EOF
                            if buffer == "" {
                                break;
                            }

                            // 就是一个回车
                            if buffer == "\n" {
                                continue;
                            }

                            // 删除\n
                            let cmd = buffer.trim().to_string();

                            match Message::parse(&cmd) {
                                Message::SetBit { key, offset, value } => {
                                    bitmap.set(key, offset, value);
                                    stream.write(b"OK\n").unwrap();
                                    println!("{:#?}", bitmap);
                                }
                                Message::GetBit { key, offset } => {
                                    let value = bitmap.get(key.clone(), offset.clone());
                                    let resp = value.to_string();
                                    stream.write(format!("{}\n", resp).as_bytes()).unwrap();

                                    println!("getbit {:#?} {} value={:#?}\n", key, offset, resp);
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
                        }
                        Err(e) => {
                            println!("Connection reset by peer {:#?}", e);
                            break;
                        }
                    }
                }

                // 删除这个连接信息
                let total = bitmap.remove_client(socket_id);
                println!("当前连接ID {} 已退出 还有{}个连接", socket_id, total);
            });
        }
    }
}
