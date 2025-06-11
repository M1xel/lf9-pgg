use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Project::Table)
                    .if_not_exists()
                    .col(pk_uuid(Project::Id).extra("DEFAULT gen_random_uuid()"))
                    .col(string(Project::Name))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Group::Table)
                    .if_not_exists()
                    .primary_key(Index::create().col(Group::Id).col(Group::ProjectId))
                    .col(uuid(Group::Id).extra("DEFAULT gen_random_uuid()"))
                    .col(uuid(Group::ProjectId))
                    .col(string(Group::Name))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-project-id")
                            .from(Group::Table, Group::ProjectId)
                            .to(Project::Table, Project::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_uuid(User::Id).extra("DEFAULT gen_random_uuid()"))
                    .col(string_uniq(User::Username))
                    .col(string(User::Name))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserGroupProject::Table)
                    .if_not_exists()
                    .col(uuid(UserGroupProject::UserId))
                    .col(uuid(UserGroupProject::GroupId))
                    .col(uuid(UserGroupProject::ProjectId))
                    .primary_key(
                        Index::create()
                            .col(UserGroupProject::UserId)
                            .col(UserGroupProject::GroupId)
                            .col(UserGroupProject::ProjectId),
                    )
                    .index(
                        Index::create()
                            .col(UserGroupProject::UserId)
                            .col(UserGroupProject::ProjectId)
                            .unique(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-user-id")
                            .from(UserGroupProject::Table, UserGroupProject::UserId)
                            .to(User::Table, User::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-project-group-id")
                            .from(
                                UserGroupProject::Table,
                                (UserGroupProject::GroupId, UserGroupProject::ProjectId),
                            )
                            .to(Group::Table, (Group::Id, Group::ProjectId))
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(LocalAuth::Table)
                    .if_not_exists()
                    .col(pk_uuid(LocalAuth::Id))
                    .col(string(LocalAuth::Hash))
                    .col(boolean(LocalAuth::PasswordChangeRequired).default(true))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-localauth-user")
                            .from(LocalAuth::Table, LocalAuth::Id)
                            .to(User::Table, User::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Project::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Group::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(UserGroupProject::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(LocalAuth::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Project {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum Group {
    Table,
    Id,
    ProjectId,
    Name,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Username,
    Name,
}

#[derive(DeriveIden)]
enum UserGroupProject {
    Table,
    UserId,
    GroupId,
    ProjectId,
}

#[derive(DeriveIden)]
enum LocalAuth {
    Table,
    Id,
    Hash,
    PasswordChangeRequired,
}
