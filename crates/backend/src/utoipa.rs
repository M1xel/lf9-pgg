use utoipa::OpenApi;

use crate::{controller, db, db::entity, error};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "PGG API",
        description = "API for the PGG (Peer Group Grading) application",
        version = "0.0.1-rc1",
    ),
    paths(
        controller::auth::login,
        controller::auth::logout,
        controller::project::get_projects,
        controller::project::get_project,
        controller::project::create_project,
        controller::project::update_project,
        controller::project::delete_project,
        controller::user::get_users,
        controller::user::get_user,
        controller::user::create_user,
        controller::user::update_user,
        controller::user::delete_user,
        controller::group::get_groups,
        controller::group::get_groups_for_project,
        controller::group::create_group,
        controller::group::update_group,
        controller::group::delete_group,
        controller::class::get_classes,
        controller::class::get_class,
        controller::class::create_class,
        controller::class::update_class,
        controller::class::delete_class,
        controller::template::get_templates,
        controller::template::get_template,
        controller::template::create_template,
        controller::template::update_template,
        controller::template::delete_template,
    ),
    components(schemas(
        controller::auth::LoginRequest,
        error::MessageResponse,
        db::project::CreateProject,
        controller::user::CreateUser,
        entity::project::Model,
        entity::user::Model,
    )),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User management endpoints"),
        (name = "projects", description = "Project management endpoints"),
        (name = "groups", description = "Group management endpoints (Not Implemented)"),
        (name = "classes", description = "Class management endpoints (Not Implemented)"),
        (name = "templates", description = "Template management endpoints (Not Implemented)"),
    )
)]
pub struct ApiDoc;

impl ApiDoc {
    pub fn openapi_spec() -> utoipa::openapi::OpenApi {
        Self::openapi()
    }
}