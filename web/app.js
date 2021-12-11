const express = require('express')
const path = require('path')
const logger = require('morgan')
const debug = require('debug')('web')
const http = require('http')
const port = 3000

const router = require('./routes')

const app = express()

app.use(logger('dev'))
app.use(express.json())
app.use(express.urlencoded({ extended: false }))
app.use(express.static(path.join(__dirname, 'public')))

app.use('/', router)
app.set('port', port)

const server = http.createServer(app)

server.listen(port)
server.on('listening', () => {
  debug('Listening on ' + port)
})
