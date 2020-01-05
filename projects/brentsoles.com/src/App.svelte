<script>
  import Projects from './components/Projects.svelte'
  import Skills from './components/Skills.svelte'
  export let name;
</script>

<style>
  h1 {
    text-align: center;
  }

  :global(section) {
    margin-bottom: 4rem;
  }

  .links {
    display: flex;
    justify-content: center;
  }

  .links a {
    margin: 0 1rem;
  }
</style>

<section>
  <h1>Brent Soles</h1>
  <div class="links">
    <a href="https://github.com/brent-soles">github</a>
    <a href="mailto:brentsoles@protonmail.com">email</a>
  </div>
</section>
<Projects />
<Skills />
