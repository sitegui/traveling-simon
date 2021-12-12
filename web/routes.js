const express = require('express')
const router = express.Router()
const GOOGLE_API_KEY = process.env.GOOGLE_API_KEY
const axios = require('axios')
const child_process = require('child_process')

if (!GOOGLE_API_KEY) {
  throw new Error('Missing GOOGLE_API_KEY')
}

// Receives an object like:
// {origins: {latitude: Number, longitude: Number}], destinations: {latitude: Number, longitude: Number}]}
// and returns an object like:
// {rideDurations: [[String]]}
router.post('/api/ride-durations', async (req, res, next) => {
  try {
    // Format like expected by Google's API:
    const origins = req.body.origins.map(origin => `${origin.latitude},${origin.longitude}`).join('|')
    const destinations = req.body.destinations.map(destination => `${destination.latitude},${destination.longitude}`).join('|')

    const answer = await axios.get('https://maps.googleapis.com/maps/api/distancematrix/json', {
      params: {
        destinations,
        origins,
        key: GOOGLE_API_KEY
      }
    })

    if (answer.data.status !== 'OK') {
      throw new Error(`Got unexpected status: ${answer.data.status}`)
    }

    const rideDurations = []
    for (const row of answer.data.rows) {
      const subRideDurations = []
      for (const element of row.elements) {
        if (element.status === 'OK') {
          subRideDurations.push(formatDuration(element.duration.value, 300))
        } else {
          subRideDurations.push(null)
        }
      }
      rideDurations.push(subRideDurations)
    }

    res.json({
      rideDurations
    })
  } catch (error) {
    next(error)
  }
})

// Proxy the call to the engine binary
router.post('/api/calculate-paths', (req, res, next) => {
  try {
    const child = child_process.execFile('../data/traveling-simon', (err, stdout, stderr) => {
      if (err) {
        res.status(500)
        return res.json({
          error: stderr
        })
      }

      res.json(JSON.parse(stdout))
    })
    child.stdin.end(JSON.stringify(req.body))
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
