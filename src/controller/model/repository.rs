use std::marker::PhantomData;
use repository_pg::{PGMediator, AbilAdmin, AbilAnon, AbilPupil, AbilTeacher};
use super::project;
use project::{Project, Comment};
use super::user::User;
mod repository_pg;

#[derive(Copy, Clone)]
pub enum UserStatus {
    Admin = 3,
    Teacher = 2,
    Pupil = 1,
    Anonymous = 0
}

pub fn ust_not_less(this: UserStatus, other: UserStatus) -> bool {
    (other as u8) <= (this as u8)
}

pub trait AnonMediator {
    fn by_year(&mut self, year: i32) -> Vec<Project>;
    
    fn by_id(&mut self, id: i32) -> Option<Project>;
    
    fn presentation(&mut self, id: i32) -> String;
    
    fn by_tags(&mut self, tags: Vec<String>, conj: bool) -> Vec<Project>;
    
    fn all_tags(&mut self) -> Vec<String>;
    
    fn max_year(&mut self) -> i32;
    
    fn user_info(&mut self, login: String) -> Option<User>;
}

pub trait PupilMediator {
    fn name(&mut self, login: &str) -> Option<String>;
    
    fn update_contacts(&mut self, login: String, contacts: String);
}

pub trait TeacherMediator {
    fn post_comment(&mut self, login: &str, content: &str, pid: i32) -> bool;
    
    fn post_project(&mut self, project: Project) -> i32;
}

pub trait AdminMediator {
    fn such_user(&mut self, login: &str, password: &str) -> Option<UserStatus>;
    
    fn such_login(&mut self, login: &str) -> bool;
    
    fn create_pupil(&mut self, login: &str, password: &str, name: &str) -> bool;
}

pub fn alloc_anon_mediator() -> Box<dyn AnonMediator>{
    Box::new(PGMediator::<AbilAnon>::get())
}

pub fn alloc_pupil_mediator() -> Box<dyn PupilMediator>{
    Box::new(PGMediator::<AbilPupil>::get())
}

pub fn alloc_teacher_mediator() -> Box<dyn TeacherMediator>{
    Box::new(PGMediator::<AbilTeacher>::get())
}

pub fn alloc_admin_mediator() -> Box<dyn AdminMediator>{
    Box::new(PGMediator::<AbilAdmin>::get())
}

pub fn init_connections() {
    repository_pg::init_connections()
}
