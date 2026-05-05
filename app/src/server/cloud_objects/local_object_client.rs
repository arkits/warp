//! No-op [`ObjectClient`] for local-first (no cloud) builds.
//!
//! All write operations return errors immediately — the SyncQueue will emit
//! ObjectCreationFailure / ObjectUpdateFailure events and the UpdateManager
//! handles those gracefully without retrying.
//!
//! [`fetch_changed_objects`] returns an empty success so that the UpdateManager
//! completes its initial load and unlocks downstream subscribers.

use std::{collections::HashMap, sync::Arc};

use anyhow::{anyhow, Result};
use async_channel::Sender;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use warp_graphql::object_permissions::AccessLevel;

use crate::{
    cloud_object::{
        model::{
            actions::{ObjectActionHistory, ObjectActionType},
            generic_string_model::GenericStringObjectId,
        },
        BulkCreateCloudObjectResult, BulkCreateGenericStringObjectsRequest,
        CreateCloudObjectResult, CreateObjectRequest, GenericStringObjectFormat,
        GenericStringObjectUniqueKey, JsonObjectType, ObjectDeleteResult, ObjectMetadataUpdateResult,
        ObjectPermissionUpdateResult, ObjectPermissionsUpdateData, ObjectType, ObjectsToUpdate,
        Owner, Revision, ServerFolder, ServerMetadata, ServerNotebook, ServerObject,
        ServerPermissions, ServerWorkflow, UpdateCloudObjectResult,
    },
    drive::{folders::FolderId, sharing::SharingAccessLevel},
    notebooks::NotebookId,
    server::{
        cloud_objects::{
            listener::ObjectUpdateMessage,
            update_manager::{GetCloudObjectResponse, InitialLoadResponse},
        },
        ids::ServerId,
        server_api::object::{GuestIdentifier, ObjectClient},
        sync_queue::SerializedModel,
    },
    workflows::WorkflowId,
};

pub struct LocalObjectClient;

impl LocalObjectClient {
    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

#[async_trait]
impl ObjectClient for LocalObjectClient {
    async fn create_workflow(&self, _request: CreateObjectRequest) -> Result<CreateCloudObjectResult> {
        Err(anyhow!("local-only: no cloud connection"))
    }

    async fn update_workflow(
        &self,
        _workflow_id: WorkflowId,
        _data: SerializedModel,
        _revision: Option<Revision>,
    ) -> Result<UpdateCloudObjectResult<ServerWorkflow>> {
        Err(anyhow!("local-only: no cloud connection"))
    }

    async fn bulk_create_generic_string_objects(
        &self,
        _owner: Owner,
        _objects: &[BulkCreateGenericStringObjectsRequest],
    ) -> Result<BulkCreateCloudObjectResult> {
        Err(anyhow!("local-only: no cloud connection"))
    }

    async fn create_generic_string_object(
        &self,
        _format: GenericStringObjectFormat,
        _uniqueness_key: Option<GenericStringObjectUniqueKey>,
        _request: CreateObjectRequest,
    ) -> Result<CreateCloudObjectResult> {
        Err(anyhow!("local-only: no cloud connection"))
    }

    async fn create_notebook(&self, _request: CreateObjectRequest) -> Result<CreateCloudObjectResult> {
        Err(anyhow!("local-only: no cloud connection"))
    }

    async fn update_notebook(
        &self,
        _notebook_id: NotebookId,
        _title: Option<String>,
        _data: Option<SerializedModel>,
        _revision: Option<Revision>,
    ) -> Result<UpdateCloudObjectResult<ServerNotebook>> {
        Err(anyhow!("local-only: no cloud connection"))
    }

    async fn create_folder(&self, _request: CreateObjectRequest) -> Result<CreateCloudObjectResult> {
        Err(anyhow!("local-only: no cloud connection"))
    }

    async fn update_folder(
        &self,
        _folder_id: FolderId,
        _name: SerializedModel,
    ) -> Result<UpdateCloudObjectResult<ServerFolder>> {
        Err(anyhow!("local-only: no cloud connection"))
    }

    async fn update_generic_string_object(
        &self,
        _object_id: GenericStringObjectId,
        _model: SerializedModel,
        _revision: Option<Revision>,
    ) -> Result<UpdateCloudObjectResult<Box<dyn ServerObject>>> {
        Err(anyhow!("local-only: no cloud connection"))
    }

    async fn grab_notebook_edit_access(&self, _notebook_id: NotebookId) -> Result<ServerMetadata> {
        Err(anyhow!("local-only: no cloud connection"))
    }

    async fn give_up_notebook_edit_access(
        &self,
        _notebook_id: NotebookId,
    ) -> Result<ServerMetadata> {
        Err(anyhow!("local-only: no cloud connection"))
    }

    async fn get_warp_drive_updates(
        &self,
        _message_sender: Sender<ObjectUpdateMessage>,
        _stream_ready_sender: Sender<()>,
    ) -> Result<()> {
        Ok(())
    }

    async fn fetch_changed_objects(
        &self,
        _objects_to_update: ObjectsToUpdate,
        _force_refresh: bool,
    ) -> Result<InitialLoadResponse> {
        Ok(InitialLoadResponse::default())
    }

    async fn fetch_single_cloud_object(&self, _id: ServerId) -> Result<GetCloudObjectResponse> {
        Err(anyhow!("local-only: no cloud connection"))
    }

    async fn transfer_notebook_owner(
        &self,
        _notebook_id: NotebookId,
        _owner: Owner,
    ) -> Result<bool> {
        Err(anyhow!("local-only: no cloud connection"))
    }

    async fn transfer_workflow_owner(
        &self,
        _workflow_id: WorkflowId,
        _owner: Owner,
    ) -> Result<bool> {
        Err(anyhow!("local-only: no cloud connection"))
    }

    async fn transfer_generic_string_object_owner(
        &self,
        _workflow_id: GenericStringObjectId,
        _owner: Owner,
    ) -> Result<bool> {
        Err(anyhow!("local-only: no cloud connection"))
    }

    async fn trash_object(&self, _id: ServerId) -> Result<bool> {
        Err(anyhow!("local-only: no cloud connection"))
    }

    async fn untrash_object(&self, _id: ServerId) -> Result<ObjectMetadataUpdateResult> {
        Err(anyhow!("local-only: no cloud connection"))
    }

    async fn delete_object(&self, _id: ServerId) -> Result<ObjectDeleteResult> {
        Err(anyhow!("local-only: no cloud connection"))
    }

    async fn empty_trash(&self, _owner: Owner) -> Result<ObjectDeleteResult> {
        Err(anyhow!("local-only: no cloud connection"))
    }

    async fn move_object(
        &self,
        _id: ServerId,
        _folder_id: Option<FolderId>,
        _owner: Owner,
        _object_type: ObjectType,
    ) -> Result<bool> {
        Err(anyhow!("local-only: no cloud connection"))
    }

    async fn record_object_action(
        &self,
        _id: ServerId,
        _action_type: ObjectActionType,
        _timestamp: DateTime<Utc>,
        _data: Option<String>,
    ) -> Result<ObjectActionHistory> {
        Err(anyhow!("local-only: no cloud connection"))
    }

    async fn leave_object(&self, _id: ServerId) -> Result<ObjectDeleteResult> {
        Err(anyhow!("local-only: no cloud connection"))
    }

    async fn set_object_link_permissions(
        &self,
        _object_id: ServerId,
        _access_level: SharingAccessLevel,
    ) -> Result<ObjectPermissionUpdateResult> {
        Err(anyhow!("local-only: no cloud connection"))
    }

    async fn remove_object_link_permissions(
        &self,
        _object_id: ServerId,
    ) -> Result<ObjectPermissionUpdateResult> {
        Err(anyhow!("local-only: no cloud connection"))
    }

    async fn add_object_guests(
        &self,
        _object_id: ServerId,
        _guest_emails: Vec<String>,
        _access_level: AccessLevel,
    ) -> Result<ObjectPermissionsUpdateData> {
        Err(anyhow!("local-only: no cloud connection"))
    }

    async fn update_object_guests(
        &self,
        _object_id: ServerId,
        _guest_emails: Vec<String>,
        _access_level: AccessLevel,
    ) -> Result<ServerPermissions> {
        Err(anyhow!("local-only: no cloud connection"))
    }

    async fn remove_object_guest(
        &self,
        _object_id: ServerId,
        _guest: GuestIdentifier,
    ) -> Result<ServerPermissions> {
        Err(anyhow!("local-only: no cloud connection"))
    }

    async fn fetch_environment_last_task_run_timestamps(
        &self,
    ) -> Result<HashMap<String, DateTime<Utc>>> {
        Ok(HashMap::new())
    }
}
