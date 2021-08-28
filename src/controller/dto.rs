use serde::ser::{Serialize, SerializeStruct, Serializer};
use super::model::project::{Project, Comment};
use super::model::user::User;
use std::time::SystemTime;
use serde::Deserialize;


pub struct HeaderDTO {
    rql: u8,
    name: String,
    login: String
}

impl HeaderDTO {
    pub fn new(rql: u8, name: String, login: String) -> HeaderDTO {
        HeaderDTO {
            rql,
            name,
            login
        }
    }
}

impl Serialize for HeaderDTO {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Header", 3)?;
        s.serialize_field("rql", &self.rql)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("login", &self.login)?;
        s.end()
    }
}


pub struct SimpleDTO {
    header: HeaderDTO
}

impl SimpleDTO {
    pub fn new(header: HeaderDTO) -> SimpleDTO {
        SimpleDTO {
            header
        }
    }
}

impl Serialize for SimpleDTO {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Header", 1)?;
        s.serialize_field("header", &self.header)?;
        s.end()
    }
}


pub struct IndexDTO {
    maxyear: i32,
    header: HeaderDTO
}

impl IndexDTO {
    pub fn new(maxyear: i32, header: HeaderDTO) -> IndexDTO {
        IndexDTO {
            maxyear,
            header
        }
    }
}

impl Serialize for IndexDTO {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Index", 2)?;
        s.serialize_field("maxyear", &self.maxyear)?;
        s.serialize_field("header", &self.header)?;
        s.end()
    }
}


pub struct CommentDTO {
    author: String,
    content: String,
    date: SystemTime
}

impl CommentDTO {
    pub fn from_comments(cmts: Vec<Comment>) -> Vec<CommentDTO> {
        let mut result = Vec::new();
        for cmt in cmts {
            result.push(CommentDTO {
                author: cmt.author,
                content: cmt.content,
                date: cmt.date
            });
        }
        result
    }
}

impl Serialize for CommentDTO {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Comment", 3)?;
        s.serialize_field("author", &self.author)?;
        s.serialize_field("content", &self.content)?;
        s.serialize_field("date", &self.date)?;
        s.end()
    }
}


pub struct ProjectDTO {
    title: String,
    description: String,
    comments: Vec<CommentDTO>,
    tags: Vec<String>,
    id: i32,
    year: i32,
    presentation: bool,
    team: Vec<(String, String)>,
    mentor: (String, String),
    header: HeaderDTO
}

impl ProjectDTO {
    pub fn from_project(prj: Project, header: HeaderDTO) -> ProjectDTO {
        let presentation = {if prj.presentation.len() == 0 {false} else {true}};
        ProjectDTO {
            title: prj.title,
            description: prj.description,
            comments: CommentDTO::from_comments(prj.comments),
            tags: prj.tags,
            id: prj.id,
            year: prj.year,
            presentation,
            team: prj.team,
            mentor: prj.mentor,
            header
        }
    }
}

impl Serialize for ProjectDTO {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Project", 10)?;
        s.serialize_field("title", &self.title)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("comments", &self.comments)?;
        s.serialize_field("tags", &self.tags)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("year", &self.year)?;
        s.serialize_field("presentation", &self.presentation)?;
        s.serialize_field("team", &self.team)?;
        s.serialize_field("mentor", &self.mentor)?;
        s.serialize_field("header", &self.header)?;
        s.end()
    }
}


pub struct ProjectCardDTO {
    title: String,
    short: String,
    tags: Vec<String>,
    id: i32
}

impl ProjectCardDTO {
    pub fn from_project(prjs: Vec<Project>) -> Vec<ProjectCardDTO> {
        let mut result = Vec::new();
        for p in prjs {
            let title = p.title;
            let mut s_end = p.description.len();
            if s_end > 100 {s_end = 100;}
            let short = String::from(&p.description[..s_end]);
            let id = p.id;
            result.push(ProjectCardDTO {
                title,
                short,
                tags: p.tags,
                id
            });
        }
        result
    }
}

impl Serialize for ProjectCardDTO {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ProjectCard", 4)?;
        s.serialize_field("title", &self.title)?;
        s.serialize_field("short", &self.short)?;
        s.serialize_field("tags", &self.tags)?;
        s.serialize_field("id", &self.id)?;
        s.end()
    }
}

pub struct YearDTO {
    year: i32,
    projects: Vec<ProjectCardDTO>,
    header: HeaderDTO
}

impl Serialize for YearDTO {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Year", 3)?;
        s.serialize_field("year", &self.year)?;
        s.serialize_field("projects", &self.projects)?;
        s.serialize_field("header", &self.header)?;
        s.end()
    }
}

impl YearDTO {
    pub fn new(year: i32, projects: Vec<ProjectCardDTO>, header: HeaderDTO) -> YearDTO {
        YearDTO {
            year,
            projects,
            header
        }
    }
}

pub struct SearchPageDTO {
    tags: Vec<String>,
    header: HeaderDTO
}

impl Serialize for SearchPageDTO {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("SearchPage", 2)?;
        s.serialize_field("tags", &self.tags)?;
        s.serialize_field("header", &self.header)?;
        s.end()
    }
}

impl SearchPageDTO {
    pub fn new(tags: Vec<String>, header: HeaderDTO) -> SearchPageDTO {
        SearchPageDTO {
            tags,
            header
        }
    }
}


pub struct UploadPageDTO {
    tags: Vec<String>,
    header: HeaderDTO
}

impl Serialize for UploadPageDTO {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("UploadPage", 2)?;
        s.serialize_field("tags", &self.tags)?;
        s.serialize_field("header", &self.header)?;
        s.end()
    }
}

impl UploadPageDTO {
    pub fn new(tags: Vec<String>, header: HeaderDTO) -> UploadPageDTO {
        UploadPageDTO {
            tags,
            header
        }
    }
}


pub struct CabinetDTO {
    login: String,
    name: String,
    rql: u8,
    contact: String,
    header: HeaderDTO
}

impl Serialize for CabinetDTO {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Cabinet", 5)?;
        s.serialize_field("login", &self.login)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("rql", &self.rql)?;
        s.serialize_field("contact", &self.contact)?;
        s.serialize_field("header", &self.header)?;
        s.end()
    }
}

impl CabinetDTO {
    pub fn from_user(user: User, header: HeaderDTO) -> CabinetDTO {
        CabinetDTO {
            login: user.login,
            name: user.name,
            rql: user.status as u8,
            contact: user.contacts,
            header
        }
    }
}


#[derive(Deserialize)]
pub struct AcceptCommentDTO {
    pub content: String,
    pub pid: i32
}

#[derive(Deserialize)]
pub struct AcceptProjectDTO {
    pub year: i32,
    pub title: String,
    pub tags: Vec<String>,
    pub description: String,
    pub presentation: String,
    pub team: Vec<String>,
    pub mentor: String
}

#[derive(FromForm)]
pub struct RegisterDTO {
    pub name: String,
    pub login: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct SearchInDTO {
    pub tags: Vec<String>,
    pub conj: bool
}

#[derive(Deserialize)]
pub struct ContactsDTO {
    pub login: String,
    pub contacts: String
}

pub struct SearchOutDTO {
    pub projects: Vec<ProjectCardDTO>
}

impl SearchOutDTO {
    pub fn new(projects: Vec<ProjectCardDTO>) -> SearchOutDTO {
        SearchOutDTO {
            projects
        }
    }
}

impl Serialize for SearchOutDTO {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("SearchOut", 1)?;
        s.serialize_field("projects", &self.projects)?;
        s.end()
    }
}

pub struct PCreatedDTO {
    pub id: i32
}

impl PCreatedDTO {
    pub fn new(id: i32) -> PCreatedDTO {
        PCreatedDTO {
            id
        }
    }
}

impl Serialize for PCreatedDTO {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("SearchOut", 1)?;
        s.serialize_field("id", &self.id)?;
        s.end()
    }
}