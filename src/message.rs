use crate::message::Message::{GetBit, SetBit, UnSupport};
use std::fmt::Debug;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Message {
    SetBit {
        key: String,
        offset: usize,
        value: u8,
    },
    GetBit {
        key: String,
        offset: usize,
    },
    UnSupport,
    Error(String),
}

impl Message {
    // 解析出是什么命令
    pub fn parse(buffer: String) -> Self {
        let instruction: Vec<&str> = buffer.split_whitespace().collect();

        // 长度小于2的全部不处理
        if instruction.len() < 2 {
            return Self::Error("命令长度少于2".to_string());
        }

        let head = instruction[0];

        if head == "setbit" {
            if instruction.len() != 4 {
                return Self::Error("参数错误，格式为：setbit key offset value".to_string());
            }

            return Self::parse_setbit(instruction);
        }

        // getbit
        if head == "getbit" {
            if instruction.len() != 3 {
                return Self::Error("缺少参数，格式为：getbit key offset".to_string());
            }

            return Self::parse_getbit(instruction);
        }

        // unSupport
        return UnSupport;
    }

    // 解析setbit
    fn parse_setbit(cmd_list: Vec<&str>) -> Self {
        let key = cmd_list[1].to_string();
        let offset = cmd_list[2].parse::<usize>();
        if let Err(_) = offset {
            return UnSupport;
        }

        let value = cmd_list[3].parse::<u8>();
        if let Err(_) = value {
            return UnSupport;
        }

        let value = value.unwrap();
        if value != 0 && value != 1 {
            return Self::Error("value只能是0或1".to_string());
        }

        SetBit {
            key,
            offset: offset.unwrap(),
            value,
        }
    }

    // 解析getbit
    fn parse_getbit(cmd_list: Vec<&str>) -> Self {
        let key = cmd_list[1].to_string();

        let offset = cmd_list[2].parse::<usize>();
        if let Err(_) = offset {
            return UnSupport;
        }

        GetBit {
            key,
            offset: offset.unwrap(),
        }
    }
}
