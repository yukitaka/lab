<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte'
  import Modal from '@/components/Modal.svelte'

  export let title: string | undefined = undefined
  export let content: string | undefined = undefined

  $: canSave = Boolean(title && content)

  const dispatch = createEventDispatcher()

  const saveNote = () => {
    if (!canSave) return
  }

  let textarea: HTMLElement

  const onInput = () => {
    if (textarea) {
        textarea.style.height = 'auto'
        textarea.style.height = `${textarea.scrollHeight}px`
    }
  }

  onMount(() => {
    onInput()
  })
</script>

<Modal>
  <div slot="body" class="modal-body">
    <textarea
      id="note-content"
      bind:this={textarea}
      bind:value={content}
      class="input"
      type="textarea"
      on:input="{onInput}"
    />
  </div>

  <div slot="footer" class="modal-footer">
    <button
      class="button save {!canSave? 'disabled' : ''}"
      on:click|stopPropagation="{saveNote}"
    >
      Save
    </button>
  </div>
</Modal>

<style lang="scss">
  .modal {
    &-body {
      width: 700px;
      padding: 20px 15px 10px;
      display: grid;
      grid-template-columns: 1fr 3fr;
      gap: 15px 0;
    }

    &-footer {
      padding: 0 15px;
      display: grid;
      grid-template-columns: 1fr 1fr;

      .button {
        height: 30px;
        padding: 0 10px;
        text-align: center;
        box-sizing: content-box;
        border-radius: 3px;
        border: 1px solid #000;

        &:active {
          background-color: #b9b7b7;
        }
      }
    }
  }
</style>
