use rocket::http::Cookies;
use super::user_cache;
use super::model::user::UserMe;
use super::{HeaderDTO, SimpleDTO};

/*pub fn authorized(cookies: &mut Cookies) -> bool {
    let login = cookies.get_private("login");
    if let Some(lc) = login {
        let passw = cookies.get_private("password");
        if let Some(pc) = passw {
            return user_cache().validate(lc.value(), pc.value());
        }
    }
    false
}

pub fn authorized_teacher(cookies: &mut Cookies) -> bool {
    let login = cookies.get_private("login");
    if let Some(lc) = login {
        let passw = cookies.get_private("password");
        if let Some(pc) = passw {
            return user_cache().validate_teacher(lc.value(), pc.value());
        }
    }
    false
}

pub fn authorized_admin(cookies: &mut Cookies) -> bool {
    let login = cookies.get_private("login");
    if let Some(lc) = login {
        let passw = cookies.get_private("password");
        if let Some(pc) = passw {
            return user_cache().validate_admin(lc.value(), pc.value());
        }
    }
    false
}*/

pub fn get_medium(cookies: &mut Cookies) -> UserMe {
    let login = cookies.get_private("login");
    if let Some(lc) = login {
        let passw = cookies.get_private("password");
        if let Some(pc) = passw {
            if let Some(u) = user_cache().user(lc.value(), pc.value()) {
                return u;
            }
        }
    }
    user_cache().anonymous()
}

pub fn return_medium(usr: UserMe) {
    user_cache().back(usr);
}

pub fn prepare_header(who: &mut UserMe) -> HeaderDTO {
    let name = 
        if let Some(name) = who.name() 
            {name} 
        else 
            {String::new()};
    HeaderDTO::new(who.status as u8, name, who.login.clone())
}

pub fn simple_context(who: &mut UserMe) -> SimpleDTO {
    SimpleDTO::new(prepare_header(who))
}
