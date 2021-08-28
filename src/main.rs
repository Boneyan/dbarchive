#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket_contrib::templates;
use rocket_contrib::serve::StaticFiles;

mod config;

mod controller;
use controller::*;
use controller::model::repository;

fn main() {
    config::config_load("config");
    repository::init_connections();
    
    rocket::ignite()
        .register(catchers![not_found])
        .mount("/", routes![index, login, logout, reg_page, register, crt_page, srch_page, search, cabinet, upd_contacts,
                            year_page, project_page, p404, admin, new_comment, new_project, presentation, presentation_info])
        .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
        .attach(templates::Template::fairing())
        .launch();
}