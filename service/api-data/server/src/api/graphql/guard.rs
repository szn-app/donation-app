use crate::access_control;
use crate::api::graphql::service::{DataContext, GlobalContext};
use async_graphql::{Context, ErrorExtensions, Guard, ResultExt, TypeDirective};

/**
 * Using async_graphql guards to enforce authorization, while using type system directives for metadata to mark graphql objects which require authentication
 */

/// Authorization directive for marking fields or objects that require authorization.
/// This directive only adds metadata and does not enforce authorization.
///
/// # Parameters
/// * `permission` - Optional permission required for authorization
#[TypeDirective(location = "FieldDefinition", location = "Object")]
pub fn auth(permission: Option<String>) {
    // This function only serves as a declaration for the directive
    // validation of input arguments can be added here
}

pub struct AuthorizeUser {
    pub namespace: String,
    pub object: String,
    pub relation: String,
}

impl Guard for AuthorizeUser {
    async fn check(&self, ctx: &Context<'_>) -> async_graphql::Result<()> {
        log::debug!("-->  Guard for graphql resolver");
        let g = ctx.data::<GlobalContext>()?;
        let c = ctx.data::<DataContext>()?;

        log::debug!("app-user-id = {:?}", &c.user_id);

        let keto_client = g.keto_channel_group.read.clone();

        let user_id = c.user_id.as_ref().ok_or_else(|| {
            const ERROR_MESSAGE: &str = "Not authenticated & No user header detected";

            log::error!("{}", ERROR_MESSAGE);

            async_graphql::Error::new(ERROR_MESSAGE).extend_with(|err, e| {
                e.set("code", 401);
                // e.set("message", err.message.clone());
            })
        })?;

        log::debug!(
            "{}:{}#{}@{}",
            &self.namespace,
            &self.object,
            &self.relation,
            &user_id
        );
        let r = access_control::check_permission_for_subject_set(
            keto_client,
            &self.namespace,
            &self.object,
            &self.relation,
            "User",
            user_id,
            "",
        )
        .await;

        match r {
            Ok(true) => {
                log::debug!("successful permission check for user {}", user_id);
                Ok::<(), async_graphql::Error>(())
            }

            Ok(false) => {
                log::info!("Unauthorized (after permission check) for user {}", user_id);

                let error_msg = "Unauthorized";
                return Err(async_graphql::Error::new(error_msg).extend_with(|err, e| {
                    e.set("code", 403);
                    e.set("message", err.message.clone()); // Optional: copy the error message
                }));
            }

            Err(e) => {
                // Convert the error to a string representation
                let error_msg = format!("Permission check call failed: [Keto] {}", e);
                return Err(async_graphql::Error::new(error_msg).extend_with(|err, e| {
                    e.set("code", 500);
                    e.set("message", err.message.clone()); // Optional: copy the error message
                }));
            }
        }
    }
}

pub struct AuthorizeRelationTuple {
    pub namespace: String,
    pub object: String,
    pub relation: String,
    pub subject_id: String,
}

impl Guard for AuthorizeRelationTuple {
    async fn check(&self, ctx: &Context<'_>) -> async_graphql::Result<()> {
        log::debug!("--> Guard for dummyTestSecure @ graphql resolver");
        let g = ctx.data::<GlobalContext>()?;

        let keto_client = g.keto_channel_group.write.clone();

        log::debug!(
            "{}:{}#{}@{}",
            &self.namespace,
            &self.object,
            &self.relation,
            &self.subject_id
        );
        let r = access_control::check_permission_for_subject(
            keto_client,
            &self.namespace,
            &self.object,
            &self.relation,
            &self.subject_id,
        )
        .await;

        match r {
            Ok(true) => {
                log::debug!("successful permission check for user {}", &self.subject_id);
                Ok::<(), async_graphql::Error>(())
            }

            Ok(false) => {
                log::info!(
                    "Unauthorized (after permission check) for user {}",
                    &self.subject_id
                );

                let error_msg = "Unauthorized";
                return Err(async_graphql::Error::new(error_msg).extend_with(|err, e| {
                    e.set("code", 403);
                    e.set("message", err.message.clone()); // Optional: copy the error message
                }));
            }

            Err(e) => {
                // Convert the error to a string representation
                let error_msg = format!("Permission check call failed: [Keto] {}", e);
                return Err(async_graphql::Error::new(error_msg).extend_with(|err, e| {
                    e.set("code", 500);
                    e.set("message", err.message.clone()); // Optional: copy the error message
                }));
            }
        }
    }
}

// TODO: implement tuple set guard wrapper
pub struct AuthorizeRelationTupleSet {}
