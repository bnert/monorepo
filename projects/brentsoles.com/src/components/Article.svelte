<script>
  import { parse } from '../pkgs/mds'
  export let mdId;
  export let mdStr = "### Heading for an article\n```\nThis is a paragraph\nWith some more text```"
</script>

<div>
  {@html parse(mdStr) }
</div>
