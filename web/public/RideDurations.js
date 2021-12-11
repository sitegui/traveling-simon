/* globals postApi */

class RideDurations {
  constructor () {
    // Store resulting as matrix[fromLatLngToLatLng], where the keys are the string representation of the segment
    this.rideDurations = new Map()
    this.processing = false
  }

  toJSON () {
    return {
      rideDurations: Array.from(this.rideDurations.entries())
    }
  }

  static fromJSON (data) {
    const result = new RideDurations()
    result.rideDurations = new Map(data.rideDurations)
    return result
  }

  async updateForSites (sites) {
    if (this.processing) {
      throw new Error('Already processing')
    }
    this.processing = true

    try {
      // Detect missing segments
      const missingOrigins = new Set()
      const missingDestinations = new Set()
      for (const origin of sites) {
        for (const destination of sites) {
          if (!this.rideDurations.has(this._getKey(origin, destination))) {
            missingOrigins.add(origin)
            missingDestinations.add(destination)
          }
        }
      }

      if (missingOrigins.size === 0 || missingDestinations.size === 0) {
        return
      }

      await this._update(Array.from(missingOrigins), Array.from(missingDestinations))
    } finally {
      this.processing = false
    }
  }

  get (origin, destination) {
    return this.rideDurations.get(this._getKey(origin, destination))
  }

  async _update (origins, destinations) {
    const body = {
      origins: origins.map(origin => ({
        latitude: origin.latitude,
        longitude: origin.longitude
      })),
      destinations: destinations.map(destination => ({
        latitude: destination.latitude,
        longitude: destination.longitude
      }))
    }

    const matrix = (await postApi('/api/ride-durations', body)).rideDurations

    for (const [i, origin] of origins.entries()) {
      for (const [j, destination] of destinations.entries()) {
        this.rideDurations.set(this._getKey(origin, destination), matrix[i][j])
      }
    }
  }

  _getKey (origin, destination) {
    return `${origin.latitude},${origin.longitude},${destination.latitude},${destination.longitude}`
  }
}
