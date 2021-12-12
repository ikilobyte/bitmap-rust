use chrono::Local;

#[derive(Debug, Clone)]
pub enum ClientStatus {
    Online,
    Offline,
}

#[derive(Debug, Clone)]
pub struct Client {
    pub id: usize,
    // 客户端唯一ID
    pub created_at: String,
    // 连接时间
    pub status: ClientStatus, //连接状态

    pub closed_at: String,
}

impl Client {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            created_at: Local::now().to_rfc3339(), // 创建时间
            status: ClientStatus::Online,          // 客户端状态
            closed_at: String::new(),              // 断开连接时间
        }
    }

    // 设置当前状态
    pub fn set_status(&mut self, status: ClientStatus) {
        self.status = status;
    }
}
