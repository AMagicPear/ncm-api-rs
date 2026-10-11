use super::Query;
use crate::error::Result;
/// 清空全部消息提醒的未读计数
/// 官方 web 客户端「清空全部新消息提醒」调用的接口（无参数），
/// 四类计数（私信 / 评论 / @我 / 通知）一并清零。
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 清空全部消息的未读计数
    /// 对应 /msg/count/clear
    pub async fn msg_count_clear(&self) -> Result<ApiResponse> {
        let data = json!({});
        self.request(
            "/api/msg/count/clear",
            data,
            Query::new().to_option(CryptoType::Weapi),
        )
        .await
    }
}
