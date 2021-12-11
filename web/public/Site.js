class Site {
  constructor (name, latitude, longitude) {
    this.name = name
    this.latitude = latitude
    this.longitude = longitude
    this.marker = null
    this.serviceTimeMinutes = 5
    this.visit = Site.VISIT_ALWAYS
    this.duties = []
  }
}

Site.VISIT_ALWAYS = 'always'
Site.VISIT_MAYBE = 'maybe'
Site.VISIT_NEVER = 'never'
