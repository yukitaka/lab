<script>
  import svelteLogo from './assets/svelte.svg'
  import Counter from './lib/Counter.svelte'
  import Home from '@/pages/Home.svelte'
  import Header from '@/components/Header.svelte'
</script>

<Header />

<Home />

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, 'Segos UI', Roboto, Oxygen,
    Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
    height: 100%;
  }
  :global(body) {
    margin: 0;
    height: 100%;
    background-color: #fbfbfb;
  }
</style>
