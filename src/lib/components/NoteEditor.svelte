<script lang="ts">
  import type { Note } from "$lib/types";
  import { Button } from "$lib/components/ui/button";
  import { Textarea } from "$lib/components/ui/textarea";
  import { marked } from "marked";
  import { Sparkles, Eye, Edit3, Clock } from "lucide-svelte";
  import { fade } from "svelte/transition";

  let {
    note = $bindable(),
    isPreview = $bindable(),
    isAnalyzing,
    handleAnalyze
  }: {
    note: Note;
    isPreview: boolean;
    isAnalyzing: boolean;
    handleAnalyze: () => void;
  } = $props();
</script>

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
