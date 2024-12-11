
mod db;
mod enc_dec;


use db::*;
use enc_dec::*;

use sqlx::{sqlite::{SqlitePool}};

use tauri::Manager;

use tauri::{command,State};

use std::{sync::Mutex};



#[derive(Debug)]
pub struct PoolState {
  #[allow(dead_code)]
  pool:SqlitePool
}
#[derive(Default, Clone)]
pub struct KeyStorage {
    key_store: Option<String>,  // The key is stored in the Mutex-wrapped KeyValue
}

#[command]
async fn add_note_item(
    state: State<'_, PoolState>,
    body: String,
    key_str: String,
) -> Result<String, String> {

    // Parse the JSON body into `NotePayload`
    let note_payload: NoteData = match serde_json::from_str(&body) {
        Ok(payload) => payload,
        Err(e) => {
            eprintln!("Invalid JSON payload: {:?}", e);
            return Err(format!("Invalid JSON: {}", e));
        }
    }; // validate fields 
    let note_string = serde_json::to_string(&note_payload).unwrap();

    // Encrypt the note data
    let encrypted_data = encrypt_data(&key_str.as_bytes(), &note_string).map_err(|e| {
        eprintln!("Error encrypting note data: {:?}", e);
        format!("Error encrypting note data: {}", e)
    })?;

    // Call the database function to create the note
    match create_note(&state.pool, encrypted_data).await {
        Ok(note_json) => {
                
            let decrypted_data = decrypt_data(&key_str.as_bytes(), &note_json.data)
            .map_err(|e|  {
                eprintln!("Error encrypting note data: {:?}", e);
                format!("Error encrypting note data: {}", e)
            })?;
            // println!("data : {:?}",decrypted_data);
            let note_data:NoteData = serde_json::from_str(&decrypted_data).map_err(|e| format!("Serialization error: {}", e))?;
            let updated_note = Note {
                id: note_json.id,
                data: note_data,
            };
            let note_string = serde_json::to_string(&updated_note).unwrap();
            Ok(note_string)
        }
        Err(e) => {
            eprintln!("Error adding note to the database: {:?}", e);
            Err(format!("Error adding note: {}", e))
        }
    }
}

// #[command]
// async fn get_note(
//     state: State<'_, PoolState>, 
//     id: String
// ) -> Result<String, String> {
//     match fetch_by_id(&state.pool, &id).await {
//         Ok(note) => {
//             let note_json = serde_json::to_string(&note);
//             Ok(note_json.unwrap())
//         },
//         Err(e) => {
//             eprintln!("Error fetching note: {:?}", e);
//             Err(format!("Error fetching note: {}", e))
//         }
//     }
// }

#[command]
async fn delete_note_by_id(
    state: State<'_, PoolState>, 
    id: i64
) -> Result<String, String> {
    println!{"{} - id", id};
    match delete_note(&state.pool, &id).await {
        Ok(note_json) => {
            let note_str = serde_json::to_string(&note_json);
            Ok(note_str.unwrap())
        }
        Err(e) => {
            eprintln!("Error fetching note: {:?}", e);
            // Err(format!("Error fetching note!!"))
            Err(format!("Error fetching note: {}", e))
        }
    }
}



#[command]
async fn update_note_item(
    state: State<'_, PoolState>,
    body: String,
    key_str: String,
) -> Result<String, String> {

    let updated_note_json: UpdateNotePayload = match serde_json::from_str(&body) {
        Ok(payload) => payload,
        Err(e) => {
            eprintln!("Invalid JSON payload: {:?}", e);
            return Err(format!("Invalid JSON: {}", e));
        }
    };
    // fetch note : ReadEncNotePayload id, data which is encrypted
    let fetch_encrypted_note_from_db: ReadEncNotePayload = fetch_by_id(&state.pool, &updated_note_json.id)
    .await
    .map_err(|e| e.to_string())?;
    // decrypt the data in ReadEncNotePayload type and update with updated_note_json
    let cleaned_data = fetch_encrypted_note_from_db.data.trim_matches('"');
    let decrypted_data = decrypt_data(&key_str.as_bytes(), &cleaned_data)
                    .map_err(|e|  {
                        eprintln!("Error decrypting note data: {:?}", e);
                        format!("Error decrypting note data: {}", e)
                    })?;
    // println!("old note data str :: {:?}", decrypted_data);
    // Update the note with decrypted data
    let mut note_data:NoteData = serde_json::from_str(&decrypted_data).map_err(|e| format!("Serialization error: {}", e))?;

    // println!("old note data json :: {:?}", note_data);
    note_data.title = updated_note_json.title;
    note_data.note_type = updated_note_json.note_type;
    note_data.content = updated_note_json.content;
    note_data.status = updated_note_json.status;
    note_data.updated = updated_note_json.updated;

    // println!("new note data :: {:?}", note_data);

    let note_string = serde_json::to_string(&note_data).unwrap();

    // encrupt the note_data
    // Encrypt the note data
    let encrypted_data = encrypt_data(key_str.as_bytes(), &note_string)
        .map_err(|e| {
        eprintln!("Error encrypting note data: {:?}", e);
        format!("Error encrypting note data: {}", e)
            }
        )?;  // str


    let updated_note = ReadEncNotePayload {
        id: fetch_encrypted_note_from_db.id,
        data: encrypted_data,
    };


    match update_note(&state.pool, updated_note).await {
        Ok(note_json) => {
                
            let decrypted_data = decrypt_data(&key_str.as_bytes(), &note_json.data)
            .map_err(|e|  {
                eprintln!("Error decrypting note data: {:?}", e);
                format!("Error decrypting note data: {}", e)
            })?;
            // println!("data : {:?}",decrypted_data);
            let note_data:NoteData = serde_json::from_str(&decrypted_data).map_err(|e| format!("Serialization error: {}", e))?;
            let updated_note = Note {
                id: note_json.id,
                data: note_data,
            };
            let note_string = serde_json::to_string(&updated_note).unwrap();
            Ok(note_string)
        }
        Err(e) => {
            eprintln!("Error updating note to the database: {:?}", e);
            Err(format!("Error updating note: {}", e))
        }
    }
}
   

#[command]
async fn get_all_notes(
    state: State<'_, PoolState>,
    key_str:String,
) -> Result<String, String>{

    match fetch_all(&state.pool).await {
        Ok(notes) => {


            let mut decrypted_notes = Vec::new();
            for note in notes {
                let cleaned_data = note.data.trim_matches('"');
                let decrypted_data = decrypt_data(&key_str.as_bytes(), &cleaned_data)
                    .map_err(|e|  {
                        eprintln!("Error encrypting note data: {:?}", e);
                        format!("Error encrypting note data: {}", e)
                    })?;

                // Update the note with decrypted data
                let note_data:NoteData = serde_json::from_str(&decrypted_data).map_err(|e| format!("Serialization error: {}", e))?;
                // println!("{:?}", note_data);

                let updated_note = Note {
                    id: note.id,
                    data: note_data,
                };

                decrypted_notes.push(updated_note);
            }
            // println!("\n dec note :: {:?}", decrypted_notes);
            // // Serialize the decrypted notes to JSON
            let dec_notes_str = serde_json::to_string(&decrypted_notes)
                .map_err(|e| format!("Serialization error: {}", e));
            // println!("{:?}", dec_notes_str);
            
            // dec_notes_str
            dec_notes_str
            // Ok(format!("asdasd"))


        },
        Err(e) => {
            eprintln!("Error fetching all notes: {:?}", e);
            Err(format!("Error fetching note: {}", e))
        }
    }
}

#[command]
async fn validate_key(
    state: State<'_, Mutex<KeyStorage>>,
    key: String,
) -> Result<bool, String> {
    let input_key = convert_str_to_key(&key)
        .map_err(|e| format!("Invalid input key: {}", e))?;
    let stored_key = read_key_from_file()
        .map_err(|e| format!("Failed to read the stored key: {}", e))?;
    
    if input_key == stored_key {
        let mut state = state.lock().unwrap();
        state.key_store = Some(key);

        println!("Updated stored key: {:?}", state.key_store);
        Ok(true)
    } else {
        Err("Key validation failed: Provided key does not match the stored key.".to_string())
    }
}





#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            save_key_to_file_in_current_dir().expect("Failed to save key to file");

            // Manage the states synchronously
            app.manage(Mutex::new(KeyStorage{
                key_store:None
            }));
            app.manage(PoolState {
                pool: tauri::async_runtime::block_on(initialize_db())
                    .expect("Failed to initialize the database"),
            });

                Ok(())
            })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            add_note_item,
            delete_note_by_id,
            update_note_item,
            get_all_notes,

            validate_key
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
