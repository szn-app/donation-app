use anyhow::Result;
use axum::response::Json as JsonResponse;

use api_data_server::api::rest::handler::PostData;

#[tokio::test]
#[ignore = "This test requires a local server to be running"]
async fn test_main() -> Result<()> {
    use httpc_test;

    let hc = httpc_test::new_client("http://localhost:3000")?;

    let response = hc.do_get("/dummy_get_data").await?;
    assert_eq!(response.status(), 200);

    let json_value = response.json_body()?;
    let data: PostData = serde_json::from_value(json_value)?;

    assert_eq!(data.id, 1);

    Ok(())
}
