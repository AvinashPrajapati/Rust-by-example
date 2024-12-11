// src/db.rs

// mod enc_dec;

use sqlx::{sqlite::{SqlitePool}, Sqlite, Row};
use sqlx::migrate::MigrateDatabase;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChecklistItem {
    pub id: i64,
    pub title: String,
    pub completed: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum NoteContent {
    Text(String),
    Checklist(Vec<ChecklistItem>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NoteData {
    pub title: String,
    pub note_type: String,       // Consider using an enum for note types
    pub content: NoteContent,   // Text or Checklist
    pub status: String,         // e.g., "active", "archived"
    pub created: Option<String>, // Optional for flexibility
    pub updated: Option<String>, // Optional for flexibility
}

// Database representation
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Note {
    pub id: i64,        // Unique identifier
    pub data: NoteData, // Encapsulated note data
}

// API payload for creating/updating notes
#[derive(Serialize, Deserialize, Debug)]
pub struct NotePayload {
    pub data: NoteData,
}

// API payload for creating/updating notes
#[derive(Serialize, Deserialize, Debug)]
pub struct EncNotePayload {
    pub data: String,
}
// API payload for creating/updating notes
#[derive(Serialize, Deserialize, Debug)]
pub struct ReadEncNotePayload {
    pub id: i64,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateNotePayload {
    pub id: i64,
    pub title: String,
    pub note_type: String,       // Consider using an enum for note types
    pub content: NoteContent,   // Text or Checklist
    pub status: String,         // e.g., "active", "archived"
    pub updated: Option<String>, // Optional for flexibility
}

pub async fn initialize_db() -> Result<SqlitePool, sqlx::Error> {
    // Define the SQLite database URL (for example, an in-memory DB for testing or a file-based DB)
    // some code.....
    let db_url = "sqlite://sqlite3.db";

    let pool:SqlitePool;

    if !<Sqlite as MigrateDatabase>::database_exists(db_url).await.unwrap_or(false) {
        // Create the database if it doesn't exist
        let _ = <Sqlite as MigrateDatabase>::create_database(db_url).await;

        pool = SqlitePool::connect(db_url).await?;

        // Create the schema for the notes table if it doesn't already exist
        let query = "
            CREATE TABLE IF NOT EXISTS notes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                data TEXT NOT NULL
            );
        ";
        sqlx::query(query).execute(&pool).await?;

    } else {
        pool = SqlitePool::connect(db_url).await?;

    }
   
    Ok(pool)
}


// use crate::enc_dec::*;

pub async fn create_note(
    pool: &SqlitePool,
    enc_data: String,
) -> Result<ReadEncNotePayload, sqlx::Error> {

    // let note_json = serde_json::to_string(&enc_payload.data).unwrap();

    let query = "INSERT INTO notes (data) VALUES ($1)";
    
    let result = sqlx::query(query)
        .bind(&enc_data) // Convert NoteType to string
        .execute(pool)
        .await?;

    let last_id = result.last_insert_rowid();
    let recent_note = match fetch_by_id(pool, &last_id).await {
        Ok(note) => {
            // Convert the fetched note to JSON
            Ok(note)
        }
        Err(e) => {
            eprintln!("Error fetching note: {:?}", e);
            Err(e)
        }
    };

    Ok(recent_note.unwrap())
}

pub async fn fetch_by_id(pool: &SqlitePool, id: &i64) -> Result<ReadEncNotePayload, sqlx::Error> {
    let query = "SELECT * FROM notes WHERE id = $1";
    let row = sqlx::query(query).bind(id).fetch_one(pool).await?;

    let note = ReadEncNotePayload {
        id: row.try_get("id")?,
        data: row.try_get("data")?,
    };

    Ok(note)
}


pub async fn delete_note(pool: &SqlitePool, id: &i64) -> Result<ReadEncNotePayload, sqlx::Error> {
    let updated_note = fetch_by_id(pool, &id).await?;
    
    let query = "DELETE FROM notes WHERE id = $1";
    let _ = sqlx::query(query).bind(id).execute(pool).await?;


    Ok(updated_note)
}


pub async fn update_note(
    pool: &SqlitePool,
    update_payload: ReadEncNotePayload,
) -> Result<ReadEncNotePayload, sqlx::Error> {

    let query = "UPDATE notes SET data = $1 WHERE id = $2";

    let _ = sqlx::query(query)
        .bind(&update_payload.data)
        .bind(&update_payload.id)
        .execute(pool)
        .await?;

    let recent_update_note = match fetch_by_id(pool, &update_payload.id).await {
        Ok(note) => {
            // Convert the fetched note to JSON
            Ok(note)
        }
        Err(e) => {
            eprintln!("Error fetching note: {:?}", e);
            Err(e)
        }
    };

    Ok(recent_update_note.unwrap())

}


pub async fn fetch_all(pool: &SqlitePool) -> Result<Vec<ReadEncNotePayload>, sqlx::Error> {
    let query = "SELECT * FROM notes ORDER BY id DESC";

    // Fetch all rows
    let rows = sqlx::query(query).fetch_all(pool).await?;

    let notes: Vec<ReadEncNotePayload> = rows
        .into_iter()
        .map(|row| {
            Ok(ReadEncNotePayload {
                id: row.try_get("id").unwrap_or_default(),
                data: row.try_get("data").unwrap_or_default(), // Assuming data is already in the desired format
            })
        })
        .collect::<Result<Vec<ReadEncNotePayload>, sqlx::Error>>()?;

    // Wrap the notes in Ok() and return
    Ok(notes)
}






