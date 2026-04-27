<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { Textarea } from "$lib/components/ui/textarea";
  import { Button } from "$lib/components/ui/button";

  // Svelte 5 Runes for state management
  let content = $state("");
  let title = $state("New Note");
  let summary = $state("");
  let isAnalyzing = $state(false);

  async function handleAnalyze() {
    if (!content) return;
    isAnalyzing = true;
    try {
      // Calling your private core-ai logic
      const result: { title: string, summary: string } = await invoke("analyze_note_content", { content });
      title = result.title;
      summary = result.summary;
    } catch (err) {
      console.error("AI Error:", err);
    } finally {
      isAnalyzing = false;
    }
  }
</script>

<main class="h-screen w-full bg-[#050505] text-zinc-400 font-inter flex flex-col">
  <header class="h-16 border-b border-white/5 flex items-center justify-between px-6 backdrop-blur-md bg-black/20">
    <div class="flex flex-col">
      <span class="text-white font-medium text-sm tracking-tight">{title}</span>
      {#if summary}
        <span class="text-[10px] text-zinc-500 uppercase tracking-widest">{summary}</span>
      {/if}
    </div>
    
    <Button 
      variant="ghost" 
      size="sm" 
      onclick={handleAnalyze}
      disabled={isAnalyzing}
      class="text-[11px] uppercase tracking-widest hover:bg-white/5 border border-white/10"
    >
      {isAnalyzing ? "Processing..." : "AI Sync"}
    </Button>
  </header>

  <section class="flex-1 p-6 overflow-hidden">
    <div class="max-w-4xl mx-auto h-full glass-dark rounded-2xl p-4 shadow-2xl relative">
      <Textarea 
        bind:value={content}
        placeholder="Start writing..."
        class="w-full h-full bg-transparent border-none focus-visible:ring-0 text-lg leading-relaxed text-zinc-200 resize-none placeholder:text-zinc-700"
      />
    </div>
  </section>
</main>

<style>
  /* Ensuring the backdrop blur works for that Mercedes-Benz/Glass feel */
  :global(body) {
    background: #050505;
    margin: 0;
  }
</style>