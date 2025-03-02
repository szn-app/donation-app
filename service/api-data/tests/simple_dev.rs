use axum::response::Json as JsonResponse;
use anyhow::Result;

use api_data::model::PostData;

#[tokio::test]
async fn test_main() -> Result<()> {
    use httpc_test;

    let hc = httpc_test::new_client("http://localhost:3000")?;

    let response = hc.do_get("/get_data").await?;
    assert_eq!(response.status(), 200);
    
    let json_value = response.json_body()?;
    let data: PostData = serde_json::from_value(json_value)?;

    assert_eq!(data.id, 1);

    Ok(())
}
