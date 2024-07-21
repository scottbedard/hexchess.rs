import { fileURLToPath } from 'url'
import fs from 'fs'
import path from 'path'

function run() {
  const __filename = fileURLToPath(import.meta.url)
  const __dirname = path.dirname(__filename)

  const file = path.resolve(__dirname, '../pkg/package.json')
  const pkg = JSON.parse(fs.readFileSync(file), 'utf-8')
  const base = JSON.parse(fs.readFileSync(path.resolve(__dirname, '../package.json'), 'utf-8'))

  pkg.main = 'hexchess.js'
  pkg.name = '@bedard/hexchess'
  pkg.version = base.version

  fs.writeFileSync(file, JSON.stringify(pkg, null, 2))
}

run()
