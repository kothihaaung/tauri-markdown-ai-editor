<script lang="ts">
  import type { Note } from "$lib/types";
  import { Button } from "$lib/components/ui/button";
  import { Plus, FileText, Search, Trash2 } from "lucide-svelte";
  import { fade, slide } from "svelte/transition";

  let {
    filteredNotes,
    searchQuery = $bindable(),
    activeNoteId = $bindable(),
    isPreview = $bindable(),
    createNewNote,
    deleteNote
  }: {
    filteredNotes: Note[];
    searchQuery: string;
    activeNoteId: string;
    isPreview: boolean;
    createNewNote: () => void;
    deleteNote: (id: string, event: MouseEvent) => void;
  } = $props();
</script>

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
