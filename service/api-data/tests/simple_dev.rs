use axum::response::Json as JsonResponse;
use anyhow::Result;

use api_data::model::PostData;

#[tokio::test]
async fn test_main() -> Result<()> {
    use httpc_test;

    let hc = httpc_test::new_client("http://localhost:3000")?;

    let response: PostData = hc.get("/get_data").await?;

    dbg!(&response);

    assert_eq!(response.id, 1);

    Ok(())
}
