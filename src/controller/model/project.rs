use std::time::SystemTime;

pub struct Comment {
    pub author: String,
    pub content: String,
    pub date: SystemTime
}

impl Comment {
    pub fn new(author: &str, content: &str, date: SystemTime) -> Comment {
        Comment {
            author: String::from(author),
            content: String::from(content),
            date
        }
    }
}

pub struct Project {
    pub title: String,
    pub description: String,
    pub id: i32,
    pub year: i32,
    pub presentation: String,
    pub comments: Vec<Comment>,
    pub tags: Vec<String>,
    pub team: Vec<(String, String)>,
    pub mentor: (String, String)
}

impl Project {
    pub fn new(id: i32, year: i32, title: &str, description: &str, presentation: &str,
                comments: Vec<Comment>, tags: Vec<String>, team: Vec<(String, String)>, mentor: (String, String)) -> Project {
        Project {
            title: String::from(title),
            description: String::from(description),
            id,
            year,
            presentation: String::from(presentation),
            comments,
            tags,
            team,
            mentor
        }
    }
    
    pub fn new_s(id: i32, year: i32, title: String, description: String, presentation: String,
                 comments: Vec<Comment>, tags: Vec<String>, team: Vec<(String, String)>, mentor: (String, String)) -> Project {
        Project {
            title,
            description,
            id,
            year,
            presentation,
            comments,
            tags,
            team,
            mentor
        }
    }
}