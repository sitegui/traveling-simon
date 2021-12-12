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

  if (response.ok) {
    return response.json()
  } else {
    const contentType = response.headers.get('Content-Type')
    if (contentType !== null && contentType.includes('application/json')) {
      const body = await response.json()
      throw new Error(`API request failed with status ${response.status}:\n${body.error}`)
    } else {
      const body = await response.text()
      throw new Error(`API request failed with status ${response.status}:\n${body}`)
    }
  }
}

window.hub = new Hub()
