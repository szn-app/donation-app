use axum::{
    extract::{Form, Query},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;

pub const HYDRA_TOKEN_URL: &str = "http://hydra-public.auth.svc/oauth2/token";

// params from frontend request
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct TokenRequest {
    pub grant_type: String,
    pub code: Option<String>,
    pub redirect_uri: Option<String>,
    pub code_verifier: Option<String>,
    pub scope: Option<String>,
    pub refresh_token: Option<String>,
}

// response to frontend token exchange
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub id_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub scope: String,
}

pub fn routes() -> Router {
    fn ouath2_routes() -> Router {
        Router::new().route("/oauth2_token", post(oauth2_token))
    }

    Router::new()
        .route("/health/status", get(health_status))
        .merge(ouath2_routes())
        .fallback(handler_404)
}

// Hydra request params
#[derive(Debug)]
struct HydraTokenForm<'a> {
    grant_type: String,
    code: &'a String,
    redirect_uri: &'a String,
    client_id: String,
    client_secret: &'a String,
    code_verifier: &'a String,
}

#[derive(Debug)]
struct HydraRefreshTokenForm<'a> {
    grant_type: String,
    client_id: String,
    scope: String,
    client_secret: &'a String,
    refresh_token: String,
}

impl<'a> HydraTokenForm<'a> {
    fn as_tuples(&self) -> [(&str, &str); 6] {
        [
            ("grant_type", &self.grant_type),
            ("code", &self.code),
            ("redirect_uri", &self.redirect_uri),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
            ("code_verifier", &self.code_verifier),
        ]
    }
}

impl<'a> HydraRefreshTokenForm<'a> {
    fn as_tuples(&self) -> [(&str, &str); 5] {
        [
            ("grant_type", &self.grant_type),
            ("client_id", &self.client_id),
            ("scope", &self.scope),
            ("client_secret", &self.client_secret),
            ("refresh_token", &self.refresh_token),
        ]
    }
}

// OIDC Relying Party (RP) logic
// TODO: use proper tracing library for production
pub async fn oauth2_token(
    Extension(secret_value): Extension<String>,
    form: Form<TokenRequest>,
) -> impl IntoResponse {
    println!(
        "--> oauth2_token endpoint/handler called with payload: {:?}",
        form
    );

    let hydra_client = reqwest::Client::builder().build().unwrap();

    let response: Result<reqwest::Response, reqwest::Error> = match form.grant_type.as_str() {
        "authorization_code" => {
            println!("--> authorization code flow detected");

            let hydra_form = HydraTokenForm {
                grant_type: "authorization_code".to_string(),
                code: form.code.as_ref().unwrap(),
                redirect_uri: form.redirect_uri.as_ref().unwrap(),
                client_id: "frontend-client".to_string(),
                client_secret: &secret_value,
                code_verifier: &form.code_verifier.as_ref().unwrap(),
            };

            hydra_client
                .post(HYDRA_TOKEN_URL)
                .form(&hydra_form.as_tuples())
                .send()
                .await
        }

        // https://openid.net/specs/openid-connect-core-1_0.html#RefreshTokens
        "refresh_token" => {
            println!("--> Refresh token flow detected");

            /*
            curl -k -i --request POST --url https://auth.donation-app.test/authorize/oauth2/token --header "accept: application/x-www-form-urlencoded" \
                --form "grant_type=refresh_token" \
                --form "client_id=frontend-client"  \
                --form "client_secret=UF...g=" \
                --form 'refresh_token="ory_rt_..."' \
                --form "scope=offline_access openid email profile"
             */
            let hydra_form = HydraRefreshTokenForm {
                grant_type: "refresh_token".to_string(),
                refresh_token: form.refresh_token.as_ref().unwrap().to_string(),
                client_id: "frontend-client".to_string(),
                client_secret: &secret_value,
                scope: form.scope.as_ref().unwrap().to_string(),
            };

            println!("--> Refresh token form: {:?}", &hydra_form);

            hydra_client
                .post(HYDRA_TOKEN_URL)
                .form(&hydra_form.as_tuples())
                .send()
                .await
        }
        _ => {
            println!("--> Unsupported grant type: {}", form.grant_type);
            return StatusCode::BAD_REQUEST.into_response();
        }
    };

    match response {
        Ok(res) => {
            // Debugging
            println!("[debug] {:?}", &res);

            let res_body = res.text().await.unwrap_or_default();
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&res_body) {
                println!("Response JSON: {:?}", json);
            }

            match serde_json::from_str::<TokenResponse>(&res_body) {
                Ok(token) => Json(token).into_response(),
                Err(e) => {
                    println!("--> Failed to parse token response: {:?}", e);
                    StatusCode::INTERNAL_SERVER_ERROR.into_response()
                }
            }
        }
        Err(e) => {
            println!("--> Request to hydra token endpoint failed: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn handler_404() -> impl IntoResponse {
    println!("--> fallback 404 handler called");

    (
        StatusCode::NOT_FOUND,
        Html("<h1>404</h1><p>Nothing to see here</p>"),
    )
}

pub async fn health_status() -> impl IntoResponse {
    println!("--> Health status endpoint called");

    Html(format!("ok")).into_response()
}
