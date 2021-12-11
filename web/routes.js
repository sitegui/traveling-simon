const express = require('express')
const router = express.Router()
const GOOGLE_API_KEY = process.env.GOOGLE_API_KEY
const axios = require('axios')

if (!GOOGLE_API_KEY) {
  throw new Error('Missing GOOGLE_API_KEY')
}

router.get('/', function (req, res) {
  res.render('index', { title: 'Express' })
})

// Receives an object like:
// {sites: {name: String, latitude: Number, longitude: Number}]}
// and returns an object like:
// {sites: [{name: String, rideDurations: {$name: String}}]}
router.post('/api/ride-durations', async (req, res, next) => {
  try {
    // Format like expected by Google's API:
    console.log(req.body)
    const sites = req.body.sites.map(site => `${site.latitude},${site.longitude}`).join('|')

    const answer = await axios.get('https://maps.googleapis.com/maps/api/distancematrix/json', {
      params: {
        destinations: sites,
        origins: sites,
        key: GOOGLE_API_KEY
      }
    })

    if (answer.data.status !== 'OK') {
      throw new Error(`Got unexpected status: ${answer.data.status}`)
    }

    const convertRow = (row, i) => {
      const rideDurations = {}
      for (const [j, element] of row.elements.entries()) {
        if (i !== j && element.status === 'OK') {
          rideDurations[req.body.sites[j].name] = formatDuration(element.duration.value, 300)
        }
      }
      return {
        name: req.body.sites[i].name,
        rideDurations
      }
    }

    res.json({
      sites: answer.data.rows.map(convertRow)
    })
  } catch (error) {
    next(error)
  }
})

function formatDuration (seconds, precision) {
  seconds = precision * Math.round(seconds / precision)

  if (seconds === 0) {
    return '0s'
  }

  const s = seconds % 60
  const m = Math.floor(seconds / 60) % 60
  const h = Math.floor(seconds / 3600)
  let r = ''
  if (h !== 0) {
    r += `${h}h`
  }
  if (m !== 0) {
    r += `${m}m`
  }
  if (s !== 0) {
    r += `${s}s`
  }
  return r
}

module.exports = router
