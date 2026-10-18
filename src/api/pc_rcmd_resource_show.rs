//! 推荐页首屏聚合（PC 3.1.41）。
//! 对应 `/pc/page/rcmd/resource/show`。

use super::Query;
use crate::error::Result;
use crate::request::{ApiClient, ApiResponse, CryptoType};
use chrono::Local;
use serde_json::{Value, json};

impl ApiClient {
    /// 推荐页首屏聚合。
    ///
    /// `blockRequestParam` 决定服务端实际计算哪些区块；缺省只请求
    /// 「每日推荐 + 风格日推」混排块。需要别的区块时用
    /// `Query::param("block_request_param", &json_string)` 传入完整的 JSON 对象。
    pub async fn pc_rcmd_resource_show(&self, query: &Query) -> Result<ApiResponse> {
        let block_request_param = query
            .get_or("block_request_param", "")
            .parse::<Value>()
            .ok()
            .filter(|value| value.is_object())
            .unwrap_or_else(|| {
                json!({
                    "PC_HOMEPAGE_DAILY_MIX": {
                        "clientTime": Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
                    }
                })
            });
        let data = json!({
            "pageCode": query.get_or("page_code", "PC_RECOMMEND_HOME"),
            "isFirstScreen": query.get_or("is_first_screen", "true") == "true",
            "cursor": query.get_or("cursor", "0").parse::<i64>().unwrap_or(0),
            "extJson": query.get_or("ext_json", ""),
            "blockCodeOrderList": query.get_or("block_code_order_list", ""),
            "blockRequestParam": block_request_param,
        });
        self.request(
            "/api/pc/page/rcmd/resource/show",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
