
import { invoke } from "@tauri-apps/api/core";

export async function db_addNote(noteItem, session_key) {
    try {
      // const result = await invoke('get_all_notes');
      const res = await invoke('add_note_item', {body:JSON.stringify(noteItem), keyStr:session_key});
      // console.log("db note Add::", res); // Should print: "Note added 
      return JSON.parse(res);
    } catch (error) {
      console.error('Error adding note:', error);
    }
  }
export async function db_updateNote(noteItem, session_key) {
    try {
      const res = await invoke('update_note_item', {body:JSON.stringify(noteItem), keyStr:session_key});
      return JSON.parse(res);

    } catch (error) {
      console.error('Error adding note:', error);
    }
  }
export async function deleteNoteById(noteId) {
    try {
      const res = await invoke('delete_note_by_id', {id:noteId} )
      // console.log(res); // Should print: "Note added successfully"
    } catch (error) {
      console.error('Error adding note:', error);
    }
  }

// // future scope
// async function getNoteById() {
//     try {
//       const result = await invoke('get_all_notes');
//       // const result = await invoke('add_note_item', {
//       //   title: 'My Sample Note',
//       //   typeOfNote: 'text',
//       //   status: 'active'
//       // });
//       console.log(result); // Should print: "Note added successfully"
//     } catch (error) {
//       console.error('Error adding note:', error);
//     }
//   }

// done
export async function db_getAllNote(session_key) {
    try {
      const res = await invoke('get_all_notes', {keyStr:session_key});
      const notes = JSON.parse(res);
      return notes;
    } catch (error) {
      console.error('Error adding note:', error);
      return [];
    }
  }

//------
export async function validateInputOnServer(input) {
    const response = await invoke('validate_key', {key:input})
    // console.log(response, "res");
    
    return response
  }

