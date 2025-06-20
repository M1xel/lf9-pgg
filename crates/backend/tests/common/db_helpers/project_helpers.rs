use crate::common::test_helpers::TestContext;
use backend::{
    Database,
    db::{entity, project::CreateProject},
};
use uuid::Uuid;

impl TestContext {
    pub async fn create_project(
        &self,
        db: &Database,
        name: Option<String>,
    ) -> Result<entity::project::Model, backend::error::ApiError> {
        let name = name.unwrap_or_else(|| format!("Test Project {}", self.test_id));
        let create_project = CreateProject { name };

        let project = db.create_project(create_project).await?;

        if let Ok(mut projects) = self.created_projects.lock() {
            projects.push(project.id);
        }

        Ok(project)
    }

    pub async fn create_project_with_name(
        &self,
        db: &Database,
        name: String,
    ) -> Result<entity::project::Model, backend::error::ApiError> {
        self.create_project(db, Some(name)).await
    }

    pub async fn create_multiple_projects(
        &self,
        db: &Database,
        count: usize,
    ) -> Result<Vec<entity::project::Model>, backend::error::ApiError> {
        let mut projects = Vec::new();

        for i in 0..count {
            let name = format!("Test Project {} {}", self.test_id, i);
            let project = self.create_project(db, Some(name)).await?;
            projects.push(project);
        }

        Ok(projects)
    }

    pub async fn get_project_by_id(
        &self,
        db: &Database,
        id: &Uuid,
    ) -> Result<Option<entity::project::Model>, backend::error::ApiError> {
        db.get_project(id).await
    }

    pub async fn get_all_projects(
        &self,
        db: &Database,
    ) -> Result<Vec<entity::project::Model>, backend::error::ApiError> {
        db.get_projects().await
    }

    pub async fn update_project(
        &self,
        db: &Database,
        id: &Uuid,
        name: String,
    ) -> Result<entity::project::Model, backend::error::ApiError> {
        let update_data = CreateProject { name };
        db.update_project(id, update_data).await
    }

    pub async fn assert_project_exists(&self, db: &Database, id: &Uuid) -> bool {
        match self.get_project_by_id(db, id).await {
            Ok(Some(_)) => true,
            _ => false,
        }
    }

    pub async fn assert_project_count(&self, db: &Database, expected: usize) -> bool {
        match self.get_all_projects(db).await {
            Ok(projects) => projects.len() == expected,
            Err(_) => false,
        }
    }

    pub async fn assert_project_not_exists(&self, db: &Database, id: &Uuid) -> bool {
        !self.assert_project_exists(db, id).await
    }

    pub async fn assert_project_name(&self, db: &Database, id: &Uuid, expected_name: &str) -> bool {
        match self.get_project_by_id(db, id).await {
            Ok(Some(project)) => project.name == expected_name,
            _ => false,
        }
    }

    pub async fn delete_project(
        &self,
        db: &Database,
        id: &Uuid,
    ) -> Result<(), backend::error::ApiError> {
        db.delete_project(id).await?;

        if let Ok(mut projects) = self.created_projects.lock() {
            projects.retain(|&project_id| project_id != *id);
        }

        Ok(())
    }

    pub async fn cleanup_projects(&self, db: &Database) {
        if let Ok(projects) = self.created_projects.lock() {
            for project_id in projects.iter() {
                let _ = db.delete_project(project_id).await;
            }
        }

        if let Ok(mut projects) = self.created_projects.lock() {
            projects.clear();
        }
    }
}
