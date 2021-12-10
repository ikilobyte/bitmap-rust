#[derive(Debug)]
pub enum ClientStatus {
    Online,
    Offline,
}

#[derive(Debug)]
pub struct Client {
    pub id: usize,
    // 客户端唯一ID
    pub created_at: String,
    // 连接时间
    pub status: ClientStatus, //连接状态
}
