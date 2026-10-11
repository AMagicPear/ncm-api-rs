use super::Query;
use crate::error::Result;
/// 手机号登录
/// 对应 Node.js module/login_cellphone.js
use crate::request::{ApiClient, ApiResponse, CryptoType};
use md5::{Digest, Md5};
use serde_json::{json, Value};

impl ApiClient {
    /// 手机号登录
    /// 对应 /login/cellphone
    pub async fn login_cellphone(&self, query: &Query) -> Result<ApiResponse> {
        let mut data = json!({
            "type": "1",
            "https": "true",
            "phone": query.get_or("phone", ""),
            "countrycode": query.get_or("countrycode", "86"),
            "rememberLogin": "true"
        });

        if let Some(captcha) = query.get("captcha") {
            // 验证码登录：只发 captcha，不发 password。参考 Node.js 参考实现
            // 的动态字段名（[captcha ? 'captcha' : 'password']），两个字段互斥；
            // password 一旦存在，服务器就走密码分支把验证码当密码比对，
            // 返回“账号或密码错误”（upstream issue #5）。
            data["captcha"] = Value::String(captcha.to_string());
        } else {
            let password = if let Some(md5_pwd) = query.get("md5_password") {
                md5_pwd.to_string()
            } else if let Some(pwd) = query.get("password") {
                format!("{:x}", Md5::digest(pwd.as_bytes()))
            } else {
                String::new()
            };
            data["password"] = Value::String(password);
        }

        self.request(
            "/api/w/login/cellphone",
            data,
            query.to_option(CryptoType::Weapi),
        )
        .await
    }
}
