use super::*;
use postgres::{Client, NoTls, IsolationLevel};
use std::time::SystemTime;
use crate::config::*;

fn connections(n: usize) -> Option<&'static mut Client> {
    static mut CONS: [Option<Client>; 4] = [None, None, None, None];
    
    unsafe {
        match n {
            0 => {
                if let None = CONS[0] {
                    let db = config_get_value("db_name").unwrap();
                    let host = config_get_value("db_address").unwrap();
                    let con_str = format!("dbname={} user=anonymous host={} password=anon666", db, host);
                    CONS[0] = Client::connect(&con_str, NoTls).ok();
                }
                CONS[0].as_mut()
            },
            1 => {
                if let None = CONS[1] {
                    let db = config_get_value("db_name").unwrap();
                    let host = config_get_value("db_address").unwrap();
                    let con_str = format!("dbname={} user=pupil host={} password=pupe666", db, host);
                    CONS[1] = Client::connect(&con_str, NoTls).ok();
                }
                CONS[1].as_mut()
            },
            2 => {
                if let None = CONS[2] {
                    let db = config_get_value("db_name").unwrap();
                    let host = config_get_value("db_address").unwrap();
                    let con_str = format!("dbname={} user=teacher host={} password=taech666", db, host);
                    CONS[2] = Client::connect(&con_str, NoTls).ok();
                }
                CONS[2].as_mut()
            },
            3 => {
                if let None = CONS[3] {
                    let db = config_get_value("db_name").unwrap();
                    let host = config_get_value("db_address").unwrap();
                    let con_str = format!("dbname={} user=admin host={} password=admin666", db, host);
                    CONS[3] = Client::connect(&con_str, NoTls).ok();
                }
                CONS[3].as_mut()
            },
            _ => None
        }
    }
}

pub fn init_connections() {
    connections(0).expect("can't establish anonymous connection");
    connections(1).expect("can't establish pupil connection");
    connections(2).expect("can't establish teacher connection");
    connections(3).expect("can't establish admin connection");
}

fn str_to_us(string: &str) -> UserStatus {
    match string {
        "Admin" => UserStatus::Admin,
        "Teacher" => UserStatus::Teacher,
        _ => UserStatus::Pupil
    }
}

pub enum AbilAnon {}
pub enum AbilPupil {}
pub enum AbilAdmin {}
pub enum AbilTeacher {}

pub struct PGMediator<T> {
    client: &'static mut Client,
    phantom: PhantomData<T>
}

impl AnonMediator for PGMediator<AbilAnon> {
    fn all_tags(&mut self) -> Vec<String> {
        let mut result = Vec::new();
        if let Ok(rows) = self.client.query(
            "select TEXT from TAGS", &[]) {
            for row in rows {
                result.push(row.get(0));
            }
        }
        result
    }
    
    fn by_year(&mut self, year: i32) -> Vec<Project> {
        let mut result = Vec::new();
        if let Ok(rows) = self.client.query(
            "select ID, TITLE, DESCRIPTION from PROJECTS where YEAR = $1", &[&year]) {
            for row in rows {
                let id = row.get(0);
                let tags = self.project_tags(id);
                result.push(Project::new(id, year, row.get(1), row.get(2), "", Vec::new(), tags, Vec::new(), (String::new(), String::new())));
            }
        }
        result
    }
    
    fn presentation(&mut self, id: i32) -> String {
        if let Ok(rows) = self.client.query(
            "select PRESENTATION from PROJECTS where ID = $1", &[&id]) {
            rows[0].get(0)
        } else {
            String::from("")
        }
    }
    
    fn by_tags(&mut self, tags: Vec<String>, conj: bool) -> Vec<Project> {
        let mut result = Vec::new();
        if conj {
            if let Ok(rows) = self.client.query(
                "select ID, TITLE, DESCRIPTION from PROJECTS where ID in 
                    (select distinct ID_PRJ from 
                        (select ID from TAGS where TEXT in (select unnest($1::text[]))) as TID
                        join
                        TAG_PRJ on TID.ID = ID_TAG)", &[&tags]) {
                for row in rows {
                    let id = row.get(0);
                    let tags = self.project_tags(id);
                    result.push(Project::new(id, 0, row.get(1), row.get(2), "", Vec::new(), tags, Vec::new(), (String::new(), String::new())));
                }
            }
        } else {
            if let Ok(rows) = self.client.query(
                "select * from SearchNoConj($1::text[])", &[&tags]) {
                for row in rows {
                    let id = row.get(0);
                    let tags = self.project_tags(id);
                    result.push(Project::new(id, 0, row.get(1), row.get(2), "", Vec::new(), tags, Vec::new(), (String::new(), String::new())));
                }
            }
        }
        result
    }
    
    fn max_year(&mut self) -> i32 {
        if let Ok(rows) = self.client.query(
            "select max(YEAR) from PROJECTS", &[]) {
            rows[0].get(0)
        } else {
            2016
        }
    }
    
    fn user_info(&mut self, login: String) -> Option<User> {
        if let Ok(rows) = self.client.query(
            "select NAME, STATUS, CONTACTS from USERS where LOGIN = $1", &[&login]) {
            if rows.len() > 0 {
                Some(User::new(str_to_us(rows[0].get(1)), login, rows[0].get(0), rows[0].get(2)))
            } else {
                None
            }
        } else {
            None
        }
    }
    
    fn by_id(&mut self, id: i32) -> Option<Project> {
        if let Ok(rows) = self.client.query(
            "select ID, YEAR, TITLE, DESCRIPTION, MENTOR, case 
                when PRESENTATION != '' then '1' 
                else '' 
                end as PRESENTATION
            from PROJECTS where ID = $1", &[&id]) {
            if rows.len() == 1 {
                let mut comments = Vec::new();
                if let Ok(rows) = self.client.query(
                    "select NAME, CONTENT, DATE from 
                        (select AUTHOR, CONTENT, DATE from COMMENTS where PROJECT = $1) as CMT
                        join 
                        (select ID, NAME from USERS) as USR
                        on CMT.AUTHOR = USR.ID
                     order by CMT.DATE desc", &[&id]) {
                    for row in rows {
                        comments.push(Comment::new(row.get(0), row.get(1), row.get(2)));
                    }
                }
                let mut team = Vec::new();
                if let Ok(rows) = self.client.query(
                    "select NAME, LOGIN from 
                        (select ID, NAME, LOGIN from USERS) as USR
                        join 
                        (select ID_USR from USR_PRJ where ID_PRJ = $1) as UP
                        on USR.ID = UP.ID_USR", &[&id]) {
                    for row in rows {
                        team.push((row.get(0), row.get(1)));
                    }
                }
                let tags = self.project_tags(id);
                let mid: i32 = rows[0].get(4);
                let mentor = {
                    let dummy = (String::new(), String::new());
                    if let Ok(rows) = self.client.query(
                        "select NAME, LOGIN from USERS where ID = $1", &[&mid]) {
                        if rows.len() > 0 {
                            (rows[0].get(0), rows[0].get(1))
                        } else {dummy}
                    } else {dummy}
                };
                Some(Project::new(rows[0].get(0), rows[0].get(1), rows[0].get(2),
                                  rows[0].get(3), rows[0].get(5), comments, tags, team, mentor))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl PGMediator<AbilAnon> {
    pub fn get() -> PGMediator<AbilAnon> {
        PGMediator {
            client: connections(0).unwrap(),
            phantom: PhantomData
        }
    }
    
    fn project_tags(&mut self, id: i32) -> Vec<String> {
        let mut tags = Vec::new();
        if let Ok(rows) = self.client.query(
            "select TEXT from 
                (select * from TAG_PRJ where ID_PRJ = $1) as TI
                join
                TAGS on TI.ID_TAG = TAGS.ID", &[&id]) {
            for row in rows {
                tags.push(row.get(0));
            }
        }
        tags
    }
}

impl PupilMediator for PGMediator<AbilPupil> {
    fn name(&mut self, login: &str) -> Option<String> {
        if let Ok(row) = self.client.query_one(
            "select NAME from PUP_USR where LOGIN = $1", &[&login]) {
            Some(row.get(0))
        } else {
            None
        }
    }
    
    fn update_contacts(&mut self, login: String, contacts: String) {
        if let Ok(_rows) = self.client.query(
            "update PUP_USR set CONTACTS = $1 where LOGIN = $2", &[&contacts, &login]) {}
    }
}

impl PGMediator<AbilPupil> {
    pub fn get() -> PGMediator<AbilPupil> {
        PGMediator {
            client: connections(1).unwrap(),
            phantom: PhantomData
        }
    }
}

impl TeacherMediator for PGMediator<AbilTeacher> {
    fn post_comment(&mut self, login: &str, content: &str, pid: i32) -> bool {
        let now = SystemTime::now();
        if let Ok(_rows) = self.client.query(
            "insert into COMMENTS(AUTHOR, PROJECT, CONTENT, DATE) 
                values((select ID from USERS_TEACHER where LOGIN = $1), $2, $3, $4)"
                , &[&login, &pid, &content, &now]) {
            true
        } else {
            false
        }
    }
    
    fn post_project(&mut self, p: Project) -> i32 {
        let mut transaction = self.client.build_transaction()
            .isolation_level(IsolationLevel::RepeatableRead)
            .start().unwrap();
        if let Ok(row) = transaction.query_one(
            "insert into PROJECTS(YEAR, TITLE, DESCRIPTION, PRESENTATION, MENTOR) 
                values($1, $2, $3, $4, (select ID from USERS_TEACHER where LOGIN = $5)) returning ID"
                , &[&p.year, &p.title, &p.description, &p.presentation, &p.mentor.1]) {
            let id = row.get(0);
            if !transaction.execute("call SetTags($1, $2::text[])", &[&id, &p.tags]).is_ok() {
                return 0;
            }
            let mut temp = Vec::new();
            for m in p.team {
                temp.push(m.1);
            }
            if !transaction.execute("call SetTeam($1, $2::text[])", &[&id, &temp]).is_ok() {
                return 0;
            }
            if !transaction.commit().is_ok() {
                return 0;
            }
            return id;
        }
        0
    }
}

impl PGMediator<AbilTeacher> {
    pub fn get() -> PGMediator<AbilTeacher> {
        PGMediator {
            client: connections(2).unwrap(),
            phantom: PhantomData
        }
    }
}

impl PGMediator<AbilAdmin> {
    pub fn get() -> PGMediator<AbilAdmin> {
        PGMediator {
            client: connections(3).unwrap(),
            phantom: PhantomData
        }
    }
}

impl AdminMediator for PGMediator<AbilAdmin> {
    fn such_user(&mut self, login: &str, password: &str) -> Option<UserStatus> {
        if let Ok(rows) = self.client.query(
            "select STATUS from USERS where login = $1 and password = $2", &[&login, &password]) {
            if 1 == rows.len() {
                return Some(str_to_us(rows[0].get(0)));
            }
        }
        None
    }
    
    fn such_login(&mut self, login: &str) -> bool {
        if let Ok(rows) = self.client.query("select * from USERS where login = $1", &[&login]) {
            1 == rows.len()
        } else {
            false
        }
    }
    
    fn create_pupil(&mut self, login: &str, password: &str, name: &str) -> bool {
        if let Ok(_rows) = self.client.query(
            "insert into USERS(login, password, name, status, contacts) values($1, $2, $3, 'Pupil', '')", &[&login, &password, &name]) {
            true
        } else {
            false
        }
    }
}
