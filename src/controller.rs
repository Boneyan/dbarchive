use rocket::response::{Redirect, Response};
use rocket::http::{RawStr, Cookie, Cookies, Status, ContentType};
use rocket::Request;
use rocket::request::Form;
use rocket_contrib::json::Json;
use super::templates::*;

pub mod model;
use model::user::user_cache;
use model::project::Project;
mod dto;
use dto::*;
mod controller_utils;
use controller_utils::*;

extern crate base64;
use base64::decode;


#[catch(404)]
pub fn not_found(_req: &Request) -> Redirect {
    Redirect::to("/404")
}

#[get("/404")]
pub fn p404(mut cookies: Cookies) -> Template {
    let mut who = get_medium(&mut cookies);
    
    let context = simple_context(&mut who);
    
    return_medium(who);
    Template::render("404", &context)
}

#[get("/")]
pub fn index(mut cookies: Cookies) -> Template {
    let mut who = get_medium(&mut cookies);
    
    let header = prepare_header(&mut who);
    let context = IndexDTO::new(who.max_year(), header);
    
    return_medium(who);
    
    Template::render("index", &context)
}

#[get("/register")]
pub fn reg_page(mut cookies: Cookies) -> Result<Template, Redirect> {
    let mut who = get_medium(&mut cookies);
    
    if who.authorized() {
        return_medium(who);
        Err(Redirect::to("/"))
    } else {
        let context = simple_context(&mut who);
        return_medium(who);
        Ok(Template::render("register", &context))
    }
}

#[get("/api/logout")]
pub fn logout(mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("login"));
    cookies.remove_private(Cookie::named("password"));
    
    Redirect::to("/")
}

#[get("/api/auth?<login>&<password>")]
pub fn login(login: &RawStr, password: &RawStr, mut cookies: Cookies) -> Status {
    if user_cache().login(login.as_str(), password.as_str()) {
        cookies.add_private(Cookie::new("login", login.as_str().to_string()));
        cookies.add_private(Cookie::new("password", password.as_str().to_string()));
    }
    
    Status::Accepted
}

#[post("/api/register", data="<dto>")]
pub fn register(dto: Form<RegisterDTO>, mut cookies: Cookies) -> Redirect {
    let u_c = user_cache();
    let login = dto.login.as_str();
    let pass = dto.password.as_str();
    if u_c.register(login, pass, dto.name.as_str()) {
        u_c.login(login, pass);
        cookies.add_private(Cookie::new("login", dto.login.clone()));
        cookies.add_private(Cookie::new("password", dto.password.clone()));
    }
    
    Redirect::to("/")
}

#[get("/create_project")]
pub fn crt_page(mut cookies: Cookies) -> Result<Template, Redirect> {
    let mut who = get_medium(&mut cookies);
    
    if !who.is_teacher() {
        return_medium(who);
        Err(Redirect::to("/"))
    } else {
        let header = prepare_header(&mut who);
        let tags = who.all_tags();
        let context = UploadPageDTO::new(tags, header);
        return_medium(who);
        Ok(Template::render("upload", &context))
    }
}

#[get("/year/<year>")]
pub fn year_page(year: i32, mut cookies: Cookies) -> Template {
    let mut who = get_medium(&mut cookies);
    
    let header = prepare_header(&mut who);
    let prjs = who.by_year(year);
    let context = YearDTO::new(year, ProjectCardDTO::from_project(prjs), header);
    
    return_medium(who);
    Template::render("year", &context)
}

#[get("/project/<id>")]
pub fn project_page(id: i32, mut cookies: Cookies) -> Result<Template, Redirect> {
    let mut who = get_medium(&mut cookies);
    
    if let Some(prj) = who.by_id(id) {
        let header = prepare_header(&mut who);
        let context = ProjectDTO::from_project(prj, header);
        
        return_medium(who);
        Ok(Template::render("project", &context))
    } else {
        return_medium(who);
        Err(Redirect::to("/404"))
    }
}

#[get("/adminstuff")]
pub fn admin(mut cookies: Cookies) -> Result<Template, Redirect> {
    let mut who = get_medium(&mut cookies);
    
    if !who.is_admin() {
        return_medium(who);
        Err(Redirect::to("/"))
    } else {
        let context = simple_context(&mut who);
        return_medium(who);
        Ok(Template::render("admin", &context))
    }
}


#[post("/api/comment/new", format = "json", data = "<dto>")]
pub fn new_comment(mut cookies: Cookies, dto: Json<AcceptCommentDTO>) -> Status {
    let mut who = get_medium(&mut cookies);
    
    if !who.is_teacher() {
        return_medium(who);
        Status::Forbidden
    } else {
        who.post_comment(dto.0.pid, dto.0.content.as_str());
        return_medium(who);
        Status::Accepted
    }
}


#[post("/api/project/new", format = "json", data = "<dto>")]
pub fn new_project(mut cookies: Cookies, dto: Json<AcceptProjectDTO>) -> Result<Json<PCreatedDTO>, Status> {
    let mut who = get_medium(&mut cookies);
    
    if !who.is_teacher() {
        return_medium(who);
        Err(Status::Forbidden)
    } else {
        let mut team = Vec::new();
        for m in dto.0.team {
            team.push((String::from(""), m));
        }
        let prj = Project::new_s(0, dto.0.year, dto.0.title, dto.0.description,
                                 dto.0.presentation, Vec::new(), dto.0.tags, team, (String::from(""), dto.0.mentor));
        let id = who.post_project(prj);
        return_medium(who);
        Ok(Json(PCreatedDTO::new(id)))
    }
}

#[get("/search")]
pub fn srch_page(mut cookies: Cookies) -> Template {
    let mut who = get_medium(&mut cookies);
    
    let header = prepare_header(&mut who);
    let tags = who.all_tags();
    let context = SearchPageDTO::new(tags, header);
    
    return_medium(who);
    Template::render("search", &context)
}

#[head("/api/presentation/<_id>")]
pub fn presentation_info(_id: i32) -> Response<'static> {
    Response::build().header(ContentType::from_extension("pdf").unwrap()).finalize()
}

#[get("/api/presentation/<id>")]
pub fn presentation(mut cookies: Cookies, id: i32) -> Vec<u8> {
    let mut who = get_medium(&mut cookies);
    
    let p = who.presentation(id);
    
    return_medium(who);
    decode(p).unwrap()
}

#[post("/api/search", format = "json", data = "<dto>")]
pub fn search(mut cookies: Cookies, dto: Json<SearchInDTO>) -> Json<SearchOutDTO> {
    let mut who = get_medium(&mut cookies);
    
    let projects = who.search_by_tags(dto.0.tags, dto.0.conj);
    
    return_medium(who);
    Json(SearchOutDTO::new(ProjectCardDTO::from_project(projects)))
}


#[post("/api/update_contacts", format = "json", data = "<dto>")]
pub fn upd_contacts(mut cookies: Cookies, dto: Json<ContactsDTO>) -> Status {
    let mut who = get_medium(&mut cookies);
    
    if who.login == dto.0.login {
        let succ = who.update_contacts(dto.0.contacts);
        return_medium(who);
        if succ {
            Status::Accepted
        } else {
            Status::Forbidden
        }
    } else {
        return_medium(who);
        Status::Forbidden
    }
}

#[get("/cabinet/<login>")]
pub fn cabinet(login: String, mut cookies: Cookies) -> Result<Template, Redirect> {
    let mut who = get_medium(&mut cookies);
    
    let header = prepare_header(&mut who);
    if let Some(user) = who.user_info(login) {
        let context = CabinetDTO::from_user(user, header);
        return_medium(who);
        Ok(Template::render("cabinet", &context))
    } else {
        return_medium(who);
        Err(Redirect::to("/404"))
    }
}
