use std::str::FromStr;

use crate::{models::User, rocket::{request, request::*}};
use crate::rocket::Request;
use crate::rocket::http::Cookie;
use crate::DbConn;
use rocket::outcome::IntoOutcome;

enum Role {
    None,
    Normal,
    Super,
}

pub struct UserRole {
    role: Role,
}

impl UserRole {
    pub fn is_admin(&self) -> bool {
        matches!(self.role, Role::Super)
    }
    
}

impl FromStr for UserRole {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(UserRole{role: if s == "true" {Role::Super} else {Role::Normal}})
    }
}
pub struct AdminRole {
    user: UserRole
}

impl<'a, 'r> FromRequest<'a, 'r> for UserRole {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<UserRole, ()> {

        request
            .cookies()
            .get_private("user_role")
            .and_then(|cookie| cookie.value().parse().ok())
            .or_forward(())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AdminRole {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<AdminRole, ()> {
        // This will unconditionally query the database!
        let user = request.guard::<UserRole>()?;

        if user.is_admin() {
            Outcome::Success(AdminRole { user })
        } else {
            Outcome::Forward(())
        }
    }
}

//https://api.rocket.rs/v0.4/rocket/request/trait.FromRequest.html


