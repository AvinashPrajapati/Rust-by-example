import { createStore } from "solid-js/store";

// Create a store to store notes
// export const [notes, setNotes] = createStore({ notes: [] });
export const [notes, setNotes] = createStore({ notes: [] });

// Function to set or update the notes list
export const updateNotes = (newNotes) => {
  // console.log(newNotes, "db updates");
  
  setNotes({ notes: newNotes.sort((a, b) => b.id - a.id) }); // Update the store with new notes
};

// Function to add a new note to the list
export const store_addNote = (note) => {
  setNotes("notes", (prevNotes) => [note, ...prevNotes]);
};

// Function to update a specific note
export const store_updateNote = (updatedNote) => {
  setNotes("notes", (prevNotes) =>
    prevNotes.map((note) =>
      note.id === updatedNote.id ? updatedNote : note
    )
  );
};

// Function to remove a note by ID
export const removeNote = (noteId) => {
  setNotes("notes", (prevNotes) => prevNotes.filter((note) => note.id !== noteId));
};
