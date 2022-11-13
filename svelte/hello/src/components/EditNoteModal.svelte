<script lang="ts">
import moment from 'moment'
  import { createEventDispatcher, onMount } from 'svelte'
  import Modal from '@/components/Modal.svelte'

  export let title: string | undefined = undefined
  export let date: string | undefined = undefined
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

<Modal on:closeModal="{() => dispatch('close')}">
  <div slot="body" class="modal-body">
    <textarea
      id="note-content"
      bind:this={textarea}
      bind:value={content}
      class="input"
      type="textarea"
      on:input="{onInput}"
    />

    {#if date}
      <div class="label">Last Updated:</div>
      <div class="text">{ formatDate(date) }</div>
    {/if}
  </div>

  <div slot="footer" class="modal-footer">
    <div class="buttons-wrapper">
      <button
        class="button save {!canSave? 'disabled' : ''}"
        on:click|stopPropagation="{saveNote}"
      >
        Save
      </button>
      <button
        class="button"
        on:click|stopPropagation="{() => dispatch('close')}"
      >
        Cancel
      </button>
    </div>
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

      .label {
        grid-column: 1;
        line-height: 30px;
      }

      .text {
        height: 30px;
        color: #808080;
        display: flex;
        align-items: center;
      }
    }

    &-footer {
      padding: 0 15px;
      display: grid;
      grid-template-columns: 1fr 1fr;

      .buttons-wrapper {
        text-align: right;
      }

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

  #note-content {
    resize: vertical;
    height: fit-content;
    min-height: 30px;
  }
</style>
