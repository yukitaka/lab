<script lang="ts">
  import Fa from 'svelte-fa/src/fa.svelte'
  import { faPlus } from '@fortawesome/free-solid-svg-icons'
  import type { NoteType } from '@/types/app'

  import Note from '@/components/Note.svelte'
  import EditNoteModal from '@/components/EditNoteModal.svelte'
  import DeleteNoteModal from '@/components/DeleteNoteModal.svelte'

  let notesJSONString: string = localStorage.getItem('notes')
  let notes: Array<NoteType> = []

  if (notesJSONString) {
    try {
      notes = JSON.parse(notesJSONString) as NoteType[]
    } catch(err) {
      console.error(err)
    }
  } else {
    notes = [
      {
        id: 1,
        title: 'Some Note',
        content: 'This note is about this and that',
        date: '202211072324',
        isFavorite: false,
        tags: ['test', 'text']
      },
      {
        id: 2,
        title: 'Another Note',
        content: 'This is yet another note about this and that',
        date: '20210806133021',
        isFavorite: false,
        tags: ['test', 'text', 'extra']
      },
      {
        id: 3,
        title: 'Lorem ipsum',
        content: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Morbi in bibendum tellus. Aenean bibendum purus id turpis hendrerit tristique vitae nec lacus. Nam viverra elementum nisl. Donec non tempus arcu. Etiam iaculis ex nec hendrerit tincidunt. Vivamus elementum velit dui, non hendrerit purus condimentum ut. In convallis mauris eget consequat tempus. Pellentesque auctor a massa ac suscipit. Donec id metus suscipit, interdum tortor at, semper massa. Donec eget finibus purus, sed varius mi. Etiam a dui eget tortor auctor pulvinar eu ac lectus. Duis non diam molestie, efficitur massa non, suscipit massa. Etiam dictum eros quis ullamcorper scelerisque. Donec fermentum id ipsum vitae ullamcorper. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Cras pharetra diam sed fringilla dictum.',
        date: '20210804133021',
        tags: ['Lorem', 'ipsum']
      }
    ]
  }

  let noteToEdit: NoteType | Record<string, unknown> | undefined
  let showEditModal = false

  const openEditNote = (note?: NoteType) => {
    noteToEdit = {}

    if (note) {
      noteToEdit = note
    }

    showEditModal = true
  }

  const closeEditModal = () => {
    showEditModal = false
  }

  let noteToDelete: NoteType | Record<string, unknown> | undefined
  let showDeleteModal = false

  const openDeleteNote = (event: CustomEvent) => {
    const deleteNoteIndex = event.detail as number
    const noteIndex = notes.findIndex(item => item.id === deleteNoteIndex)

    if (noteIndex !== -1) {
      noteToDelte = notes[noteindex]
      showDeleteModal = true
    }
  }

  const closeDeleteModal = () => {
    noteToDelete = {}
    showDeleteModal = false
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

  const saveNote = (event: CustomEvent) => {
    closeEditModal()
  }

  const deleteNote = (event: CustomEvent) => {
    closeDeleteModal()
    saveNotesToStorage()
  }
</script>

<main>
  <div class="note-card-container">
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div class="note-card-add" on:click="{() => { openEditNote() }}">
      <Fa icon={faPlus} color="#afaeae" size="3x" />
    </div>

    {#each notes as note (note.id)}
      <Note
        {...note}
        on:click="{() => { openEditNote(note) }}"
        on:toggleFavorite="{toggleFavorite}"
      />
    {/each}
  </div>
</main>

{#if showEditModal}
  <EditNoteModal
    {...noteToEdit}
    on:save="{saveNote}"
    on:delete="{openDeleteNote}"
    on:close="{closeEditModal}"
  />
{/if}

{#if showDeleteModal}
  <DeleteNoteModal
    {...noteToDelete}
    on:delete="{deleteNote}"
    on:close="{closeDeleteModal}"
  />
{/if}

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
