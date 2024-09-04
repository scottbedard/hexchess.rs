import { fileURLToPath } from 'url'
import fs from 'fs'
import path from 'path'
import toml from 'smol-toml'

function run() {
  const __filename = fileURLToPath(import.meta.url)
  const __dirname = path.dirname(__filename)

  const cargo = toml.parse(fs.readFileSync(path.resolve(__dirname, '../Cargo.toml'), 'utf-8'))
  const npm = JSON.parse(fs.readFileSync(path.resolve(__dirname, '../package.json')), 'utf-8')
  
  if (cargo.package.version !== npm.version) {
    throw new Error('Cargo / NPM version mismatch')
  }
}

run()
