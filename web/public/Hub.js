/* global $, $$, L, Site, show, hide, RideDurations, postApi */

class Hub {
  constructor (el) {
    this.map = L.map($('#map')).setView([47.4645927, -0.5583979], 12)
    this.sites = []
    this.editingSite = null
    this.rideDurations = new RideDurations()
    this.minStartAt = '09:00'
    this.maxEndAt = null
    this.pathPolyline = null

    // eslint-disable-next-line no-template-curly-in-string
    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
      attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors',
      maxZoom: 18
    }).addTo(this.map)

    this.map.on('click', event => this.addSite(event.latlng.lat, event.latlng.lng))

    this.showSitesPane = $('#show-sites-pane')
    this.editSitePane = $('#edit-site-pane')
    this.calculatingPathsPane = $('#calculating-paths-pane')
    this.showPathsPane = $('#show-paths-pane')
    this.panes = {
      showSites: this.showSitesPane,
      editSite: this.editSitePane,
      calculatingPaths: this.calculatingPathsPane,
      showPaths: this.showPathsPane
    }

    // Start edit-site pane
    $('#site-duty-add').onclick = event => {
      event.preventDefault()
      this.pushDutyRow('', '')
    }
    $('#edit-site-pane form').onsubmit = event => {
      event.preventDefault()
      this.saveSite()
    }
    $('#site-back').onclick = event => {
      event.preventDefault()
      this.showSites()
    }
    $('#site-remove').onclick = event => {
      event.preventDefault()
      this.removeSite()
    }

    // Start show-sites pane
    $('#calculate-paths').onclick = () => {
      this.calculatePaths()
    }
    $('#min-start-at').oninput = event => {
      this.minStartAt = event.currentTarget.value
      this.persistStorage()
    }
    $('#max-end-at').oninput = event => {
      const value = event.currentTarget.value
      this.maxEndAt = value === '' ? null : value
      this.persistStorage()
    }

    // Start show-paths pane
    $('.back', this.showPathsPane).onclick = () => {
      this.showSites()
    }

    // Reload persisted data
    const data = Hub.loadStorage()
    if (data) {
      for (const dataSite of data.sites) {
        const site = new Site(dataSite.name, dataSite.latitude, dataSite.longitude)
        site.visit = dataSite.visit
        site.serviceTimeMinutes = dataSite.serviceTimeMinutes
        site.duties = dataSite.duties
        site.canStartHere = dataSite.canStartHere
        this.updateSiteMarker(site)
        this.sites.push(site)
      }
      this.rideDurations = RideDurations.fromJSON(data.rideDurations)
      this.minStartAt = data.minStartAt
      this.maxEndAt = data.maxEndAt

      if (this.sites.length > 0) {
        const bounds = new L.LatLngBounds(this.sites.map(site => site.marker.getLatLng()))
        this.map.fitBounds(bounds)
      }
    }

    this.showSites()
  }

  switchPane (name) {
    for (const [paneName, pane] of Object.entries(this.panes)) {
      pane.classList.toggle('d-none', paneName !== name)
    }
    this.editingSite = null
    if (this.pathPolyline) {
      this.pathPolyline.remove()
      this.pathPolyline = null
    }
    this.persistStorage()
  }

  addSite (latitude, longitude) {
    const name = `Site ${this.sites.length + 1}`
    const site = new Site(name, latitude, longitude)
    this.sites.push(site)
    this.updateSiteMarker(site)
    this.editSite(site)
    this.persistStorage()
  }

  editSite (site) {
    this.switchPane('editSite')
    this.editingSite = site
    for (const otherSite of this.sites) {
      otherSite.marker.setOpacity(otherSite === site ? 1.0 : 0.5)
    }

    // Fill in the form
    $('#site-name').value = site.name
    $('#site-service-time').value = String(site.serviceTimeMinutes)
    $('#edit-site-pane form')['site-visit'].value = site.visit
    for (const el of $$('tr.site-duty')) {
      el.remove()
    }
    for (const duty of site.duties) {
      this.pushDutyRow(duty.start, duty.end)
    }
    $('#site-can-start-here').checked = site.canStartHere
  }

  pushDutyRow (start, end) {
    const newRow = this.cloneTemplate($('#site-duty-template'))
    newRow.classList.add('site-duty')
    const addRowEl = $('#site-duty-add-row')
    addRowEl.parentElement.insertBefore(newRow, addRowEl)

    $('#site-duty-start', newRow).value = start
    $('#site-duty-end', newRow).value = end
    $('#site-duty-remove', newRow).onclick = event => {
      event.preventDefault()
      newRow.remove()
    }
  }

  removeSite () {
    const index = this.sites.indexOf(this.editingSite)
    if (index !== undefined) {
      this.editingSite.marker.remove()
      this.sites.splice(index, 1)
    }
    this.showSites()
  }

  saveSite () {
    // TODO: check invariants: unique name and valid bounded time windows

    this.editingSite.name = $('#site-name').value
    this.editingSite.serviceTimeMinutes = Number.parseInt($('#site-service-time').value)
    this.editingSite.visit = $('#edit-site-pane form')['site-visit'].value
    this.editingSite.duties = []
    for (const row of $$('tr.site-duty')) {
      const start = $('#site-duty-start', row).value
      const end = $('#site-duty-end', row).value
      if (start !== '' && end !== '') {
        this.editingSite.duties.push({ start, end })
      }
    }
    this.editingSite.canStartHere = $('#site-can-start-here').checked
    this.updateSiteMarker(this.editingSite)

    this.persistStorage()
    this.showSites()
  }

  showSites () {
    this.switchPane('showSites')

    for (const el of $$('.site', this.showSitesPane)) {
      el.remove()
    }

    const rowTemplate = $('#site-template')
    for (const site of this.sites) {
      site.marker.setOpacity(1.0)
      const row = this.cloneTemplate(rowTemplate)
      row.classList.add('site')
      $('.site-name', row).textContent = site.name
      $('.site-visit', row).textContent = site.visit
      $('.site-duties', row).textContent = site.duties.length
      $('.site-edit', row).onclick = () => {
        this.editSite(site)
      }
      $('.sites', this.showSitesPane).appendChild(row)
    }

    $('#min-start-at').value = this.minStartAt
    $('#max-end-at').value = this.maxEndAt ? this.maxEndAt : ''
  }

  updateSiteMarker (site) {
    if (site.marker) {
      site.marker.remove()
    }

    const color = site.visit === Site.VISIT_ALWAYS ? 'green' : (site.visit === Site.VISIT_MAYBE ? 'yellow' : 'grey')
    const marker = L.marker({ lat: site.latitude, lng: site.longitude }, {
      title: site.name,
      icon: new L.Icon({
        iconUrl: `https://raw.githubusercontent.com/pointhi/leaflet-color-markers/master/img/marker-icon-2x-${color}.png`,
        shadowUrl: 'https://cdnjs.cloudflare.com/ajax/libs/leaflet/0.7.7/images/marker-shadow.png',
        iconSize: [25, 41],
        iconAnchor: [12, 41],
        popupAnchor: [1, -34],
        shadowSize: [41, 41]
      })
    }).addTo(this.map)
    marker.on('click', event => this.editSite(site))
    site.marker = marker
  }

  cloneTemplate (template) {
    const result = template.cloneNode(true)
    result.id = null
    show(result)
    return result
  }

  calculatePaths () {
    this.switchPane('calculatingPaths')

    this.rideDurations.updateForSites(this.sites).then(() =>
      postApi('/api/calculate-paths', this.prepareCalculationWorld())
    ).then(paths => {
      this.showPaths(paths)
    })
  }

  showPaths (paths) {
    this.switchPane('showPaths')
    for (const row of $$('.path', this.showPathsPane)) {
      row.remove()
    }

    let activeRow = null
    const template = $('#path-template')
    for (const path of paths) {
      const row = this.cloneTemplate(template)
      row.classList.add('path')
      $('.path-total-ride', row).textContent = path.cost.totalRide
      $('.path-total-time', row).textContent = path.cost.totalTime
      $('.path-stops-on-duty', row).textContent = path.cost.stopsOnDuty
      $('.path-stops', row).textContent = path.cost.stops
      row.onclick = () => {
        if (activeRow) {
          activeRow.classList.remove('table-active')
        }
        row.classList.add('table-active')
        activeRow = row
        this.showPath(path)
      }
      $('.paths', this.showPathsPane).appendChild(row)
    }

    hide($('.detailed-path', this.showPathsPane))
  }

  showPath (path) {
    const visitedSiteNames = new Set(path.stops.map(stop => stop.site))
    const siteByName = new Map(this.sites.map(site => [site.name, site]))

    for (const site of this.sites) {
      site.marker.setOpacity(visitedSiteNames.has(site.name) ? 1.0 : 0.5)
    }

    const latlngs = [siteByName.get(path.startIn).marker.getLatLng()]
    for (const stop of path.stops) {
      latlngs.push(siteByName.get(stop.site).marker.getLatLng())
    }

    if (this.pathPolyline) {
      this.pathPolyline.remove()
    }
    this.pathPolyline = L.polyline(latlngs, { color: 'black', weight: 5 }).addTo(this.map)
    this.map.fitBounds(this.pathPolyline.getBounds())

    show($('.detailed-path', this.showPathsPane))
    const steps = [
      [path.startAt, `Start from ${path.startIn}`]
    ]
    for (const stop of path.stops) {
      if (stop.rideStart !== path.startAt) {
        steps.push([stop.rideStart, `Ride to ${stop.site}`])
      }
      if (stop.duty) {
        steps.push([stop.rideEnd, `Arrive at ${stop.site}. Duty is from ${stop.duty.start} until ${stop.duty.end}`])
      } else {
        steps.push([stop.rideEnd, `Arrive at ${stop.site}`])
      }
      if (stop.serviceStart !== stop.rideEnd) {
        steps.push([stop.serviceStart, `Wait in ${stop.site} for duty start`])
      }
    }

    const template = $('#step-template')
    const pathStepsEl = $('.path-steps', this.showPathsPane)
    for (const step of $$('.step', this.showPathsPane)) {
      step.remove()
    }
    for (const [time, text] of steps) {
      const stepEl = this.cloneTemplate(template)
      stepEl.classList.add('step')
      $('.step-time', stepEl).textContent = time
      $('.step-text', stepEl).textContent = text
      pathStepsEl.insertBefore(stepEl, pathStepsEl.firstChild)
    }
  }

  prepareCalculationWorld () {
    // Collect and convert relevant sites
    const sites = []
    for (const site of this.sites) {
      if (site.visit === Site.VISIT_NEVER) {
        continue
      }

      sites.push({
        name: site.name,
        latitude: site.latitude,
        longitude: site.longitude,
        rideDurations: {},
        duties: site.duties,
        serviceTime: `${site.serviceTimeMinutes}m`,
        mustVisit: site.visit === Site.VISIT_ALWAYS,
        canStartHere: site.canStartHere
      })
    }

    // Fill in ride duration information
    for (const origin of sites) {
      for (const destination of sites) {
        const ride = this.rideDurations.get(origin, destination)
        if (ride) {
          origin.rideDurations[destination.name] = ride
        }
      }
    }

    return {
      sites,
      minStartAt: this.minStartAt,
      maxEndAt: this.maxEndAt,
      maxTestedExtensions: 10
    }
  }

  static loadStorage () {
    const dataString = window.localStorage.getItem('hub-data')
    if (!dataString) {
      return
    }

    try {
      const data = JSON.parse(dataString)
      if (data.version !== 1) {
        console.error(`Could not load from storage: invalid version ${data.version}`)
        return
      }
      return data
    } catch (e) {
      console.error('Failed to parse JSON from storage')
    }
  }

  persistStorage () {
    const data = {
      version: 1,
      sites: this.sites.map(site => ({
        name: site.name,
        latitude: site.latitude,
        longitude: site.longitude,
        serviceTimeMinutes: site.serviceTimeMinutes,
        visit: site.visit,
        duties: site.duties,
        canStartHere: site.canStartHere
      })),
      rideDurations: this.rideDurations.toJSON(),
      minStartAt: this.minStartAt,
      maxEndAt: this.maxEndAt
    }

    window.localStorage.setItem('hub-data', JSON.stringify(data))
  }
}
