#![feature(proc_macro_hygiene, decl_macro)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;

use dsl::any;
use rocket::http::{hyper::h1::Http11Message, Status};
use rocket::http::{Cookie, Cookies};
use rocket_contrib::databases::diesel::*;
use rocket_contrib::json::Json;

#[database("postgres")]
struct DbConn(diesel::PgConnection);

mod models;
mod roles;
mod schema;

use models::*;
use roles::*;
use schema::{favourites, fruits, orders, users};

#[get("/")]
fn welcome_page() -> &'static str {
    "Welcome Page"
}

#[get("/login")]
fn login_page() -> &'static str {
    "LoginPage"
}

#[get("/logout")]
fn logout_page(mut cookies: Cookies) -> Status {
    cookies.remove_private(Cookie::named("login"));
    cookies.remove_private(Cookie::named("user_role"));

    Status::Ok
}

#[post("/login", data = "<user_cred>")]
fn login_page_post(conn: DbConn, mut cookies: Cookies, user_cred: Json<NewUser>) -> Status {
    let user: Result<User, diesel::result::Error> = users::table
        .filter(users::columns::login.eq(&user_cred.0.login))
        .first(&*conn);

    match user {
        Ok(user) => {
            if user.login.eq(&user_cred.0.login) && user.password.eq(&user_cred.0.password) {
                cookies.add_private(Cookie::new(
                    "user_role",
                    if user.admin_user { "true" } else { "false" },
                ));
                cookies.add_private(Cookie::new("login", user.login));

                return Status::Ok;
            }
            Status::BadRequest
        }
        Err(_) => Status::BadRequest,
    }
}

#[get("/register")]
fn register_page() -> &'static str {
    "RegistrationPage"
}

#[post("/register", data = "<new_user>")]
fn register_post_page(conn: DbConn, new_user: Json<User>) -> Status {
    let result: Result<User, diesel::result::Error> = diesel::insert_into(users::table)
        .values(&new_user.0)
        .get_result(&*conn);

    match result {
        Ok(_) => Status::Ok,
        Err(_) => Status::BadRequest,
    }
}

#[get("/fruitDashboard")]
fn fruit_dashboard_page(conn: DbConn, _user_role: UserRole) -> Json<Vec<Fruit>> {
    let fruits_list = fruits::table.load::<Fruit>(&*conn).unwrap();
    Json(fruits_list)
}

#[post("/fruitDashboard", data = "<new_fruit>")]
fn add_fruit(conn: DbConn, new_fruit: Json<NewFruit>, _user_role: AdminRole) -> Json<Fruit> {
    let result = diesel::insert_into(fruits::table)
        .values(&new_fruit.0)
        .get_result(&*conn)
        .unwrap();
    Json(result)
}

#[get("/fruitDashboard/<id>")]
fn fruit_item_page(conn: DbConn, id: i32, _user_role: UserRole) -> Json<Fruit> {
    let fruit = fruits::table
        .filter(fruits::columns::id.eq(id))
        .first(&*conn)
        .unwrap();
    Json(fruit)
}

#[put("/fruitDashboard/<id>", data = "<new_fruit>")]
fn fruit_update(
    conn: DbConn,
    id: i32,
    new_fruit: Json<NewFruit>,
    _user_role: AdminRole,
) -> Json<Fruit> {
    let result = diesel::update(fruits::table.filter(fruits::columns::id.eq(id)))
        .set((
            fruits::columns::name.eq(new_fruit.0.name),
            fruits::columns::in_stock.eq(new_fruit.0.in_stock),
        ))
        .get_result(&*conn)
        .unwrap();

    Json(result)
}

#[get("/favouriteDashboard")]
fn favourite_dashboard_page(
    conn: DbConn,
    _user_role: UserRole,
    mut cookie: Cookies,
) -> Json<Vec<Fruit>> {
    let user_login = cookie.get_private("login").unwrap().value().to_string();

    let users_favourite = favourites::table
        .filter(favourites::columns::login.eq(user_login))
        .select(favourites::columns::fruit_id)
        .load::<i32>(&*conn)
        .unwrap();

    let fruits = fruits::table
        .filter(fruits::columns::id.eq(any(users_favourite)))
        .load::<Fruit>(&*conn)
        .unwrap();

    Json(fruits)
}

#[get("/favouriteDashboard/<id>")]
fn fevourite_item_page(conn: DbConn, id: i32, _user_role: UserRole) -> Json<Fruit> {
    let fruit = fruits::table
        .filter(fruits::columns::id.eq(id))
        .first(&*conn)
        .unwrap();
    Json(fruit)
}

#[post("/favouriteDashboard/<id>")]
fn fevourite_add_item_page(
    conn: DbConn,
    id: i32,
    _user_role: UserRole,
    mut cookies: Cookies,
) -> Json<Favourite> {
    let user_login = cookies.get_private("login").unwrap().value().to_string();

    let new_favourite = Favourite {
        login: user_login,
        fruit_id: id,
    };

    let result = diesel::insert_into(favourites::table)
        .values(&new_favourite)
        .get_result(&*conn)
        .unwrap();

    Json(result)
}

#[get("/ordersDashboard")]
fn orders_dashboard_page(conn: DbConn, _user_role: AdminRole) -> Json<Vec<Order>> {
    let orders_list = orders::table.load::<Order>(&*conn).unwrap();
    Json(orders_list)
}

#[post("/ordersDashboard/<fruit_id>")]
fn add_order(
    conn: DbConn,
    _user_role: UserRole,
    mut cookies: Cookies,
    fruit_id: i32,
) -> Json<Order> {
    let current_time = chrono::Utc::now().naive_utc();
    let user_login = cookies.get_private("login").unwrap().value().to_string();

    let new_order: NewOrder = NewOrder {
        fruit_id,
        web_user_login: user_login,
        order_date: current_time,
    };
    let result = diesel::insert_into(orders::table)
        .values(&new_order)
        .get_result(&*conn)
        .unwrap();
    Json(result)
}

#[get("/ordersDashboard/<id>")]
fn order_item_page(conn: DbConn, id: i32, _user_role: AdminRole) -> Json<Order> {
    let order = orders::table
        .filter(orders::columns::id.eq(id))
        .first(&*conn)
        .unwrap();
    Json(order)
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                welcome_page,
                login_page,
                login_page_post,
                register_page,
                register_post_page,
                fruit_dashboard_page,
                fruit_item_page,
                add_fruit,
                fruit_update,
                favourite_dashboard_page,
                fevourite_item_page,
                fevourite_add_item_page,
                orders_dashboard_page,
                order_item_page,
                add_order,
                logout_page,
            ],
        )
        .attach(DbConn::fairing())
        .launch();
}
