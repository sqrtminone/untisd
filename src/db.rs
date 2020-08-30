use rusqlite::{params, Connection};
use anyhow::Result;

use crate::request::{Rooms, Subjects, SubjectElements, TimetableEntry};

pub fn init_db() -> Result<rusqlite::Connection> {
    let dbcon = Connection::open("untisd.db")?;
    dbcon.execute(
    "CREATE TABLE IF NOT EXISTS rooms (
        id INTEGER,
        name TEXT,
        longName TEXT,
        foreColor INTEGER,
        backColor INTEGER
    );", params![] )?;
    
    dbcon.execute(
    "CREATE TABLE IF NOT EXISTS subjects (
        id INTEGER,
        name TEXT,
        longName TEXT,
        foreColor INTEGER,
        backColor INTEGER
    );", params![] )?;

    dbcon.execute(
    "CREATE TABLE IF NOT EXISTS timetable (
        id INTEGER,
        date INTEGER,
        startTime INTEGER,
        endTime INTEGER,
        lstype TEXT,
        code TEXT,
        lstext TEXT,
        statflags TEXT,
        activityType TEXT
    );", params![])?;
   
    //TODO find out if having a dedicated timetabe$ID_to_klassen
    //table. Like a table with the name of the timetable entry that
    //only contains the ids for the klassen in it. So that no additional
    //iteration->comparision has to be performed but rather can be just a
    //linear iteration
    dbcon.execute(
    "CREATE TABLE IF NOT EXISTS to_klassen_timetable (
        timetable_id INTEGER,
        rooms_id INTEGER
    );", params![] )?;

    dbcon.execute(
    "CREATE TABLE IF NOT EXISTS teachers_to_timetable (
        timetable_id INTEGER,
        klassen_id INTEGER
    );", params![] )?;

    dbcon.execute(
    "CREATE TABLE IF NOT EXISTS subjects_to_timetable (
        timetable_id INTEGER,
        subject_id INTEGER
    );", params![] )?;

    dbcon.execute(
    "CREATE TABLE IF NOT EXISTS rooms_to_timetable_entry (
        timetable_entry_id INTEGER,
        room_id INTEGER
    );", params![] )?;

    Ok(dbcon)
}

//function responsible for updating the rooms table that
//represents any possible/existing room
pub fn db_update_rooms(c: &Connection ,rooms: Rooms) -> Result<()> {
    //TODO check config file for behavior on update of values.
    //Like what to do if a room value changed and now there would
    //be a collision with the old value. Should the table be dumped
    //somewhere, the updated value be ignored or simply overwritten
    
    //todo for now just drop the old table and overwrite it
    //(i know its ghetto but works for now)
    c.execute("DELETE FROM rooms;", params![]);
    for room in rooms.room {
        //TODO add logging facilities to trace when a room got added or
        //removed
        c.execute("INSERT INTO rooms
        (
            id,
            name,
            longName,
            foreColor,
            backColor
        ) VALUES ( ?1, ?2, ?3, ?4, ?5);",
        params![ room.id, room.name, room.longName,
            room.foreColor, room.backColor],)?;
    }
    Ok(())
}


pub fn db_update_subjects(c: &Connection, subjects: Subjects) -> Result<()> {
    //TODO check config file for behavior on update of values.
    //Like what to do if a room value changed and now there would
    //be a collision with the old value. Should the table be dumped
    //somewhere, the updated value be ignored or simply overwritten
    
    //todo for now just drop the old table and overwrite it
    //(i know its ghetto but works for now)
    c.execute("DELETE FROM subjects;", params![]);
    for subject in subjects.subject {
        //TODO add logging facilities to trace when a room got added or
        //removed
        c.execute("INSERT INTO subjects
        (
            id,
            name,
            longName,
            foreColor,
            backColor
        ) VALUES ( ?1, ?2, ?3, ?4, ?5);",
        params![ subject.id, subject.name, subject.longName,
            subject.foreColor, subject.backColor],)?;
    }
    Ok(())
}

pub fn db_update_timetable(c: &Connection, timetable: Vec<TimetableEntry> ) -> Result<()> {
    //TODO check config file for behavior on update of values.
    //Like what to do if a room value changed and now there would
    //be a collision with the old value. Should the table be dumped
    //somewhere, the updated value be ignored or simply overwritten
    
    //todo for now just drop the old table and overwrite it
    //(i know its ghetto but works for now)
    c.execute("DELETE FROM timetable;", params![])?;
    c.execute("DELETE FROM rooms_to_timetable_entry;", params![]);
    for timetable_entry in timetable {
        //TODO add logging facilities to trace when a room got added or
        //removed
        for room in timetable_entry.ro.unwrap() {
            c.execute("INSERT INTO rooms_to_timetable_entry
            (
                timetable_entry_id,
                room_id
            ) VALUES ( ?1, ?2);", params![
                timetable_entry.id,
                room.id
            ],)?;
        }
        c.execute("INSERT INTO timetable
        (
            id,
            date,
            startTime,
            endTime,
            lstype,
            code,
            lstext,
            statflags,
            activityType
        ) VALUES ( ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9);",
        params![  
            timetable_entry.id,
            timetable_entry.date,
            timetable_entry.startTime,
            timetable_entry.endTime,
            timetable_entry.lstype,
            timetable_entry.code,
            timetable_entry.lstext,
            timetable_entry.statflags,
            timetable_entry.activityType,
        ],)?;
    }
    Ok(())
}

struct TimetableResolvedEntry {
    startTime: i64,
    endTime: i64,
    name: String,
}

//gets 
pub fn link_subtables_for_timetable_entry(c: &Connection) -> Result<()> {
    let mut stmt = c.prepare("SELECT timetable.startTime,endTime,name FROM timetable 
                                INNER JOIN rooms_to_timetable_entry 
                                    ON timetable.id=rooms_to_timetable_entry.timetable_entry_id
                                    INNER JOIN rooms ON
                                        rooms_to_timetable_entry.room_id=rooms.id
                             ORDER BY startTime ASC;")?;
    let boi = stmt.query_map(params![], |row|
                             Ok( TimetableResolvedEntry {
                                 startTime: row.get(0)?,
                                 endTime: row.get(1)?,
                                 name: row.get(2)?,
                             })
    )?;
    for i in boi {
        let i = i.unwrap();
        println!("{} | {} | {}",
                 i.startTime,
                 i.endTime,
                 i.name
        );
    }
    Ok(())
}
