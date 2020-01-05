<script>
  const github = 'https://github.com/brent-soles'
  const skills = [
    {
      lang: 'c++',
      xp: '3 years',
      lvl: 'Experienced',
      tools: [
        'clang',
        'gcc',
        'gdb',
        'gtest',
        'cunit'
      ],
      pkgs: [
        {
          name: 'flags',
          desc: 'Implementation of go "flags" package in C++',
          href: '#'
        }
      ]
    },
    {
      lang: 'js',
      xp: '2 years',
      lvl: 'Experienced',
      tools: [
        'react', 'vue', 'svelte',
        'express', 'koa', 'Apollo GraphQL'
      ],
      pkgs: [
        {
          name: 'funnl',
          desc: 'Pipe function utility (w/ async)',
          href: '#'
        }
      ]
    },
    {
      lang: 'git',
      xp: '4 years',
      lvl: 'Competent',
      tools: [
        'tig'
      ]
    },
    {
      lang: 'linux',
      xp: '4 years',
      lvl: 'Competent',
      tools: [
        'grep', 'tmux', 'vim', 'ps', 'htop'
      ]
    },
    {
      lang: 'python',
      xp: '1 year',
      lvl: 'Competent',
      tools: [
        'pip', 'virtualenv'
      ]
    },
    {
      lang: 'go',
      xp: '1 year',
      lvl: 'Competent',
      tools: [
        'make', 'gorilla/mux', 'gorilla/router'
      ]
    },
    {
      lang: 'docker',
      xp: '1 year',
      lvl: 'Beginner'
    },
    {
      lang: 'elixir',
      xp: '6 months',
      lvl: 'Beginner',
      tools: [
        'mix', 'absinthe GraphQL'
      ]
    },
    {
      lang: 'C#',
      xp: '6 months',
      lvl: 'Beginner',
    },
    {
      lang: 'rust',
      xp: '6 months',
      lvl: 'Beginner',
      tools: [
        'hyper'
      ]
    }
  ]
</script>

<style>
  #skills {
    display: flex;
    flex-direction: column;
    justify-content: center;
    width: 100%;
    margin: 0;
  }

  #skills > h2 {
    text-align: center;
  }

  #skills .body-container {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    width: 50%;
    margin: 0 auto;
  }

  .card {
    padding: 1rem 2rem;
    border: 1px solid rgba(0, 0, 0, .1);
    border-radius: 6px;
    margin: 1rem;
    transition: all .2s ease-in-out;
  }

  .card:hover {
    box-shadow: 0px 6px 12px rgba(0, 0, 0, .2);
  }

  .card__heading {
    margin: 0 0 1em 0;
  }

  .card__subheading {
    font-weight: semi-bold;
  }

  .card > .card__body {
    margin: 0;
    padding: 0;
  }

  .card > .card__body {
    display: flex;
    flex-wrap: wrap;
  }

  .card > .card__body > li {
    cursor: pointer;
    border: 1px solid rgba(0, 0, 0, .1);
    border-radius: 6px;
    display: inline;
    list-style-type: none;
    margin: .25em;
    padding: .25em .75em;
  }

  .card > .card__body > li:hover {
    background: rgba(0, 0, 0, .1);
    color: #fff;
    transition: all .2s ease-in-out;
  }

  @media (max-width: 1250px) {
    #skills .body-container {
      grid-template-columns: repeat(2, 1fr);
    }
  }

  @media (max-width: 1024px) {
    #skills .body-container {
      width: 75%;
    }

    img {
      width: 14rem;
    }
  }

  @media (max-width: 768px) {
    #skills .body-container {
      width: 100%;
    }
  }

  @media (max-width: 550px) {
     #skills .body-container {
      grid-template-columns: repeat(1, 1fr);
    }
  }


</style>

<section id="skills">
  <h2>Skills</h2>
  <div class="body-container">
    {#each skills as skill}
      <div class="card skill-card">
        <h2 class="card__heading">{ skill.lang }</h2>
        <p class="card__subheading"># years: { skill.xp }</p>
        <p class="card__subheading">Comptency: { skill.lvl }</p>
        {#if skill.tools}
          <span>Tools:</span>
          <ul class="card__body">
            {#each skill.tools as tool}
              <li>{ tool }</li>
            {/each}
          </ul>
        {/if}
      </div>
    {/each}
  </div>
</section>
