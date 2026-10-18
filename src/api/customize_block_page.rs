//! 定制区块（私人雷达等）。
//! 对应 `/pc/customize/block/page`。

use super::Query;
use crate::error::Result;
use crate::request::{ApiClient, ApiResponse, CryptoType};
use serde_json::json;

impl ApiClient {
    /// 定制区块：`CUSTOMIZE_PLAYLIST_MGC` 是私人雷达，`CUSTOMIZE_PLAYLIST_OFFICIAL`
    /// 是官方雷达。`newStyle` 默认开，和 3.1.41 客户端一致。
    pub async fn customize_block_page(&self, query: &Query) -> Result<ApiResponse> {
        let data = json!({
            "newStyle": query.get_or("new_style", "true") == "true"
        });
        self.request(
            "/api/pc/customize/block/page",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
