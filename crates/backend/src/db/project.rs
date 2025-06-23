use super::Database;
use crate::error::ApiError;
use log::debug;

use crate::db::entity::project;
use sea_orm::ActiveValue::{NotSet, Set, Unchanged};
use sea_orm::{ActiveModelTrait, DeleteResult, EntityTrait};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

// TODO: Move the struct out of here into the controller
#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateProject {
    #[validate(length(min = 3, max = 255))]
    /// Project name (minimum 3 characters and maximum 255 characters)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn test_validation_create_project_struct_valid() {
        let project = CreateProject {
            name: "Test Project".to_string(),
        };
        let validation_result = project.validate();
        assert!(validation_result.is_ok());
    }

    #[actix_web::test]
    async fn test_validation_create_project_struct_invalid_too_short() {
        let project = CreateProject {
            name: "TP".to_string(), // too short
        };
        let validation_result = project.validate();
        assert!(validation_result.is_err());
    }

    #[actix_web::test]
    async fn test_validation_create_project_struct_empty() {
        let project = CreateProject {
            name: "".to_string(), // empty string
        };
        let validation_result = project.validate();
        assert!(validation_result.is_err());
    }

    #[actix_web::test]
    async fn test_validation_create_project_struct_min_length() {
        let project = CreateProject {
            name: "abc".to_string(), // exactly at min length
        };
        let validation_result = project.validate();
        assert!(validation_result.is_ok());
    }

    #[actix_web::test]
    async fn test_validation_create_project_struct_long_name() {
        // 256 characters long should be invalid because of max length
        let long_name = "a".repeat(256);
        let project = CreateProject { name: long_name };
        let validation_result = project.validate();
        assert!(validation_result.is_err());
    }
}
