<script lang="ts">
  import Fa from 'svelte-fa/src/fa.svelte'
  import { faStar } from '@fortawesome/free-solid-svg-icons'
  import { createEventDispatcher } from 'svelte'
  import { formatDate } from '@/libs/utils'

  export let id: number
  export let title: string
  export let content: string
  export let date: string
  export let isFavorite: boolean
  export let tags: string[]

  const dispatch = createEventDispatcher();

  const trimContent = (content: string) => {
    if (content.length > 100) {
      return `${content.substring(0, 100)}...`
    }

    return content
  }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div class="note-card" on:click>
  <div class="title">{title}</div>
  <div class="preview">{trimContent(content)}</div>

  <div class="tag-wrapper">
  {#each tags as tag (tag)}
    <div class="tag">{tag}</div>
  {/each}
  </div>

  <div class="card-footer">
    <div class="date">{formatDate(date)}</div>
    <div class="fav-icon" on:click|stopPropagation="{() => dispatch('toggleFavorite', id)}">
      <Fa icon={faStar} color="{isFavorite ? '#ffda00' : '#afaeae'}" />
    </div>
  </div>
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

    .tag {
      background-color: #d6d2d2;
      color: #d60000;
      padding: 2px 10x;
      border-radius: 20px;
      font-size: 12px;
      margin-right: 5px;
      height: 20px;

      &-wrapper {
        display: flex;
        align-items: flex-end;
        flex-wrap: wrap;
      }
    }

    .card-footer {
      display: flex;
      justify-content: space-between;
      align-items: flex-end;

      .date {
        color: #afaeae;
        font-size: 14px;
      }
    }
    
    .fav-icon {
      cursor: pointer;
    }
  }
</style>
