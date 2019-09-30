const { parse } = require('../pkgs/mds')

it('Parses # as h1', () => {
  const md = `# Heading 1`
  const renderedMd = `<h1>Heading 1</h1>`

  expect(parse(md)).toBe(renderedMd)
})

it('Parses ## as h2', () => {
  const md = `## Heading 2`
  const renderedMd = `<h2>Heading 2</h2>`

  expect(parse(md)).toBe(renderedMd)
})

it('Parses ## as with spaces/newlines', () => {
  const md = `
## Heading 2
`
  const renderedMd = `
<h2>Heading 2</h2>
`

  expect(parse(md)).toBe(renderedMd)
})
