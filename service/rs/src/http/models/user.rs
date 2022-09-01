use crate::error::{Result, Error};

use entity::user;
use entity::user::Model as UserModel;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use sea_orm::{Condition, DatabaseConnection};

pub struct User {
    id: i64,
}

impl User {
    pub async fn find(db: &DatabaseConnection, name: &str, passwd: &str) -> Result<UserModel> {
        use user::Column as Col;
        let hashed_passwd = passwd;
        let conds = Condition::all()
            .add(Col::Name.eq(name))
            .add(Col::HashedPasswd.eq(hashed_passwd));
        let user_model: Option<UserModel> = user::Entity::find().filter(conds).one(db).await?;
        user_model.ok_or(Error::RowNotFound{})
    }

    pub async fn find_by_id(id: i64) -> Result<User> {
    }
}
