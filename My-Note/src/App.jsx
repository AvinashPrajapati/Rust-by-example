import "./App.css";

// import {getAllNote} from "./db";
import { invoke } from "@tauri-apps/api/core";

import {
  db_getAllNote,
  deleteNoteById,
  db_addNote,
  db_updateNote,
  validateInputOnServer
} from "../db.js";

import { createSignal, onMount } from "solid-js";
import {
  notes,
  updateNotes,
  store_updateNote,
  setNotes,
  removeNote,
  store_addNote,
} from "./notesStore.js";
import { createStore } from "solid-js/store";


function App() {
  // handleAction()
  const [secureKey, setSecureKey] = createSignal("");
  const [searchTerm, setSearchTerm] = createSignal("");
  const [currentPage, setCurrentPage] = createSignal(1);
  const [popupVisible, setPopupVisible] = createSignal(false);
  const [isAddPopup, setIsAddPopup] = createSignal(true);
  const [popupTitle, setPopupTitle] = createSignal(""); // Explicit title state for popup
  const [noteTitle, setNoteTitle] = createSignal(""); // Separate state for form title

  const [noteType, setNoteType] = createSignal(""); // State for note type

  const [noteContent, setNoteContent] = createSignal(""); // Separate state for form content
  const [checklistItems, setChecklistItems] = createStore({temp_checklist:[]});

  const [editNoteId, setEditNoteId] = createSignal(null); // Keep track of note ID being edited

  const [showWelcomeScreen, setShowWelcomeScreen] = createSignal(true); // Manage UI state

  async function handleWelcomeAction() {
    const userInput = prompt("Are you sure you want to see the Note Application? Type 'ok' to proceed.");
    if (userInput) {
      const isValid = await validateInputOnServer(userInput);
      if (isValid) {
        setSecureKey(userInput.trim());
        // set notes list
        const notesData = await db_getAllNote(userInput.trim()); // Fetch notes from db
        
        updateNotes(notesData); // Update the store with fetched notes
        // console.log(notes, "on mount");

        
        
        setShowWelcomeScreen(false);
      } else {
        alert("Invalid input. Restart the app to try again.");
      }
    } else {
      alert("You chose not to proceed. Restart the app to try again.");
    }
  }


  const itemsPerPage = 10;

  const filteredNotes = () =>
    notes.notes.filter((item) => {
      // console.log(item, "assa") //---

      return item.data.title.toLowerCase().includes(searchTerm().toLowerCase());
    });

  const paginatedNotes = () => {
    const notesData = filteredNotes();
    // console.log('Paginated Notes:', notesData);  // Log the data to check
    return notesData.slice(
      (currentPage() - 1) * itemsPerPage,
      currentPage() * itemsPerPage
    );
  };
  const totalPages = () => Math.ceil(filteredNotes().length / itemsPerPage);

  const openAddNotePopup = () => {
    setIsAddPopup(true);
    setPopupTitle("Add Note");
    setNoteTitle("");
    setNoteType("text");
    setNoteContent("");
    setChecklistItems({ temp_checklist: [] });
    setEditNoteId(null); // No note is being edited
    setPopupVisible(true);
  };

  const openEditNotePopup = (note) => {
    setIsAddPopup(false);
    // console.log(note.note_type, "updated");

    setPopupTitle("Edit Note");
    setNoteTitle(note.data.title);
    setNoteType(note.data.note_type);

    if (note.data.note_type === "text") {
      setNoteContent(note.data.content);
      setChecklistItems({ temp_checklist: [] }); // Clear checklist items for text type
    } else if (note.data.note_type === "checklist") {
      console.log(note.data.content, "edit check");
      
      setChecklistItems({ temp_checklist: note.data.content }); // Load checklist items
      setNoteContent(""); // Clear text content for checklist type
    }

    setEditNoteId(note.id); // Track the ID of the note being edited
    setPopupVisible(true);
  };

  const handleSaveNote = async () => {
    if (editNoteId() !== null) {
      // checklistItems.temp_checklist = 
      const updatedNote = {
        // not required created
        id:editNoteId(),
        title: noteTitle(),
        note_type: noteType(),
        content: (noteType()=='checklist')?checklistItems.temp_checklist:noteContent(),
        status: "no",
        updated: new Date().toISOString(),
      };

      console.log(updatedNote, "update note called");
      try {
        const newUpdatedNote = await db_updateNote(updatedNote, secureKey()); 
        store_updateNote(newUpdatedNote); // Update the note in the store
      } catch (error) {
        console.error("Failed to update note:", error);
        alert("Failed to update note.");
      }
    } else {
      // Adding a new note

      const note_obj = {
        title: noteTitle(),
        note_type: noteType(),
        content: noteType() === "checklist" ? checklistItems.temp_checklist : noteContent(),
        status: "yes",
        created: new Date().toISOString(), // Store as ISO string for consistency
        updated: new Date().toISOString(), // Default updated to created timestamp
      };
      console.log(note_obj, "add note called");
      
      try {
        const res = await db_addNote(note_obj, secureKey()); // Call the backend to add the note
        console.log("Note added", res);

        // parse in JSON
        
        store_addNote(res); // Update the store with the newly added note
      } catch (error) {
        console.error("Failed to add note:", error);
        alert("Failed to add note.");
      }
    }
    setPopupVisible(false);
  };

  const handleDelete = async (noteId) => {
    try {
      console.log(notes.notes);
      await deleteNoteById(noteId); // Call the delete function
      console.log(notes.notes);

      removeNote(noteId);
    } catch (error) {
      console.error("Error deleting note:", error);
    }
  };

  const welcomeScreen = (
  <div class="flex items-center justify-center w-screen h-screen bg-blue-500 text-white">
    <div class="text-center">
      <h1 class="text-4xl font-bold mb-4">Welcome to the Notes App</h1>
      <button
        class="bg-white text-blue-500 px-4 py-2 rounded font-bold hover:bg-gray-100"
        onClick={handleWelcomeAction}
      >
        Enter Notes App
      </button>
    </div>
  </div>
  )
  const noteApp = (
    <div class="p-2 m-auto max-w-3xl">
            <div className="flex justify-start flex-wrap items-center mb-2 gap-2">
              <button
              title="Add New Note"
                class="bg-blue-500 text-white rounded px-2 pb-1"
                onClick={openAddNotePopup}
              >
                <b>+</b>
              </button>
              <h1 class="text-2xl font-bold">
                {/* Add Note Button */}
                Notes App
              </h1>
            </div>

            {/* Search Bar */}
            <input
              type="text"
              placeholder="Search notes..."
              value={searchTerm()}
              onInput={(e) => {
                setSearchTerm(e.target.value);
                setCurrentPage(1); // Reset to the first page on search
              }}
              class="border border-gray-400 p-1 w-full rounded"
            />

            {/* Pagination */}
            <div class="m-auto my-1 w-fit">
              Page : :
              {Array.from({ length: totalPages() }, (_, i) => i + 1).map((page) => (
                <button
                  class={`px-2 mx-1 rounded-full ${
                    page === currentPage() ? "bg-blue-600 text-white" : "bg-gray-300"
                  }`}
                  onClick={() => setCurrentPage(page)}
                >
                  {page}
                </button>
              ))}
            </div>

            {/* Notes List */}
            <div>
              {paginatedNotes().map((note) => (
                <div class="rounded-sm border border-gray-400 p-0 pr-2 mb-1 flex justify-between items-center" key={note.id}>
                  <p class="font-bold text-sm">
                  {note.data.note_type=='text'?<span title="Text Type">📝</span>:<span title="Checklist">✅</span>}- {note.id} • {note.data.title}
                  </p>
                  
                  <div className="flex justify-center items-center gap-2">
                    <button
                      class="text-red-500 text-xs"
                      onClick={() => {
                        if (confirm("Are you sure you want to delete this note?")) {
                          handleDelete(note.id);
                        }
                      }}
                    >
                      <b>Delete</b>
                    </button>

                    <button
                      class="text-blue-500 text-xs"
                      onClick={() => {
                        setIsAddPopup(false);
                        openEditNotePopup(note);
                      }}
                    >
                      <b>Edit</b>
                    </button>
                  </div>
                </div>
              ))}
            </div>

            {/* Popup for Add/Edit Note */}
            {popupVisible() && (
              <div class="fixed inset-0 bg-black bg-opacity-50 flex justify-center items-center">
                <div class="bg-white p-4 rounded w-667 w-full-95">
                  <h2 class="text-lg font-bold">{popupTitle()}</h2>
                  <input
                    type="text"
                    placeholder="Title"
                    class="border border-gray-400 p-2 w-full mb-2 rounded "
                    value={noteTitle()}
                    onInput={(e) => setNoteTitle(e.target.value)}
                  />

                  {isAddPopup() ? (
                    <select
                      value={noteType()}
                      class="border border-gray-400 p-2 w-full mb-2 rounded"
                      onChange={(e) => setNoteType(e.target.value)}
                      required
                    >
                      <option value="text">Text</option>
                      <option value="checklist">Checklist</option>
                    </select>
                  ) : (
                    <p>
                      <small>
                        <b>Note Type : : </b>
                        {noteType()}
                      </small>
                    </p>
                  )}

                  {noteType() === "checklist" ? (
                    //
                    <>
                      <ul>
                        {checklistItems.temp_checklist.map((item, index) => (
                          <li key={item.id} class="flex items-center mb-2">

                            <button
                              class="mr-2 text-red-500"
                              onClick={() => {
                                setChecklistItems("temp_checklist", (prev) =>
                                  prev.filter((_, i) => i !== index)
                                );
                              }}
                            >
                              <b>Delete</b>
                            </button>


                            <input
                              type="text"
                              placeholder="Task"
                              value={item.title}
                              onInput={(e) =>{
                                setChecklistItems("temp_checklist", index, "title", e.target.value);
                              }
                              }
                              class="border border-gray-400 p-1 w-full rounded"
                            />
                            <select
                              value={item.completed}
                              onChange={(e) =>
                                handleChecklistItemChange(
                                  index,
                                  "completed",
                                  e.target.value
                                )
                              }
                              class="border border-gray-400 ml-2 p-1 rounded"
                            >
                              <option value="yes">Yes</option>
                              <option value="no">No</option>
                            </select>
                           
                          </li>
                        ))}
                      </ul>
                      <button
                        class="text-blue-500 mt-2"
                        onClick={() => {
                          setChecklistItems("temp_checklist", (prev) => [
                            ...prev,
                            { id: Date.now(), title: "", completed: "no" },
                          ]);
                        }}
                      >
                        + Add Task
                      </button>
                    </>
                  ) : (
                    <textarea
                      placeholder="Content"
                      value={noteContent()}
                      onInput={(e) => setNoteContent(e.target.value)}
                      class="border border-gray-400 p-2 w-full mb-2 resize-none h-40 rounded"
                    />
                  )}

                  <div class="flex justify-end">
                    <button
                      class="text-red-500 mr-2"
                      onClick={() => setPopupVisible(false)}
                    >
                      Cancel
                    </button>
                    <button class="text-blue-500" onClick={handleSaveNote}>
                      Save
                    </button>
                  </div>
                </div>
              </div>
            )}
          </div>
  )

  return (
    <>
      {showWelcomeScreen() ? welcomeScreen : noteApp}
    </>
  );
}

export default App;
