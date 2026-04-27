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

  type Note = {
    id: string;
    title: string;
    content: string;
    summary: string;
    updatedAt: number;
  };

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
  <!-- Sidebar -->
  <aside class="w-72 border-r border-white/5 flex flex-col bg-[#080808]">
    <div class="p-4 flex items-center justify-between">
      <div class="flex items-center gap-2 text-white font-semibold">
        <div class="w-6 h-6 bg-indigo-500 rounded-md flex items-center justify-center">
          <FileText size={14} class="text-white" />
        </div>
        <span>BrainNotes</span>
      </div>
      <Button variant="ghost" size="icon" class="h-8 w-8 hover:bg-white/5" onclick={createNewNote}>
        <Plus size={18} />
      </Button>
    </div>

    <div class="px-4 mb-4 relative">
      <Search class="absolute left-7 top-1/2 -translate-y-1/2 text-zinc-600" size={14} />
      <input
        bind:value={searchQuery}
        placeholder="Search..."
        class="w-full bg-white/5 border border-white/10 rounded-lg py-2 pl-9 pr-4 text-xs focus:outline-none focus:border-indigo-500/50"
      />
    </div>

    <div class="flex-1 overflow-y-auto px-2 space-y-1">
      {#each filteredNotes as note (note.id)}
        <div in:fade={{ duration: 200 }} out:slide={{ duration: 150 }}>
          <div 
            role="button"
            tabindex="0"
            class="w-full text-left p-3 rounded-xl transition-all group relative cursor-pointer {activeNoteId === note.id ? 'bg-white/5 text-white ring-1 ring-white/10' : 'text-zinc-500 hover:bg-white/[0.02]'}"
            onclick={() => { activeNoteId = note.id; isPreview = false; }}
            onkeydown={(e) => e.key === 'Enter' && (activeNoteId = note.id)}
          >
            <div class="font-medium text-sm truncate pr-6">{note.title || "Untitled"}</div>
            <div class="text-[11px] text-zinc-600 mt-1 truncate">
              {note.summary || note.content.substring(0, 40).replace(/#|\*|`/g, "") || "No summary..."}
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
  <main class="flex-1 flex flex-col min-w-0">
    {#if notes.length > 0 && activeNoteId}
      {#each notes as note (note.id)}
        {#if note.id === activeNoteId}
          <header class="h-16 border-b border-white/5 flex items-center justify-between px-6 bg-black/20 backdrop-blur-md">
            <input
              bind:value={note.title}
              class="bg-transparent border-none text-white font-medium text-lg focus:outline-none flex-1"
              placeholder="Note Title"
            />
            <div class="flex items-center gap-2">
              <div class="flex bg-white/5 p-1 rounded-lg border border-white/10">
                <Button variant="ghost" size="sm" class="h-7 text-[10px] {!isPreview ? 'bg-white/10' : ''}" onclick={() => (isPreview = false)}>
                  <Edit3 size={12} class="mr-1" /> Edit
                </Button>
                <Button variant="ghost" size="sm" class="h-7 text-[10px] {isPreview ? 'bg-white/10' : ''}" onclick={() => (isPreview = true)}>
                  <Eye size={12} class="mr-1" /> Preview
                </Button>
              </div>
              <Button
                variant="ghost"
                size="sm"
                onclick={handleAnalyze}
                disabled={isAnalyzing}
                class="h-9 px-4 text-[11px] border border-indigo-500/20 text-indigo-400 group"
              >
                <Sparkles size={14} class="mr-2 {isAnalyzing ? 'animate-spin' : 'group-hover:animate-pulse'}" />
                {isAnalyzing ? "Syncing..." : "AI Sync"}
              </Button>
            </div>
          </header>

          <section class="flex-1 overflow-y-auto p-8">
            <div class="max-w-4xl mx-auto space-y-6">
              {#if note.summary}
                <div in:fade class="p-4 bg-indigo-500/5 border border-indigo-500/20 rounded-xl text-sm flex gap-3 shadow-[0_0_20px_-10px_rgba(99,102,241,0.3)]">
                  <Sparkles size={16} class="text-indigo-400 shrink-0" />
                  <div>
                    <span class="text-[10px] uppercase tracking-wider text-indigo-400/60 font-bold block mb-1">AI Summary</span>
                    <p class="text-zinc-300">{note.summary}</p>
                  </div>
                </div>
              {/if}
              <div class="min-h-[500px]">
                {#if isPreview}
                  <article class="prose prose-invert max-w-none" in:fade>
                    {@html marked(note.content || "")}
                  </article>
                {:else}
                  <Textarea
                    bind:value={note.content}
                    placeholder="Start writing..."
                    class="w-full h-full bg-transparent border-none focus-visible:ring-0 text-lg text-zinc-200 resize-none min-h-[70vh] p-0"
                  />
                {/if}
              </div>
            </div>
          </section>

          <footer class="h-8 border-t border-white/5 flex items-center justify-between px-6 text-[10px] text-zinc-600 bg-black/40">
            <div class="flex gap-4">
              <span>Words: {note.content.split(/\s+/).filter(Boolean).length || 0}</span>
              <span>Chars: {note.content.length || 0}</span>
            </div>
            <div class="flex items-center gap-2">
              <Clock size={10} />
              Last update: {new Date(note.updatedAt).toLocaleTimeString()}
            </div>
          </footer>
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