use super::repository::*;
use super::project::Project;

use crate::config::*;
use std::collections::HashMap;
use UserStatus::*;

pub struct User {
    pub status: UserStatus,
    pub login: String,
    pub name: String,
    pub contacts: String
}

impl User {
    pub fn new(status: UserStatus, login: String, name: String, contacts: String) -> User {
        User {
            status,
            login,
            name,
            contacts
        }
    }
}

pub struct UserMe {
    pub status: UserStatus,
    pub login: String,
    mediator_n: Box<dyn AnonMediator>,
    mediator_p: Option<Box<dyn PupilMediator>>,
    mediator_t: Option<Box<dyn TeacherMediator>>,
    mediator_a: Option<Box<dyn AdminMediator>>,
}

impl UserMe {
    fn new(status: UserStatus, login: &str) -> UserMe {
        let mediator_n = alloc_anon_mediator();
        let mut mediator_p: Option<Box<dyn PupilMediator>> = None;
        let mut mediator_t: Option<Box<dyn TeacherMediator>> = None;
        let mut mediator_a: Option<Box<dyn AdminMediator>> = None;
        match status {
            Pupil => {
                mediator_p = Some(alloc_pupil_mediator());
            },
            Teacher => {
                mediator_p = Some(alloc_pupil_mediator());
                mediator_t = Some(alloc_teacher_mediator());
            },
            Admin => {
                mediator_p = Some(alloc_pupil_mediator());
                mediator_t = Some(alloc_teacher_mediator());
                mediator_a = Some(alloc_admin_mediator());
            },
            _ => ()
        }
        
        UserMe {
            status,
            login: String::from(login),
            mediator_n,
            mediator_p,
            mediator_t,
            mediator_a
        }
    }
    
    pub fn authorized(&self) -> bool {
        if let Anonymous = self.status {
            false
        } else {
            true
        }
    }
    
    pub fn by_year(&mut self, year: i32) -> Vec<Project> {
        self.mediator_n.by_year(year)
    }
    
    pub fn name(&mut self) -> Option<String> {
        if let Some(med) = self.mediator_p.as_mut() {
            med.name(self.login.as_str())
        } else {
            None
        }
    }
    
    pub fn max_year(&mut self) -> i32 {
        self.mediator_n.max_year()
    }
    
    pub fn by_id(&mut self, id: i32) -> Option<Project> {
        self.mediator_n.by_id(id)
    }
    
    pub fn is_admin(&mut self) -> bool {
        ust_not_less(self.status, Admin)
    }
    
    pub fn is_teacher(&mut self) -> bool {
        ust_not_less(self.status, Teacher)
    }
    
    pub fn post_comment(&mut self, pid: i32, content: &str) {
        if let Some(med) = self.mediator_t.as_mut() {
            med.post_comment(self.login.as_str(), content, pid);
        }
    }
    
    pub fn post_project(&mut self, prj: Project) -> i32 {
        if let Some(med) = self.mediator_t.as_mut() {
            med.post_project(prj)
        } else {
            0
        }
    }
    
    pub fn search_by_tags(&mut self, tags: Vec<String>, conj: bool) -> Vec<Project> {
        self.mediator_n.by_tags(tags, conj)
    }
    
    pub fn all_tags(&mut self) -> Vec<String> {
        self.mediator_n.all_tags()
    }
    
    pub fn presentation(&mut self, id: i32) -> String {
        self.mediator_n.presentation(id)
    }
    
    pub fn user_info(&mut self, login: String) -> Option<User> {
        self.mediator_n.user_info(login)
    }
    
    pub fn update_contacts(&mut self, contacts: String) -> bool {
        if let Some(med) = self.mediator_p.as_mut() {
            med.update_contacts(self.login.clone(), contacts);
            true
        } else {
            false
        }
    }
}

pub struct UserCache {
    cap: usize,
    users: HashMap<String, UserMe>,
    //anonymous: UserMe,
    mediator_a: Box<dyn AdminMediator>,
    unq_check: bool
}

impl UserCache {
    fn init() -> UserCache {
        let cap = config_get("ucache_cap").unwrap();
        UserCache {
            cap,
            users: HashMap::with_capacity(cap),
            //anonymous: UserMe::new(UserStatus::Anonymous, ""),
            mediator_a: alloc_admin_mediator(),
            unq_check: config_get("db_unique").unwrap()
        }
    }
    
    pub fn login(&mut self, login: &str, password: &str) -> bool {
        if let Some(st) = self.mediator_a.such_user(login, password) {
            if self.users.len() == self.cap {
                let key = self.users.keys().next().unwrap().clone();
                self.users.remove(&key);
            }
            //let key = format!("{}{}", login, password);
            //self.users.insert(key, UserMe::new(st, login));
            self.users.insert(login.to_string(), UserMe::new(st, login));
            true
        } else {
            false
        }
    }
    
    pub fn user(&mut self, login: &str, password: &str) -> Option<UserMe> {
        //let key = format!("{}{}", login, password);
        //if let Some(_user) = self.users.get(&key) {
        if let Some(_user) = self.users.get(login) {
            //self.users.remove(&key)
            self.users.remove(login)
            //Some(user.clone())
        } else {
            self.login(login, password);
            //if let Some(_user) = self.users.get(&key) {
            if let Some(_user) = self.users.get(login) {
                //Some(user.clone())
                //self.users.remove(&key)
                self.users.remove(login)
            } else {
                None
            }
        }
    }
    
    pub fn back(&mut self, usr: UserMe) {
        //let key = format!("{}{}", login, password);
        //self.users.insert(key, usr);
        self.users.insert(usr.login.to_string(), usr);
    }
    
    /*pub fn validate(&mut self, login: &str, password: &str) -> bool {
        //if !self.users.contains_key(&format!("{}{}", login, password)) {
        if !self.users.contains_key(login) {
            self.login(login, password)
        } else {
            true
        }
    }
    
    fn validate_bound(&mut self, login: &str, password: &str, bound: UserStatus) -> bool {
        //let key = format!("{}{}", login, password);
        //if let Some(user) = self.users.get(&key) {
        if let Some(user) = self.users.get(login) {
            ust_not_less(user.status, bound)
        } else {
            self.login(login, password);
            //if let Some(user) = self.users.get(&key) {
            if let Some(user) = self.users.get(login) {
                ust_not_less(user.status, bound)
            } else {
                false
            }
        }
    }
    
    pub fn validate_teacher(&mut self, login: &str, password: &str) -> bool {
        self.validate_bound(login, password, Teacher)
    }
    
    pub fn validate_admin(&mut self, login: &str, password: &str) -> bool {
        self.validate_bound(login, password, Admin)
    }*/
    
    pub fn register(&mut self, login: &str, password: &str, name: &str) -> bool {
        if self.unq_check || !self.mediator_a.such_login(login) {
            return self.mediator_a.create_pupil(login, password, name);
        }
        false
    }
    
    pub fn anonymous(&mut self) -> UserMe {
        //self.anonymous.clone()
        UserMe::new(UserStatus::Anonymous, "")
    }
}

pub fn user_cache() -> &'static mut UserCache {
    static mut CACHE: Option<UserCache> = None;
    unsafe {
        if let None = CACHE {
            CACHE = Some(UserCache::init());
        }
        CACHE.as_mut().unwrap()
    }
}
