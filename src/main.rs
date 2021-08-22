use warp::Filter;

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

    let route = warp::path!("api" / "isodd" / u128)
        .and(warp::header::optional("Authorization"))
        .map(|value, auth: Option<String>| {
            let is_premium = auth.map(|a| a == "IAMPREMIUMUSER").unwrap_or(false);

            if !is_premium && value >= 3000 {
                return warp::reply::with_status(
                    warp::reply::json(&serde_json::json!({"error": "このAPIを3000以上の数に対して使用するには、プレミアムユーザーである必要があります。"})),
                    warp::http::StatusCode::PAYMENT_REQUIRED,
                );
            }

            let mut reply = serde_json::json!({ "isodd": value % 2 != 0 });

            if !is_premium {
                reply
                    .as_object_mut()
                    .unwrap()
                    .insert(
                        "ad".into(),
                        serde_json::Value::String("限界開発鯖は主に高専生で構成される開発者コミュニティです。 https://approvers.dev".into())
                    );
            }

            warp::reply::with_status(warp::reply::json(&reply), warp::http::StatusCode::OK)
        })
        .with(warp::trace::request());

    tracing::info!("starting to serve at port {}", port);
    warp::serve(route).bind(([0, 0, 0, 0], port)).await;
}
