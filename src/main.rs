use crate::client::Client;
use crate::message::Message;
use std::io::{Read, Write};
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
                let mut buffer = [0; 1];
                let mut content = String::new();

                loop {
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("{:#?}", buffer);
                            if buffer[0] == 0 {
                                println!("client is close!");
                                break;
                                // 用户关闭了连接
                            }
                            // 读取完毕一个完整的包
                            if buffer[0] == 10 {
                                let cmd = content.clone();
                                // 重置一下
                                buffer = [0; 1];

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
                                        stream.write(format!("{}\n", resp).as_bytes()).unwrap();

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
