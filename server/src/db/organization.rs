use log::debug;
use sqlx::{ query_as, query_scalar };
use uuid::Uuid;

use crate::{ db::user, state::AppState };

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
}

pub async fn get_orgs_by_user_id(
    user_id: &Uuid,
    ctx: &AppState
) -> Result<Vec<Organization>, sqlx::Error> {
    match
        query_as!(
            Organization,
            r#"SELECT organizations.id, organizations.name FROM organizations JOIN user_organizations ON organizations.id = user_organizations.organization_id WHERE user_organizations.user_id = $1"#,
            user_id
        ).fetch_all(&ctx.db).await
    {
        Ok(organizations) => Ok(organizations),
        Err(e) => {
            return Err(e);
        }
    }
}

pub struct CreateOrganization {
    pub name: String,
    pub user_id: Uuid,
}

pub async fn create_organization(
    org: CreateOrganization,
    ctx: &AppState
) -> Result<Uuid, sqlx::Error> {
    let organization_id: Uuid = match
        query_scalar!(
            r#"INSERT INTO organizations (name, owner_user_id) VALUES ($1, $2) RETURNING id"#,
            org.name,
            org.user_id
        ).fetch_one(&ctx.db).await
    {
        Ok(id) => id,
        Err(e) => {
            debug!("Failed to create organization: {:?}", e);
            return Err(e);
        }
    };

    attach_user_to_organization(&org.user_id, &organization_id, ctx).await?;

    Ok(organization_id)
}

pub async fn attach_user_to_organization(
    user_id: &Uuid,
    organization_id: &Uuid,
    ctx: &AppState
) -> Result<(), sqlx::Error> {
    match
        query_scalar!(
            r#"INSERT INTO user_organizations (user_id, organization_id) VALUES ($1, $2) RETURNING (user_id, organization_id)"#,
            user_id,
            organization_id
        ).fetch_one(&ctx.db).await
    {
        Ok(_) => Ok(()),
        Err(e) => {
            debug!("Failed to create organization xxxx: {:?}", e);
            return Err(e);
        }
    }
}
