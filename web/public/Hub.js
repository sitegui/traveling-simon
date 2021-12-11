/* global $, $$, L, Site, show, hide, RideDurations */

class Hub {
  constructor (el) {
    this.map = L.map($('#map')).setView([47.4645927, -0.5583979], 12)
    this.sites = []
    this.editingSite = null
    this.rideDurations = new RideDurations()

    // eslint-disable-next-line no-template-curly-in-string
    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
      attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors',
      maxZoom: 18
    }).addTo(this.map)

    this.map.on('click', event => this.addSite(event.latlng.lat, event.latlng.lng))

    this.showSitesPane = $('#show-sites-pane')
    this.editSitePane = $('#edit-site-pane')
    this.calculatingPathsPanel = $('#calculating-paths-pane')

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

    // Reload persisted data
    const data = Hub.loadStorage()
    if (data) {
      for (const dataSite of data.sites) {
        const site = new Site(dataSite.name, dataSite.latitude, dataSite.longitude)
        site.visit = dataSite.visit
        site.serviceTimeMinutes = dataSite.serviceTimeMinutes
        site.duties = dataSite.duties
        this.updateSiteMarker(site)
        this.sites.push(site)
      }
      this.rideDurations = RideDurations.fromJSON(data.rideDurations)

      if (this.sites.length > 0) {
        const bounds = new L.LatLngBounds(this.sites.map(site => site.marker.getLatLng()))
        this.map.fitBounds(bounds)
      }
    }

    this.showSites()
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
    this.editingSite = site
    show(this.editSitePane)
    hide(this.showSitesPane)
    hide(this.calculatingPathsPanel)
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
    this.updateSiteMarker(this.editingSite)

    this.persistStorage()
    this.showSites()
  }

  showSites () {
    this.editingSite = null
    hide(this.editSitePane)
    show(this.showSitesPane)
    hide(this.calculatingPathsPanel)

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
    hide(this.editSitePane)
    hide(this.showSitesPane)
    show(this.calculatingPathsPanel)

    this.rideDurations.updateForSites(this.sites).then(() => {
      // Collect and convert relevant sites
      const sites = []
      for (const site of this.sites) {
        if (site.visit === Site.VISIT_NEVER) {
          continue
        }

        sites.push({
          name: site.name,
          rideDuration: {},
          duties: site.duties,
          serviceTime: `${site.serviceTimeMinutes}m`,
          mustVisit: site.visit === Site.VISIT_ALWAYS
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

      const world = {
        sites,
        // TODO: allow a way to actually fill in these fields
        start_in_one_of: sites[0].name,
        min_start_at: '00:00',
        max_end_at: null,
        max_tested_extensions: 10
      }
    })
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
        duties: site.duties
      })),
      rideDurations: this.rideDurations.toJSON()
    }

    window.localStorage.setItem('hub-data', JSON.stringify(data))
  }
}
