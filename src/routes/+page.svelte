<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Textarea } from "$lib/components/ui/textarea";
  import { Button } from "$lib/components/ui/button";
  import { marked } from "marked";
  import { 
    Plus, 
    FileText, 
    Sparkles, 
    Eye, 
    Edit3, 
    Search,
    Trash2,
    Clock
  } from "lucide-svelte";
  import { fade, slide } from "svelte/transition";

  // Note type definition
  type Note = {
    id: string;
    title: string;
    content: string;
    summary: string;
    updatedAt: number;
  };

  // State management using Svelte 5 runes
  let notes = $state<Note[]>([]);
  let activeNoteId = $state("");
  let isAnalyzing = $state(false);
  let isPreview = $state(false);
  let searchQuery = $state("");

  // Load from LocalStorage on mount
  $effect(() => {
    const saved = localStorage.getItem("brain-notes");
    if (saved) {
      try {
        const parsed = JSON.parse(saved);
        if (parsed && parsed.length > 0) {
          notes = parsed;
          activeNoteId = notes[0].id;
        } else {
          createNewNote();
        }
      } catch (e) {
        console.error("Failed to load notes", e);
        createNewNote();
      }
    } else {
      createNewNote();
    }
  });

  // Save to LocalStorage whenever notes change
  $effect(() => {
    if (notes.length > 0) {
      localStorage.setItem("brain-notes", JSON.stringify(notes));
    }
  });

  // Derived state
  const activeNote = $derived(notes.find(n => n.id === activeNoteId) || notes[0]);
  const filteredNotes = $derived(
    notes.filter(n => 
      n.title.toLowerCase().includes(searchQuery.toLowerCase()) || 
      n.content.toLowerCase().includes(searchQuery.toLowerCase())
    ).sort((a, b) => b.updatedAt - a.updatedAt)
  );

  async function handleAnalyze() {
    if (!activeNote?.content) return;
    isAnalyzing = true;
    try {
      const result: { title: string, summary: string } = await invoke("analyze_note_content", { 
        content: activeNote.content 
      });
      
      // Update the note in the list
      const index = notes.findIndex(n => n.id === activeNoteId);
      if (index !== -1) {
        notes[index].title = result.title;
        notes[index].summary = result.summary;
        notes[index].updatedAt = Date.now();
        // Trigger re-assignment for deep reactivity if needed (though runes handle objects)
        notes = [...notes];
      }
    } catch (err) {
      console.error("AI Error:", err);
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
      updatedAt: Date.now()
    };
    notes = [newNote, ...notes];
    activeNoteId = newNote.id;
    isPreview = false;
  }

  function deleteNote(id: string, event: MouseEvent) {
    event.stopPropagation();
    if (notes.length <= 1) {
        // Just clear the note instead of deleting the last one
        const index = notes.findIndex(n => n.id === id);
        if (index !== -1) {
            notes[index].content = "";
            notes[index].title = "Untitled Note";
            notes[index].summary = "";
            notes[index].updatedAt = Date.now();
        }
        return;
    }
    notes = notes.filter(n => n.id !== id);
    if (activeNoteId === id) {
      activeNoteId = notes[0].id;
    }
  }

  function updateContent(val: string) {
    const index = notes.findIndex(n => n.id === activeNoteId);
    if (index !== -1) {
      notes[index].content = val;
      notes[index].updatedAt = Date.now();
    }
  }
</script>

<div class="h-screen w-full bg-[#050505] text-zinc-400 font-inter flex overflow-hidden">
  <!-- Sidebar -->
  <aside class="w-72 border-r border-white/5 flex flex-col bg-[#080808]">
    <div class="p-4 flex items-center justify-between">
      <div class="flex items-center gap-2 text-white font-semibold tracking-tight">
        <div class="w-6 h-6 bg-indigo-500 rounded-md flex items-center justify-center">
          <FileText size={14} class="text-white" />
        </div>
        <span>BrainNotes</span>
      </div>
      <Button variant="ghost" size="icon" class="h-8 w-8 hover:bg-white/5" onclick={createNewNote}>
        <Plus size={18} />
      </Button>
    </div>

    <div class="px-4 mb-4">
      <div class="relative">
        <Search class="absolute left-3 top-1/2 -translate-y-1/2 text-zinc-600" size={14} />
        <input 
          bind:value={searchQuery}
          placeholder="Search notes..." 
          class="w-full bg-white/5 border border-white/10 rounded-lg py-2 pl-9 pr-4 text-xs focus:outline-none focus:border-indigo-500/50 transition-colors"
        />
      </div>
    </div>

    <div class="flex-1 overflow-y-auto px-2 space-y-1">
      {#each filteredNotes as note (note.id)}
        <div in:fade={{ duration: 200 }} out:slide={{ duration: 150 }}>
          <div 
            role="button"
            tabindex="0"
            class="w-full text-left p-3 rounded-xl transition-all group relative cursor-pointer {activeNoteId === note.id ? 'bg-white/5 text-white shadow-sm ring-1 ring-white/10' : 'hover:bg-white/[0.02] text-zinc-500'}"
            onclick={() => activeNoteId = note.id}
            onkeydown={(e) => e.key === 'Enter' && (activeNoteId = note.id)}
          >
            <div class="font-medium text-sm truncate pr-6">{note.title || "Untitled"}</div>
            <div class="text-[11px] text-zinc-600 mt-1 truncate">
              {note.summary || note.content.substring(0, 40) || "Empty note..."}
            </div>
            
            <button 
              class="absolute right-2 top-1/2 -translate-y-1/2 opacity-0 group-hover:opacity-100 p-1 hover:text-red-400 transition-all z-20"
              onclick={(e) => deleteNote(note.id, e)}
            >
              <Trash2 size={14} />
            </button>
          </div>
        </div>
      {/each}
    </div>
  </aside>

  <!-- Main Content -->
  <main class="flex-1 flex flex-col min-w-0 relative">
    <header class="h-16 border-b border-white/5 flex items-center justify-between px-6 backdrop-blur-md bg-black/20 z-10">
      <div class="flex flex-col min-w-0 flex-1 mr-4">
        <input 
          value={activeNote?.title}
          oninput={(e) => {
            const index = notes.findIndex(n => n.id === activeNoteId);
            if (index !== -1) notes[index].title = e.currentTarget.value;
          }}
          class="bg-transparent border-none text-white font-medium text-lg tracking-tight focus:outline-none w-full"
          placeholder="Note Title"
        />
      </div>
      
      <div class="flex items-center gap-2">
        <div class="flex bg-white/5 p-1 rounded-lg border border-white/10 mr-2">
          <Button 
            variant="ghost" 
            size="sm" 
            class="h-7 px-3 text-[10px] uppercase tracking-widest {!isPreview ? 'bg-white/10 text-white' : ''}"
            onclick={() => isPreview = false}
          >
            <Edit3 size={12} class="mr-1.5" /> Edit
          </Button>
          <Button 
            variant="ghost" 
            size="sm" 
            class="h-7 px-3 text-[10px] uppercase tracking-widest {isPreview ? 'bg-white/10 text-white' : ''}"
            onclick={() => isPreview = true}
          >
            <Eye size={12} class="mr-1.5" /> Preview
          </Button>
        </div>

        <Button 
          variant="ghost" 
          size="sm" 
          onclick={handleAnalyze}
          disabled={isAnalyzing}
          class="h-9 px-4 text-[11px] uppercase tracking-widest hover:bg-indigo-500/10 border border-indigo-500/20 text-indigo-400 group"
        >
          <Sparkles size={14} class="mr-2 group-hover:animate-pulse" />
          {isAnalyzing ? "Analyzing..." : "AI Sync"}
        </Button>
      </div>
    </header>

    <section class="flex-1 overflow-y-auto bg-[#050505] p-8">
      <div class="max-w-4xl mx-auto space-y-6">
        {#if activeNote?.summary}
          <div 
            in:fade
            class="p-4 bg-indigo-500/5 border border-indigo-500/20 rounded-xl text-zinc-400 text-sm leading-relaxed flex gap-3"
          >
            <div class="mt-0.5 text-indigo-400 shrink-0">
              <Sparkles size={16} />
            </div>
            <div>
              <span class="text-[10px] uppercase tracking-[0.2em] text-indigo-400/60 font-bold block mb-1">AI Summary</span>
              {activeNote.summary}
            </div>
          </div>
        {/if}

        <div class="min-h-[500px]">
          {#if isPreview}
            <article class="prose prose-invert prose-zinc max-w-none prose-headings:text-white prose-a:text-indigo-400">
              {@html marked(activeNote?.content || "")}
            </article>
          {:else}
            <Textarea 
              value={activeNote?.content}
              oninput={(e) => updateContent(e.currentTarget.value)}
              placeholder="Start writing in Markdown..."
              class="w-full h-full bg-transparent border-none focus-visible:ring-0 text-lg leading-relaxed text-zinc-200 resize-none placeholder:text-zinc-800 p-0 min-h-[70vh]"
            />
          {/if}
        </div>
      </div>
    </section>

    <!-- Bottom Info -->
    <footer class="h-8 border-t border-white/5 flex items-center justify-between px-6 text-[10px] text-zinc-600 bg-black/40">
      <div class="flex gap-4">
        <span>Words: {activeNote?.content.split(/\s+/).filter(Boolean).length || 0}</span>
        <span>Characters: {activeNote?.content.length || 0}</span>
      </div>
      <div>
        Last saved: {new Date(activeNote?.updatedAt || 0).toLocaleTimeString()}
      </div>
    </footer>
  </main>
</div>

<style>
  :global(.prose) {
    font-family: 'Inter', sans-serif;
    color: #a1a1aa; /* zinc-400 */
  }
  :global(.prose h1, .prose h2, .prose h3) {
    color: #ffffff;
    font-weight: 600;
    margin-top: 1.5em;
    margin-bottom: 0.5em;
  }
  :global(.prose p) {
    margin-bottom: 1.25em;
    line-height: 1.75;
  }
  :global(.prose code) {
    background: rgba(255, 255, 255, 0.05);
    padding: 0.2em 0.4em;
    border-radius: 4px;
    font-size: 0.9em;
    color: #e2e2e2;
  }
  
  /* Smooth scrollbars */
  ::-webkit-scrollbar {
    width: 6px;
  }
  ::-webkit-scrollbar-track {
    background: transparent;
  }
  ::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.05);
    border-radius: 10px;
  }
  ::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 255, 255, 0.1);
  }

  :global(body) {
    background: #050505;
    margin: 0;
    overflow: hidden;
  }
</style>