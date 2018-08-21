use rocket_contrib::json::Json;
use serde_json::Value;

#[catch(400)]
pub fn bad_request() -> Json<Value> {
    Json(json!({
        "type": "bad_request",
        "message": "リクエストが不正です"
    }))
}

#[catch(401)]
pub fn unauthorized() -> Json<Value> {
    Json(json!({
        "status": "unauthorized",
        "message": "認証が必要です"
    }))
}

#[catch(402)]
pub fn payment_required() -> Json<Value> {
    Json(json!({
        "status": "payment_required",
        "message": "支払いが必要です。"
    }))
}

#[catch(403)]
pub fn forbidden() -> Json<Value> {
    Json(json!({
        "status": "forbidden",
        "message": "このリソースへのアクセスは禁止されています"
    }))
}

#[catch(404)]
pub fn not_found() -> Json<Value> {
    Json(json!({
        "status": "not_found",
        "message": "リソースが存在しません"
    }))
}

#[catch(405)]
pub fn method_not_allowed() -> Json<Value> {
    Json(json!({
        "status": "method_not_allowed",
        "message": "許可されていないメソッドでのアクセスです"
    }))
}

#[catch(406)]
pub fn not_acceptable() -> Json<Value> {
    Json(json!({
        "status": "not_acceptable",
        "message": "受理できないリクエストです"
    }))
}

#[catch(407)]
pub fn proxy_authentication_required() -> Json<Value> {
    Json(json!({
        "status": "proxy_authentication_required",
        "message": "プロキシを通してアクセスする必要があります"
    }))
}

#[catch(408)]
pub fn request_timeout() -> Json<Value> {
    Json(json!({
        "status": "request_timeout",
        "message": "リクエストがタイムアウトしました"
    }))
}

#[catch(409)]
pub fn confrict() -> Json<Value> {
    Json(json!({
        "status": "confrict",
        "message": "リクエストが競合しました"
    }))
}

#[catch(410)]
pub fn gone() -> Json<Value> {
    Json(json!({
        "status": "gone",
        "message": "そのリソースは消滅しています"
    }))
}

#[catch(411)]
pub fn length_required() -> Json<Value> {
    Json(json!({
        "status": "length_required",
        "message": "ヘッダーにContent-Lengthパラメーターが必要です"
    }))
}

#[catch(412)]
pub fn procondition_failed() -> Json<Value> {
    Json(json!({
        "status": "procondition_failed",
        "message": "前提条件で失敗しました"
    }))
}

#[catch(413)]
pub fn payload_too_large() -> Json<Value> {
    Json(json!({
        "status": "payload_too_large",
        "message": "ペイロードがおおきすぎます"
    }))
}

#[catch(500)]
pub fn internal_error() -> Json<Value> {
    Json(json!({
        "status": "internal_error",
        "message": "内部的な問題が発生しております"
    }))
}

#[catch(501)]
pub fn not_implemented() -> Json<Value> {
    Json(json!({
        "status": "not_implemented",
        "message": "実装されていないAPIです"
    }))
}

#[catch(502)]
pub fn bad_gateway() -> Json<Value> {
    Json(json!({
        "status": "bad_gateway",
        "message": "不正なゲートウェイです"
    }))
}

#[catch(503)]
pub fn service_unavailable() -> Json<Value> {
    Json(json!({
        "status": "service_unavailable",
        "message": "混雑・メンテナンスのため一時的にサービスが使用不可能です"
    }))
}

#[catch(504)]
pub fn gateway_timeout() -> Json<Value> {
    Json(json!({
        "status": "gateway_timeout",
        "message": "ゲートウェイでタイムアウトしました"
    }))
}
