const fs = require('fs')
const path = require('path')

function run() {
  const file = path.resolve(__dirname, '../pkg/package.json')

  const pkg = JSON.parse(fs.readFileSync(file, 'utf-8'))

  pkg.main = 'hexchess.js'

  fs.writeFileSync(file, JSON.stringify(pkg, null, 2))
}

run()
