const ferrisTypes = [
  {
    attr: 'does_not_compile',
    title: 'This code does not compile!'
  }
]

document.addEventListener('DOMContentLoaded', () => {
  for (const ferrisType of ferrisTypes) {
    attachFerrises(ferrisType)
  }
})

function attachFerrises(type) {
  const elements = document.getElementsByClassName(type.attr)

  for (const codeBlock of elements) {
    const lines = codeBlock.textContent.split(/\r|\r\n|\n/).length - 1;

    if (lines >= 4) {
      attachFerris(codeBlock, type)
    }
  }
}

function attachFerris(element, type) {
  const img = document.createElement('img')
  img.setAttribute('src', 'img/' + type.attr + '.svg')
  img.setAttribute('title', type.title)
  img.className = 'ferris'

  element.parentElement.insertBefore(img, element)
}
