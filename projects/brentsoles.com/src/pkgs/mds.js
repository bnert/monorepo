const grammar = {
  el: 'directive content',
  directive: {
    '': {
      tag: 'p',
      plural: false,
      term: /\n\n/,
      pre: /\n/,
      post: /\n/
    },
    '#': {
      tag: 'h',
      plural: true,
      term: /\s/,
      pre: /[\n]{0,1}/,
      post: /[\#\s]/
    },
    '`': {
      tag: 'code',
      plural: true,
      term: /\n/,
      pre: /\n/,
      post: /[\`\s]/
    }
  }
}

const isDirective = (char, pre, post) => {
  const directive = grammar.directive[char]
  if(!directive) return false

  return directive.pre.test(pre) && directive.post.test(post)
}

const parse = (
  mdString
) => {
  let htmlString = ''

  for(let i = 0; i < mdString.length; i++) {
    let ch = mdString[i]
    let pre = mdString[i - 1] // For first, will be undefined
    let post = mdString [i + 1]
    let inc = 0
    if(isDirective(ch, pre, post)) {
      let { tag, plural, term } = grammar.directive[ch]
      if(plural) {
        while(!term.test(mdString[i])) {
          inc++
          i++
        }
      }

      htmlString += `<${tag}${plural ? inc : ''}>`
      i++

      do {
        htmlString += mdString[i]
        i++
      } while(mdString[i] !== '\n' && i < mdString.length)

      htmlString += `</${tag}${plural ? inc : ''}>`
      if (mdString[i] === '\n') {
        htmlString += mdString[i]
      }
    }

    if(mdString[i] !== undefined) {
      htmlString += mdString[i]
    }
  }
  return htmlString
}

module.exports = {
  parse
}
