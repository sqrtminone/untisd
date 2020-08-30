use serde::Deserialize;
use anyhow::{Result,anyhow};

//TODO move to correct crates
use crate::untis::RpcRespone;

//representation of Room like retrived via getRooms
#[derive(Deserialize)]
pub struct Room {
    pub id: u16, //TODO can this be bigger :o ?
    pub name: String,
    pub longName: String,
    //TODO apparently this can go missing?
    pub foreColor: Option<u32>,
    //TODO apparently this can go missing?
    pub backColor: Option<u32>,
}

pub struct Rooms {
    pub room: Vec<Room>,
    //age: time_fet_info_sth TODO,
}

impl Rooms {
    pub fn from_json(text: String) -> Result<Self> {
        //TODO don't do this ugly inline parsing but rather use impl
        //and traits. So that the response function foo can check for a
        //trait which guarantees from_json() to be present and directly
        //uses that instead of having to shift the parsing and serializing here
        let res: RpcRespone<Vec<Room>> = serde_json::from_str(&text)?;
        Ok(Rooms {
            room: res.result,
        })
    }
}

//representation of Subjects like retrived via getSubjects
#[derive(Deserialize)]
pub struct Subject {
    pub id: u16, //TODO can this be bigger :o ?
    pub name: String,
    pub longName: String,
    //TODO apparently this can go missing?
    pub foreColor: Option<String>,
    //TODO apparently this can go missing?
    pub backColor: Option<String>,
}

pub struct Subjects {
    pub subject: Vec<Subject>,
    //age: time_fet_info_sth TODO,
}

impl Subjects {
    pub fn from_json(text: String) -> Result<Self> {
        //TODO don't do this ugly inline parsing but rather use impl
        //and traits. So that the response function foo can check for a
        //trait which guarantees from_json() to be present and directly
        //uses that instead of having to shift the parsing and serializing here
        let res: RpcRespone<Vec<Subject>> = serde_json::from_str(&text)?;
        Ok(Subjects {
            subject: res.result,
        })
    }
}

//representation of Timetable like retrived via getTimetable
#[derive(Deserialize)]
pub struct TimetableEntry {
    pub id: Option<i64>,
    pub date: Option<i64>,
    pub startTime: Option<i64>,
    pub endTime: Option<i64>,
    pub kl: Option<Vec<KlassenID>>,
    pub te: Option<Vec<TeacherID>>,
    pub su: Option<Vec<SubjectID>>,
    pub ro: Option<Vec<RoomID>>,
    pub lstype: Option<String>,
    pub code: Option<String>,
    pub lstext: Option<String>,
    pub statflags: Option<String>,
    pub activityType: Option<String>,
}

#[derive(Deserialize)]
pub struct SubjectElements{
    pub subjects: Vec<TimetableEntry>,
}

impl SubjectElements {
    pub fn from_json(text: String) -> Result<Self> {
        //TODO don't do this ugly inline parsing but rather use impl
        //and traits. So that the response function foo can check for a
        //trait which guarantees from_json() to be present and directly
        //uses that instead of having to shift the parsing and serializing here
        let res: RpcRespone<Vec<TimetableEntry>> = serde_json::from_str(&text)?;
        Ok(SubjectElements {
            subjects: res.result,
        })
    }
}

#[derive(Deserialize)]
pub struct KlassenID {
    pub id: i64,
}

#[derive(Deserialize)]
pub struct SubjectID {
    pub id: i64,
}

#[derive(Deserialize)]
pub struct TeacherID {
    pub id: i64,
}

#[derive(Deserialize)]
pub struct RoomID {
    pub id: i64,
}
