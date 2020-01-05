<script>
</script>

<style>
  a {
    color: rgba(0, 0, 0, .8);
  }

  #projects {
    align-items: center;
    display: flex;
    flex-direction: column;
  }

  #projects > h2 {
    text-align: center;
  }

  ul {
    list-style-type: none;
    width: calc(50% - 2rem);
    margin: 0;
    padding: 0;
  }

  li {
    display: grid;
    box-sizing: border-box;
    border: 1px solid rgba(0, 0, 0, .1);
    border-radius: 6px;
    margin: 1rem 0;
    padding: 1rem 1.25rem;
    transition: all .2s ease-in-out;
  }

  li:hover {
    box-shadow: 0px 6px 12px rgba(0, 0, 0, .2);
  }

  img {
    border-radius: 6px;
    display: flex;
    justify-self: flex-end;
    grid-row-start: 1;
    grid-row-end: 3;
    grid-column: 2;
    width: 16rem;
    transition: all .2s ease-in-out;
  }

  li:hover > img {
    box-shadow: 0px 8px 12px rgba(0, 0, 0, .2);
    transform: scale(1.25) translate(1rem, 1rem);
  }

  @media (max-width: 1440px) {
    img {
      width: 14rem;
    }
  }

  @media (max-width: 1024px) {
    ul {
      width: calc(75% - 2rem);
    }
  }

  @media (max-width: 768px) {
    ul {
      width: calc(100% - 2rem);
    }
  }

  @media (max-width: 425px) {
    img {
      justify-self: center;
      grid-row-start: 1;
      grid-row-end: 1;
      grid-column: 1;
    }
  }

  @media (max-width: 320px) {
    img {
      width: 13rem;
    }
  }
</style>

<section id="projects">
  <h2>Projects</h2>
  <ul>
    <li>
      <h3 id="rmnd.dev">
        <a href="#rmnd.dev">rmnd.dev</a>
      </h3>
      <p>A cli based productivity app which sends reminders/todos via text/email</p>
    </li>
    <li>
      <h3 id="followers-collective">
        <a href="#followers-collective">followers collective</a>
      </h3>
      <p>A community of christian creatives</p>
      <img src="/images/followers-collective-rev-1.jpg" alt="img">
    </li>
  </ul>
</section>
