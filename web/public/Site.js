class Site {
  constructor (name, latitude, longitude) {
    this.name = name
    this.latitude = latitude
    this.longitude = longitude
    this.marker = null
    this.serviceTimeMinutes = 15
    this.visit = Site.VISIT_ALWAYS
    this.duties = []
    this.canStartHere = false
  }
}

Site.VISIT_ALWAYS = 'ALWAYS'
Site.VISIT_MAYBE = 'MAYBE'
Site.VISIT_NEVER = 'NEVER'
