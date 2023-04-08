use crate::db;
use crate::error_handler::CustomError;
use crate::schema::notes;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "notes"] //allows you to do "notes::table"
pub struct Note {
    pub content: String,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Notes {
    pub id: i32,
    pub content: String,
}

impl Notes {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let the_dbnotes = notes::table.load::<Notes>(&conn)?;
        Ok(the_dbnotes)
    }
    pub fn create(note: Note) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let note_to_insert = Note::from(note); //used for value-to-value conversions while consuming the input value (implemented below)
        let the_dbnotes = diesel::insert_into(notes::table)
            .values(note_to_insert)
            .get_result(&conn)?;
        Ok(the_dbnotes)
    }
    pub fn update(id: i32, note: Note) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let note_updated = diesel::update(notes::table)
            .filter(notes::id.eq(id))
            .set(note)
            .get_result(&conn)?;
        Ok(note_updated)
    }
    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(notes::table.filter(notes::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

impl Note {
    fn from(note: Note) -> Note {
        Note {
            content: note.content,
        }
    }
}