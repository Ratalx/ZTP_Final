

use diesel::{Insertable, Queryable};
use serde_derive::{Serialize, Deserialize};
use crate::schema::{users, fruits, orders, favourites};
use chrono::naive::NaiveDateTime;

#[derive(Queryable, Serialize, Insertable, Deserialize)]
#[table_name="users"]
pub struct User {
    pub login:  String,
    pub password: String,
    pub admin_user: bool,
}
#[derive(Insertable, Deserialize)]
#[table_name="users"]
pub struct NewUser {
    pub login:  String,
    pub password: String,
}

#[derive(Queryable, Serialize, Insertable, Deserialize)]
#[table_name = "fruits"]
pub struct Fruit {
    id: i32,
    name: String,
    in_stock: bool,
}
#[derive(Insertable,Deserialize)]
#[table_name = "fruits"]
pub struct NewFruit {
    pub name: String,
    pub in_stock: bool,
}

#[derive(Queryable, Serialize, Insertable, Deserialize)]
#[table_name = "orders"]
pub struct Order {
    id: i32,
    web_user_login: String,
    fruit_id: i32,
    order_date: NaiveDateTime,
}
#[derive(Insertable,Deserialize)]
#[table_name = "orders"]
pub struct NewOrder {
    pub web_user_login: String,
    pub fruit_id: i32,
    pub order_date: NaiveDateTime,
}
#[derive(Queryable, Serialize, Insertable, Deserialize)]
#[table_name = "favourites"]
pub struct Favourite {
    pub login: String,
    pub fruit_id: i32,
}
