<script lang="ts">
  import Fa from 'svelte-fa/src/fa.svelte'
  import { faStar } from '@fortawesome/free-solid-svg-icons'
  import { createEventDispatcher } from 'svelte'
  import { formatDate } from '@/libs/utils'

  export let title: string
  export let content: string

  const trimContent = (content: string) => {
    if (content.length > 100) {
      return `${content.substring(0, 100)}...`
    }

    return content
  }
</script>

<div class="note-card">
  <div class="title">{title}</div>
  <div class="preview">{trimContent(content)}</div>
</div>

<style lang="scss">
  .note-card {
    width: 150px;
    height: 195px;
    margin-right: 15px;
    background-color: #fff;
    padding: 15px;
    border-radius: 10px;
    box-shadow: rgb(0 0 0 / 24%) 0px 3px 8px;
    display: grid;
    grid-template-rows: 1fr 4fr 2fr 1fr;

    .title {
      color: #6f6f6f;
      font-weight: bold;
      margin-bottom: 10px;
    }

    .preview {
      color: #2f2f2f;
      font-size: 15px;
      word-break: break-word;
    }
  }
</style>
