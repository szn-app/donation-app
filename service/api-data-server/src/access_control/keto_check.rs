use shared_protobuf_keto::proto::relation_tuples::{
    check_service_client::CheckServiceClient, subject, CheckRequest, RelationTuple, Subject,
    SubjectSet,
}; // spec: https://github.com/ory/keto/blob/master/proto/buf.md
use std::error::Error;
use tonic::{self, transport::Channel};
use log;

/// Checks if a subject has a specific relation with an object
///
/// This is a convenience method that combines functionality of
/// check_permission_for_subject and check_permission_for_subject_set
///
/// # Arguments
///
/// * `namespace` - The namespace (e.g., "files")
/// * `object` - The object ID (e.g., "document-1")
/// * `relation` - The relation to check (e.g., "view")
/// * `subject_id` - The subject ID (e.g., "user-1")
/// * `subject_set` - Optional tuple of (namespace, object, relation) if using subject sets
pub async fn check_permission(
    keto_grpc_channel: Channel,
    namespace: &str,
    object: &str,
    relation: &str,
    subject_id: Option<&str>,
    subject_set: Option<(&str, &str, &str)>,
) -> Result<bool, Box<dyn Error>> {
    match (subject_id, subject_set) {
        (Some(id), None) => {
            check_permission_for_subject(keto_grpc_channel, namespace, object, relation, id).await
        }
        (None, Some((set_namespace, set_object, set_relation))) => {
            check_permission_for_subject_set(
                keto_grpc_channel,
                namespace,
                object,
                relation,
                set_namespace,
                set_object,
                set_relation,
            )
            .await
        }
        _ => {
            Err("Either subject_id or subject_set must be provided, but not both or neither".into())
        }
    }
}

/// Checks if a subject with the given ID has a specific relation with an object
///
/// # Arguments
///
/// * `namespace` - The namespace (e.g., "files")
/// * `object` - The object ID (e.g., "document-1")
/// * `relation` - The relation to check (e.g., "view")
/// * `subject_id` - The subject ID (e.g., "user-1")
pub async fn check_permission_for_subject(
    keto_grpc_channel: Channel,
    namespace: &str,
    object: &str,
    relation: &str,
    subject_id: &str,
) -> Result<bool, Box<dyn Error>> {
    let subject = Some(Subject {
        r#ref: Some(subject::Ref::Id(subject_id.to_string())),
    });

    let relation_tuple = RelationTuple {
        namespace: namespace.to_string(),
        object: object.to_string(),
        relation: relation.to_string(),
        subject,
    };

    log::debug!("{:?}", relation_tuple); 

    let request = tonic::Request::new(CheckRequest {
        tuple: Some(relation_tuple),
        max_depth: 10,
        ..Default::default()
    });
    let mut client = CheckServiceClient::new(keto_grpc_channel);
    let response = client.check(request).await?;

    // Extract the allowed field from the response
    Ok(response.into_inner().allowed)
}

/// Checks if a subject set has a specific relation with an object
///
/// # Arguments
///
/// * `namespace` - The namespace (e.g., "files")
/// * `object` - The object ID (e.g., "document-1")
/// * `relation` - The relation to check (e.g., "view")
/// * `subject_set_namespace` - The namespace of the subject set (e.g., "groups")
/// * `subject_set_object` - The object ID of the subject set (e.g., "admins")
/// * `subject_set_relation` - The relation of the subject set (e.g., "member")
pub async fn check_permission_for_subject_set(
    keto_grpc_channel: Channel,
    namespace: &str,
    object: &str,
    relation: &str,
    subject_set_namespace: &str,
    subject_set_object: &str,
    subject_set_relation: &str,
) -> Result<bool, Box<dyn Error>> {
    let subject = Some(Subject {
        r#ref: Some(subject::Ref::Set(SubjectSet {
            namespace: subject_set_namespace.to_string(),
            object: subject_set_object.to_string(),
            relation: subject_set_relation.to_string(),
        })),
    });

    let relation_query = RelationTuple {
        namespace: namespace.to_string(),
        object: object.to_string(),
        relation: relation.to_string(),
        subject,
    };

    let request = tonic::Request::new(CheckRequest {
        tuple: Some(relation_query),
        max_depth: 10, // maximum depth to search for a relation (default: 10)
        ..Default::default()
    });

    let mut client = CheckServiceClient::new(keto_grpc_channel);
    let response = client.check(request).await?;

    // Extract the allowed field from the response
    Ok(response.into_inner().allowed)
}

// TODO: implement batch checks https://github.com/ory/keto/blob/master/proto/buf.md#ory-keto-relation_tuples-v1alpha2-Subject
pub async fn batch_check_permission() {}

// $`cargo test -q access_control::keto_check::tests::test_keto_permission_check -- --exact --nocapture`
#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::server::connection::keto::KetoChannelGroup;

    #[tokio::test]
    async fn test_keto_permission_check() {
        const KETO_ENDPOINT: &str = "localhost:4466";

        let KetoChannelGroup {
            read: read_channel,
            write: _,
        } = KetoChannelGroup::new(&KETO_ENDPOINT, &KETO_ENDPOINT)
            .await
            .expect("Failed to create gRPC channel");

        let r = check_permission_for_subject(read_channel, "Test", "object", "relation", "subject")
            .await; // Test:object#relation@subject

        println!("{:?}", &r);

        assert!(r.unwrap(), "Permission check returned false");
    }

    #[tokio::test]
    async fn test_keto_permission_check_with_subject_set() {
        const KETO_ENDPOINT: &str = "localhost:4466";

        let KetoChannelGroup {
            read: read_channel,
            write: _,
        } = KetoChannelGroup::new(&KETO_ENDPOINT, &KETO_ENDPOINT)
            .await
            .expect("Failed to create gRPC channel");

        let r = check_permission_for_subject_set(
            read_channel,
            "Test",
            "object",
            "relation",
            "Test",
            "subject",
            "relation",
        )
        .await; // Test:object#relation@(Test:subject#relation)

        println!("{:?}", &r);

        assert!(r.unwrap(), "Permission check returned false");
    }

    #[tokio::test]
    async fn test_keto_permission_check_with_subject_namespaced() {
        const KETO_ENDPOINT: &str = "localhost:4466";

        let KetoChannelGroup {
            read: read_channel,
            write: _,
        } = KetoChannelGroup::new(&KETO_ENDPOINT, &KETO_ENDPOINT)
            .await
            .expect("Failed to create gRPC channel");

        let r = check_permission_for_subject_set(
            read_channel,
            "Endpoint",
            "test",
            "access",
            "Group",
            "admin",
            "",
        )
        .await; // Endpoint:test#access@(Group:admin)

        println!("{:?}", &r);

        assert!(r.unwrap(), "Permission check returned false");
    }
}
