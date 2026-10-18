//! 推荐页「推荐歌单」区块（PC 3.1.41）。
//! 对应 `/pc/personal/page/playlist/rcmd`。

use super::Query;
use crate::error::Result;
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 推荐页的推荐歌单，一次 12 条；`blockData.creatives` 里是歌单卡片。
    pub async fn personal_page_playlist_rcmd(&self, query: &Query) -> Result<ApiResponse> {
        self.request(
            "/api/pc/personal/page/playlist/rcmd",
            json!({}),
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
