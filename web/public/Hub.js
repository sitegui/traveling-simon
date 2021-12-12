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
    $('form', this.editSitePane).onchange = () => {
      this.autoSaveSite()
    }
    $('#site-duty-add').onclick = event => {
      event.preventDefault()
      this.pushDutyRow('', '')
    }
    $('form', this.editSitePane).onsubmit = event => {
      event.preventDefault()
      if (this.autoSaveSite()) {
        this.showSites()
      }
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
    $('.calculate-paths', this.showSitesPane).onclick = () => {
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
    $('.path-alternatives', this.showPathsPane).onclick = () => {
      hide($('.path-alternatives', this.showPathsPane))
      $$('.path-is-dominated', this.showPathsPane).forEach(show)
    }

    // Start calculating-paths pane
    $('.back', this.calculatingPathsPane).onclick = () => {
      this.showSites()
    }
    $('.show-details', this.calculatingPathsPane).onclick = () => {
      hide($('.show-details', this.calculatingPathsPane))
      show($('.error-details', this.calculatingPathsPane))
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
    // Find an unused name
    let name
    let n = 1
    do {
      name = `Site ${n}`
      n += 1
    } while (!this.sites.every(site => site.name !== name))

    const site = new Site(name, latitude, longitude)
    if (this.sites.length === 0) {
      site.canStartHere = true
    }
    this.sites.push(site)
    this.updateSiteMarker(site)
    this.editSite(site)
    $('#site-name').select()
    this.persistStorage()
  }

  editSite (site) {
    this.switchPane('editSite')
    this.editingSite = site
    for (const otherSite of this.sites) {
      otherSite.marker.setOpacity(otherSite === site ? 1.0 : 0.5)
    }
    this.map.panTo(site.marker.getLatLng(), { animate: true })

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
    this.pushDutyRow('', '')
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
      this.autoSaveSite()
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

  autoSaveSite () {
    // Name
    const nameEl = $('#site-name')
    const name = nameEl.value
    const validName = this.sites.every(otherSite => otherSite === this.editingSite || otherSite.name !== name)
    nameEl.setCustomValidity(validName ? '' : 'Another site with this name already exists')
    nameEl.classList.toggle('is-invalid', !validName)
    if (validName) {
      this.editingSite.name = name
    }

    // Duties
    const duties = []
    let validDuties = true
    for (const row of $$('tr.site-duty')) {
      const start = $('#site-duty-start', row).value
      const endEl = $('#site-duty-end', row)
      const end = endEl.value
      let error = ''
      if (start !== '' && end !== '') {
        if (end < start) {
          error = 'It cannot end before it starts'
          validDuties = false
        } else {
          duties.push({ start, end })
        }
      }
      endEl.setCustomValidity(error)
      endEl.classList.toggle('is-invalid', error !== '')
    }
    if (validDuties) {
      this.editingSite.duties = duties
    }

    const formEl = $('#edit-site-pane form')
    this.editingSite.serviceTimeMinutes = Number($('#site-service-time').value)
    this.editingSite.visit = formEl['site-visit'].value
    this.editingSite.canStartHere = $('#site-can-start-here').checked
    this.updateSiteMarker(this.editingSite)

    this.persistStorage()

    return formEl.checkValidity()
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
      $('.site-visit', row).textContent = site.visit === Site.VISIT_ALWAYS ? 'Yes' : (site.visit === Site.VISIT_MAYBE ? 'Maybe' : 'No')
      let duties
      if (site.duties.length === 0) {
        duties = '-'
      } else {
        duties = site.duties.map(duty => `${duty.start} - ${duty.end}`).join('\n')
      }
      $('.site-duties', row).textContent = duties
      $('.site-start', row).textContent = site.canStartHere ? 'Yes' : ''
      row.onclick = () => {
        this.editSite(site)
      }
      $('.sites', this.showSitesPane).appendChild(row)
    }

    $('#min-start-at').value = this.minStartAt
    $('#max-end-at').value = this.maxEndAt ? this.maxEndAt : ''

    const empty = this.sites.length === 0
    $('.sites-list', this.showSitesPane).classList.toggle('d-none', empty)
    $('.sites-list-empty', this.showSitesPane).classList.toggle('d-none', !empty)
    $('.calculate-paths', this.showSitesPane).disabled = empty
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
    show($('.calculating', this.calculatingPathsPane))
    hide($('.calculation-error', this.calculatingPathsPane))
    show($('.show-details', this.calculatingPathsPane))
    hide($('.error-details', this.calculatingPathsPane))

    this.rideDurations.updateForSites(this.sites).then(() =>
      postApi('/api/calculate-paths', this.prepareCalculationWorld())
    ).then(paths => {
      this.showPaths(paths)
    }).catch(error => {
      hide($('.calculating', this.calculatingPathsPane))
      show($('.calculation-error', this.calculatingPathsPane))
      $('.error-details', this.calculatingPathsPane).textContent = error
    })
  }

  showPaths (paths) {
    this.switchPane('showPaths')
    for (const row of $$('.path', this.showPathsPane)) {
      row.remove()
    }

    let activeRow = null
    let moreAlternatives = 0
    const template = $('#path-template')
    let firstRow = null
    for (const path of paths) {
      const row = this.cloneTemplate(template)
      row.classList.add('path')
      row.classList.toggle('path-is-dominated', path.isDominated)
      row.classList.toggle('d-none', path.isDominated)
      $('.path-total-ride', row).textContent = path.cost.totalRide
      const lastStop = path.stops[path.stops.length - 1]
      $('.path-total-time', row).textContent = `${path.startAt} - ${lastStop.serviceEnd} (${path.cost.totalTime})`
      $('.path-stops', row).textContent = `${path.cost.stops} (${path.cost.stopsOnDuty} in shifts)`
      row.onclick = () => {
        if (activeRow) {
          activeRow.classList.remove('table-active')
        }
        row.classList.add('table-active')
        activeRow = row
        this.showPath(path)
      }
      $('.paths', this.showPathsPane).appendChild(row)

      if (path.isDominated) {
        moreAlternatives += 1
      }

      if (firstRow === null) {
        firstRow = row
      }
    }

    hide($('.detailed-path', this.showPathsPane))
    const moreAlternativesEl = $('.path-alternatives', this.showPathsPane)
    if (moreAlternatives === 0) {
      hide(moreAlternativesEl)
    } else {
      show(moreAlternativesEl)
      moreAlternativesEl.textContent = `Show ${moreAlternatives} more alternative${moreAlternatives === 1 ? '' : 's'}`
    }

    const empty = firstRow === null
    if (!empty) {
      firstRow.click()
    }
    $('.paths-list', this.showPathsPane).classList.toggle('d-none', empty)
    $('.paths-list-empty', this.showPathsPane).classList.toggle('d-none', !empty)
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
        steps.push([stop.rideStart, `Ride to ${stop.site} in ${stop.ride}`])
      }
      if (stop.duty) {
        steps.push([stop.rideEnd, `Arrive at ${stop.site} for shift ${stop.duty.start} - ${stop.duty.end}`])
      } else {
        steps.push([stop.rideEnd, `Arrive at ${stop.site}`])
      }
      if (stop.serviceStart !== stop.rideEnd) {
        steps.push([stop.serviceStart, `Wait ${stop.wait} for shift start`])
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
      pathStepsEl.appendChild(stepEl)
    }
  }

  prepareCalculationWorld () {
    // Collect and convert relevant sites
    const sites = []
    for (const site of this.sites) {
      const rideDurations = {}
      for (const destination of this.sites) {
        const ride = this.rideDurations.get(site, destination)
        if (ride) {
          rideDurations[destination.name] = ride
        }
      }

      sites.push({
        name: site.name,
        rideDurations,
        duties: site.duties,
        serviceTime: `${site.serviceTimeMinutes}m`,
        visit: site.visit,
        canStartHere: site.canStartHere
      })
    }

    return {
      sites,
      minStartAt: this.minStartAt,
      maxEndAt: this.maxEndAt,
      maxTestedExtensions: 10,
      maxBagItems: 100,
      maxResults: 10
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
