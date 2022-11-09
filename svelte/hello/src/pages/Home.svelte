<script lang="ts">
  import Fa from 'svelte-fa/src/fa.svelte'
  import { faPlus } from '@fortawesome/free-solid-svg-icons'
  import type { NoteType } from '@/types/app'

  import Note from '@/components/Note.svelte'

  let notesJSONString: string = localStorage.getItem('notes')
  let notes: Array<NoteType> = []

  if (notesJSONString) {
  } else {
    notes = [
      {
        id: 1,
        title: 'Some Note',
        content: 'This note is about this and that',
        date: '202211072324',
        isFavorite: false,
        tags: ['test', 'text']
      }
    ]
  }

  const saveNotesToStorage = () => {
    notes = notes

    localStorage.setItem('notes', JSON.stringify(notes))
  }

  const toggleFavorite = (event: CustomEvent) => {
    const noteId: number = (event.detail as number)
    const note = notes.find(item => item.id === noteId)

    if (note) {
      note.isFavorite = !note.isFavorite
      saveNotesToStorage()
    }
  }
</script>

<main>
  <div class="note-card-container">
    <div class="note-card-add">
      <Fa icon={faPlus} color="#afaeae" size="3x" />
    </div>

    {#each notes as note (note.id)}
      <Note {...note} on:toggleFavorite="{toggleFavorite}" />
    {/each}
  </div>
</main>

<style lang="scss">
  main {
    padding: 2em;
    margin: 0 auto;
    width: 100vw;
    box-sizing: border-box;
  }
  .note-card {
    &-add {
      background-color: #d6d4d4;
      display: flex;
      justify-content: center;
      align-items: center;
      box-shadow: none;
      border: 4px dashed #a29f9f;
      width: 144px;
      height: 189px;
      color: #a29f9f;
      margin-right: 15px;
      padding: 15px;
      border-radius: 10px;
      &:hover {
        background-color: #c5c5c5;
      }
    }
    &-container {
      display: flex;
    }
  }
</style>
