mod untis;
mod rpc;
mod request;
mod db;

//TODO create abstraction module instead of raw using it

use rusqlite::{params, Connection};

use anyhow::Result;
use untis::Untis;
use request::{Rooms, Subjects};

use crate::rpc::SimpleTimetableParams;
use crate::request::SubjectElements;
use crate::db::{
    init_db, db_update_rooms, db_update_subjects, db_update_timetable,
    link_subtables_for_timetable_entry};


fn main() -> Result<()> {
    let mut u = Untis::new("Leibniz-gym-remscheid",
                       "mese",
                       "STUDENT_ID",
                       "PASSWORD");



    //u.conntextion_info();
    u.login();

    let c = init_db()?;

    let rooms = Rooms::from_json( u.issue_request("getRooms").unwrap() );
    //for i in rooms.unwrap().room {
    //    println!("{} / {} --> {}", i.name, i.longName, i.id);
    //}
   
    //println!("\n\n\n--------------\n\n");
    let subjects = Subjects::from_json( u.issue_request("getSubjects").unwrap() );
    //for i in subjects.unwrap().subject {
    //    println!("{} / {} --> {}", i.name, i.longName, i.id);
    //}

    let rp = SimpleTimetableParams::new(
        // 352, 353, 354, 444
        u.session.as_ref().unwrap().class_id , 1,
        Some(20200831),
        Some(20200831));
    let ttable_subjects = SubjectElements::from_json( 
        u.issue_request_parameterized("getTimetable", rp).unwrap() );

    //for i in ttable_subjects.unwrap().subjects {
    //    println!("\t{}/{} -> {}",i.date.unwrap(), i.startTime.unwrap(),
    //    i.ro.unwrap()[0].id);
    //}

    db_update_rooms(&c, rooms.unwrap());
    db_update_subjects(&c, subjects.unwrap());
    db_update_timetable(&c, ttable_subjects.unwrap().subjects)?;

    link_subtables_for_timetable_entry(&c);

    Ok(())
}
