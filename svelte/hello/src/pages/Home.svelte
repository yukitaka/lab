<script lang="ts">
  import Fa from 'svelte-fa/src/fa.svelte'
  import { faPlus } from '@fortawesome/free-solid-svg-icons'
  import type { NoteType } from '@/types/app'
</script>

<main>
  <div class="note-card-container">
    <div class="note-card-add">
      <Fa icon={faPlus} color="#afaeae" size="3x" />
    </div>
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
