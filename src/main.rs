use kanaria::string::UCSStr;
use kanaria::utils::ConvertTarget;
use num_bigint::BigUint;
use percent_encoding::percent_decode;
use serde_json::json;
use std::str::FromStr;
use warp::http::StatusCode;
use warp::reply::{Json, WithStatus};
use warp::{Filter, Reply};

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT")
        .map(|x| x.parse().expect("PORT is not valid"))
        .unwrap_or(3000);

    let use_ansi = std::env::var("NO_COLOR").is_err();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_ansi(use_ansi)
        .init();

    let route = warp::path!("api" / "isodd" / String)
        .and(warp::header::optional("Authorization"))
        .map(service)
        .with(warp::trace::request());

    tracing::info!("starting to serve at port {}", port);
    warp::serve(route).bind(([0, 0, 0, 0], port)).await;
}

fn reply(json: serde_json::Value, status: impl Into<Option<StatusCode>>) -> WithStatus<Json> {
    warp::reply::with_status(
        warp::reply::json(&json),
        status.into().unwrap_or(StatusCode::OK),
    )
}

fn service(value: String, auth: Option<String>) -> impl Reply {
    // nice security, isn't it? :D
    let is_premium = auth.map(|a| a == "IAMPREMIUMUSER").unwrap_or(false);

    let value = match percent_decode(value.as_bytes()).decode_utf8() {
        Ok(v) => v,
        Err(_) => return reply(json!({ "error": "URLが不正です" }), None),
    };

    if value.len() >= 1000 {
        return reply(json!({ "error": "URLが長過ぎます" }), None);
    }

    let result = match is_odd(value) {
        Ok(t) => t,
        Err(_) => return reply(json!({ "error": "サポートしていない数字形式です" }), None),
    };

    #[allow(non_snake_case)]
    let MAX_FREE_VALUE: BigUint = BigUint::from(1_000_000u32);

    if !is_premium && result.parsed_num >= MAX_FREE_VALUE {
        return reply(
            json!({
                "error": format!("このAPIを{}以上の数に対して使用するには、プレミアムユーザーである必要があります。", MAX_FREE_VALUE)
            }),
            warp::http::StatusCode::PAYMENT_REQUIRED,
        );
    }

    let value_str =
        if result.is_negative { "-" } else { "" }.to_string() + &result.parsed_num.to_string();

    let mut reply_json = serde_json::json!({
        "is_odd": result.is_odd,
        "value": value_str,
    });

    if !is_premium {
        reply_json.as_object_mut().unwrap().insert(
            "ad".into(),
            serde_json::Value::String(
                "限界開発鯖は主に高専生で構成される開発者コミュニティです。 https://approvers.dev"
                    .into(),
            ),
        );
    }

    reply(reply_json, None)
}

struct IsOddResult {
    parsed_num: BigUint,
    is_negative: bool,
    is_odd: bool,
}

// Error means parsing error.
// Returns parsed value in 10 radix and whether it is odd number
fn is_odd(num: impl Into<String>) -> Result<IsOddResult, ()> {
    let mut num = num.into();
    let mut is_negative = false;

    if num.starts_with('-') {
        num.remove(0);
        is_negative = true;
    }

    let num = BigUint::from_str(&num)
        .or_else(|_| kanji_number_parser::parse(&num))
        .or_else(|_| roman::from(&num).map(|x| BigUint::from(x as u32)).ok_or(()))
        .or_else(|_| {
            BigUint::from_str(
                &UCSStr::from_str(&num)
                    .narrow(ConvertTarget::NUMBER)
                    .to_string(),
            )
        })
        .map_err(|_| ())?;

    let is_odd = num.bit(0);

    Ok(IsOddResult {
        parsed_num: num,
        is_negative,
        is_odd,
    })
}

#[test]
fn is_odd_test() {
    assert_eq!(is_odd("12345"), Ok((12345u32.into(), true)));
    assert_eq!(is_odd("1234"), Ok((1234u32.into(), false)));
    assert_eq!(is_odd("千二百三十四"), Ok((1234u32.into(), false)));
    assert_eq!(is_odd("千二百三十"), Ok((1230u32.into(), false)));
}
