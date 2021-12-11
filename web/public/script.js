function $ (query, scope = document) {
  return scope.querySelector(query)
}

function $$ (query, scope = document) {
  return Array.from(scope.querySelectorAll(query))
}

function show (el) {
  el.classList.remove('d-none')
}

function hide (el) {
  el.classList.add('d-none')
}

window.hub = new Hub()
