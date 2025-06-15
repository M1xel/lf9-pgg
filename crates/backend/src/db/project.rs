use super::Database;
use crate::error::ApiError;
use log::debug;

use crate::entity::project;
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};
use sea_orm::{ActiveModelTrait, DeleteResult, EntityTrait};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;

#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateProject {
    #[validate(length(min = 3))]
    /// Project name (minimum 3 characters)
    pub name: String,
}

impl Database {
    pub async fn get_projects(&self) -> Result<Vec<project::Model>, ApiError> {
        debug!("Fetching all projects");

        let projects = project::Entity::find().all(&self.conn).await?;
        Ok(projects)
    }

    pub async fn get_project(&self, id: &Uuid) -> Result<Option<project::Model>, ApiError> {
        debug!("Fetching project with id: {}", id);

        let project = project::Entity::find_by_id(id.to_owned())
            .one(&self.conn)
            .await?;

        if project.is_none() {
            return Err(ApiError::NotFound);
        }
        Ok(project)
    }

    pub async fn create_project(
        &self,
        create_project: CreateProject,
    ) -> Result<project::Model, ApiError> {
        debug!("Creating project with name: {}", create_project.name);

        let project = project::ActiveModel {
            id: NotSet,
            name: Set(create_project.name),
        };

        let project = project.insert(&self.conn).await?;
        Ok(project)
    }

    pub async fn update_project(
        &self,
        id: &Uuid,
        project: CreateProject,
    ) -> Result<project::Model, ApiError> {
        debug!("Updating project with id: {}", &id);

        let active_model = project::ActiveModel {
            id: Unchanged(*id),
            name: Set(project.name),
        };

        let project = active_model.update(&self.conn).await?;

        Ok(project)
    }

    pub async fn delete_project(&self, id: &Uuid) -> Result<DeleteResult, ApiError> {
        debug!("Deleting project with id: {}", id);

        let project = project::Entity::delete_by_id(id.to_owned())
            .exec(&self.conn)
            .await?;

        if project.rows_affected == 0 {
            return Err(ApiError::NotFound);
        }

        Ok(project)
    }
}
