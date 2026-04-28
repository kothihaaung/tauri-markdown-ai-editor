<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Button } from "$lib/components/ui/button";
  import { FileText } from "lucide-svelte";
  import type { Note } from "$lib/types";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import NoteEditor from "$lib/components/NoteEditor.svelte";

  // State
  let notes = $state<Note[]>([]);
  let activeNoteId = $state("");
  let isAnalyzing = $state(false);
  let isPreview = $state(false);
  let searchQuery = $state("");

  // Derived
  let activeNote = $derived(
    notes.find((n) => n.id === activeNoteId) || notes[0],
  );

  // Effects
  $effect(() => {
    const saved = localStorage.getItem("brain-notes");
    if (saved) {
      try {
        const parsed = JSON.parse(saved);
        if (parsed?.length > 0) {
          notes = parsed;
          activeNoteId = notes[0].id;
        } else {
          createNewNote();
        }
      } catch (e) {
        createNewNote();
      }
    } else {
      createNewNote();
    }
  });

  $effect(() => {
    if (notes.length > 0)
      localStorage.setItem("brain-notes", JSON.stringify(notes));
  });

  const filteredNotes = $derived(
    notes
      .filter(
        (n) =>
          n.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
          n.content.toLowerCase().includes(searchQuery.toLowerCase()),
      )
      .sort((a, b) => b.updatedAt - a.updatedAt),
  );

  // AI Logic
  async function handleAnalyze() {
    if (!activeNote?.content) return;
    isAnalyzing = true;
    try {
      const result: { title: string; summary: string } = await invoke(
        "analyze_note_content",
        { content: activeNote.content },
      );

      const index = notes.findIndex((n) => n.id === activeNoteId);
      if (index !== -1) {
        // Mutate properties directly — Svelte 5's $state proxy tracks these.
        // Do NOT replace the object (notes[index] = {...}) as that breaks
        // the keyed {#each} binding reference.
        notes[index].title = result.title.trim();
        notes[index].summary = result.summary.trim();
        notes[index].updatedAt = Date.now();
      }
    } catch (err) {
      console.error("❌ AI Error:", err);
    } finally {
      isAnalyzing = false;
    }
  }

  function createNewNote() {
    const newNote: Note = {
      id: Math.random().toString(36).substring(7),
      title: "Untitled Note",
      content: "",
      summary: "",
      updatedAt: Date.now(),
    };
    notes = [newNote, ...notes];
    activeNoteId = newNote.id;
    isPreview = false;
  }

  function deleteNote(id: string, event: MouseEvent) {
    event.stopPropagation();
    if (notes.length <= 1) {
      const index = notes.findIndex(n => n.id === id);
      if (index !== -1) {
        notes[index].title = "Untitled Note";
        notes[index].content = "";
        notes[index].summary = "";
        notes[index].updatedAt = Date.now();
      }
      return;
    }
    notes = notes.filter((n) => n.id !== id);
    if (activeNoteId === id) activeNoteId = notes[0].id;
  }
</script>

<div class="h-screen w-full bg-[#050505] text-zinc-400 font-inter flex overflow-hidden">
  <Sidebar
    {filteredNotes}
    bind:searchQuery
    bind:activeNoteId
    bind:isPreview
    {createNewNote}
    {deleteNote}
  />

  <!-- Main Content -->
  <main class="flex-1 flex flex-col min-w-0">
    {#if notes.length > 0 && activeNoteId}
      {#each notes as note, i (note.id)}
        {#if note.id === activeNoteId}
          <NoteEditor
            bind:note={notes[i]}
            bind:isPreview
            {isAnalyzing}
            {handleAnalyze}
          />
        {/if}
      {/each}
    {:else}
      <div class="flex-1 flex items-center justify-center text-zinc-700">
        <div class="text-center">
          <FileText size={48} class="mx-auto mb-4 opacity-20" />
          <p>No note selected</p>
          <Button variant="outline" size="sm" class="mt-4" onclick={createNewNote}>Create New Note</Button>
        </div>
      </div>
    {/if}
  </main>
</div>

<style>
  :global(.prose) { font-family: 'Inter', sans-serif; color: #a1a1aa; }
  :global(.prose h1) { color: white; font-size: 2rem; font-weight: 800; margin-bottom: 1.5rem; }
  :global(.prose h2) { color: white; font-size: 1.5rem; font-weight: 700; margin-top: 2rem; }
  :global(.prose p) { margin-bottom: 1.25rem; line-height: 1.8; }
  :global(.prose code) { background: rgba(255, 255, 255, 0.05); padding: 0.2rem 0.4rem; border-radius: 4px; color: #e2e2e2; }
  :global(.prose pre) { background: #0c0c0c; padding: 1rem; border-radius: 8px; border: 1px solid rgba(255,255,255,0.05); overflow-x: auto; }
  
  ::-webkit-scrollbar { width: 6px; }
  ::-webkit-scrollbar-track { background: transparent; }
  ::-webkit-scrollbar-thumb { background: rgba(255, 255, 255, 0.05); border-radius: 10px; }
  ::-webkit-scrollbar-thumb:hover { background: rgba(255, 255, 255, 0.1); }
  :global(body) { background: #050505; margin: 0; overflow: hidden; }
</style>