import React, { useState, useEffect } from "react";
import axios from 'axios';

const baseurl = process.env.REACT_APP_API_URL;

interface Note {
  id: string;
  content: string;
}

function App() {
  const [notes, setNotes] = useState<Note[]>([]);

  useEffect(() => {
    // Fetch initial notes from server
    axios.get(`${baseurl}`)
      .then(response => {
        setNotes(response.data);
      })
      .catch(error => {
        console.error('Error fetching notes:', error);
      });
  }, []);

  const handleNoteAdd = () => {
    // Send POST request to create new note
    //the request body should contain a JSON object with a single property content
    axios.post(`${baseurl}`, { "content": "" })
      .then(response => {
        //the response body from the POST request will have an "id" field and a "content" field in the JSON format.
        setNotes((prevNotes) => [...prevNotes, response.data]);
      })
      .catch(error => {
        console.error('Error creating note:', error);
      });
  };

  const handleNoteChange = (index: number, event: React.ChangeEvent<HTMLTextAreaElement>) => {
    const noteToUpdate = notes[index];
    const newNote = { ...noteToUpdate, content: event.target.value };
    // Send PUT request to update note
    //the request body should contain a JSON object with a single property content, which represents the updated content of the note
    axios.put(`${baseurl}/${noteToUpdate.id}`, newNote)
      .then(response => {
        //The response body is expected to contain an "id" field and a "content" field in JSON format.
        const newNotes = [...notes];
        newNotes[index] = response.data;
        setNotes(newNotes);
      })
      .catch(error => {
        console.error('Error updating note:', error);
      });
  };

  const handleNoteDelete = (index: number) => {
    const noteToDelete = notes[index];
    // Send DELETE request to delete note
    //There is no request body for the delete request
    axios.delete(`${baseurl}/${noteToDelete.id}`)
      .then(response => {
        const newNotes = [...notes];
        newNotes.splice(index, 1);
        setNotes(newNotes);
      })
      .catch(error => {
        console.error('Error deleting note:', error);
      });
  };

  return (
    <div>
      <button onClick={handleNoteAdd}>Create Note</button>
      {notes.map((note, index) => (
        <div key={note.id}>
          <textarea value={note.content} onChange={(event) => handleNoteChange(index, event)} />
          <button onClick={() => handleNoteDelete(index)}>Delete</button>
        </div>
      ))}
    </div>
  );
}

export default App;
