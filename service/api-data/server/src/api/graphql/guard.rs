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

pub struct AuthorizeUser<'a> {
    pub namespace: &'a str,
    pub object: &'a str,
    pub relation: &'a str,
}

impl<'a> AuthorizeUser<'a> {
    pub fn group_admin_guard() -> Self {
        Self {
            namespace: "Group",
            object: "admin",
            relation: "member",
        }
    }

    pub fn k8s_endpoint_access_guard() -> Self {
        Self {
            namespace: "Endpoint",
            object: "k8s",
            relation: "access",
        }
    }
}

impl<'a> Guard for AuthorizeUser<'a> {
    async fn check(&self, ctx: &Context<'_>) -> async_graphql::Result<()> {
        log::debug!("-->  Guard for AuthorizeUser @ graphql resolver");
        let c = ctx.data::<DataContext>()?;

        log::debug!("app-user-id = {:?}", &c.user_id);

        let user_id = c.user_id.as_ref().ok_or_else(|| {
            const ERROR_MESSAGE: &str = "Not authenticated & No user header detected";
            log::error!("{}", ERROR_MESSAGE);
            async_graphql::Error::new(ERROR_MESSAGE).extend_with(|err, e| {
                e.set("code", 401);
            })
        })?;

        // Convert AuthorizeUser to AuthorizeRelationTuple
        let tuple = AuthorizeRelationTuple {
            namespace: self.namespace,
            object: self.object,
            relation: self.relation,
            subject_namespace: Some("User"),
            subject_object: user_id,
            subject_relation: None,
        };

        tuple.check(ctx).await
    }
}

pub struct AuthorizeRelationTuple<'a> {
    pub namespace: &'a str,
    pub object: &'a str,
    pub relation: &'a str,
    pub subject_namespace: Option<&'a str>,
    pub subject_object: &'a str,
    pub subject_relation: Option<&'a str>,
}

impl<'a> Guard for AuthorizeRelationTuple<'a> {
    async fn check(&self, ctx: &Context<'_>) -> async_graphql::Result<()> {
        log::debug!("--> Guard for AuthorizeRelationTuple @ graphql resolver");
        let g = ctx.data::<GlobalContext>()?;
        let keto_client = g.keto_channel_group.read.clone();

        let r = if self.subject_namespace.is_none() && self.subject_relation.is_none() {
            // If only object is provided, use check_permission_for_subject
            log::debug!(
                "{}:{}#{}@{}",
                self.namespace,
                self.object,
                self.relation,
                self.subject_object
            );
            access_control::check_permission_for_subject(
                keto_client,
                self.namespace,
                self.object,
                self.relation,
                self.subject_object,
            )
            .await
        } else {
            // If subject set information is provided, use check_permission_for_subject_set
            log::debug!(
                "{}:{}#{}@{}:{}#{}",
                self.namespace,
                self.object,
                self.relation,
                self.subject_namespace.unwrap_or(""),
                self.subject_object,
                self.subject_relation.unwrap_or("")
            );
            access_control::check_permission_for_subject_set(
                keto_client,
                self.namespace,
                self.object,
                self.relation,
                self.subject_namespace.unwrap_or(""),
                self.subject_object,
                self.subject_relation.unwrap_or(""),
            )
            .await
        };

        match r {
            Ok(true) => {
                log::debug!(
                    "successful permission check for subject {}",
                    self.subject_object
                );
                Ok::<(), async_graphql::Error>(())
            }

            Ok(false) => {
                log::info!(
                    "Unauthorized (after permission check) for subject {}",
                    self.subject_object
                );

                let error_msg = "Unauthorized";
                return Err(async_graphql::Error::new(error_msg).extend_with(|err, e| {
                    e.set("code", 403);
                    e.set("message", err.message.clone());
                }));
            }

            Err(e) => {
                let error_msg = format!("Permission check call failed: [Keto] {}", e);
                return Err(async_graphql::Error::new(error_msg).extend_with(|err, e| {
                    e.set("code", 500);
                    e.set("message", err.message.clone());
                }));
            }
        }
    }
}
