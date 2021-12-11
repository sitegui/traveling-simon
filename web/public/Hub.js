/* global $, $$, L, Site, show, hide */

class Hub {
  constructor (el) {
    this.map = L.map($('#map')).setView([47.4645927, -0.5583979], 12)
    this.sites = []
    this.editingSite = null

    // eslint-disable-next-line no-template-curly-in-string
    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
      attribution: '&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors',
      maxZoom: 18
    }).addTo(this.map)

    this.map.on('click', event => this.addSite(event.latlng.lat, event.latlng.lng))

    this.showSitesPane = $('#options-show-sites')
    this.editSitePane = $('#options-edit-site')

    // Start edit-site pane
    $('#site-duty-add').onclick = event => {
      event.preventDefault()
      this.pushDutyRow('', '')
    }
    $('#options-edit-site form').onsubmit = event => {
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

    this.showSites()
  }

  addSite (latitude, longitude) {
    const name = `Site ${this.sites.length + 1}`
    const site = new Site(name, latitude, longitude)
    this.sites.push(site)
    this.updateSiteMarker(site)
    this.editSite(site)
  }

  editSite (site) {
    this.editingSite = site
    show(this.editSitePane)
    hide(this.showSitesPane)
    for (const otherSite of this.sites) {
      otherSite.marker.setOpacity(otherSite === site ? 1.0 : 0.5)
    }

    // Fill in the form
    $('#site-name').value = site.name
    $('#site-service-time').value = String(site.serviceTimeMinutes)
    $('#options-edit-site form')['site-visit'].value = site.visit
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
    this.editingSite.visit = $('#options-edit-site form')['site-visit'].value
    this.editingSite.duties = []
    for (const row of $$('tr.site-duty')) {
      const start = $('#site-duty-start', row).value
      const end = $('#site-duty-end', row).value
      if (start !== '' && end !== '') {
        this.editingSite.duties.push({ start, end })
      }
    }
    this.updateSiteMarker(this.editingSite)

    this.showSites()
  }

  showSites () {
    this.editingSite = null
    hide(this.editSitePane)
    show(this.showSitesPane)

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
}
