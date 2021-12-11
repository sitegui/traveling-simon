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

async function postApi (url, body) {
  const response = await fetch(url, {
    method: 'POST',
    body: JSON.stringify(body),
    headers: {
      'Content-Type': 'application/json'
    }
  })

  return response.json()
}

window.hub = new Hub()
