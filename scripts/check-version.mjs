import { fileURLToPath } from 'url'
import fs from 'fs'
import path from 'path'
import toml from 'smol-toml'

function run() {
  const __filename = fileURLToPath(import.meta.url)
  const __dirname = path.dirname(__filename)

  const cargo = toml.parse(fs.readFileSync(path.resolve(__dirname, '../Cargo.toml'), 'utf-8'))
  const npm = JSON.parse(fs.readFileSync(path.resolve(__dirname, '../package.json')), 'utf-8')

  console.log('Checking versions...')
  console.log()
  console.log(`Cargo:   ${cargo.package.version}`)
  console.log(`NPM:     ${npm.version}`)

  if (cargo.package.version !== npm.version) {
    throw new Error(`Version mismatch [npm: ${npm.version}, cargo: ${cargo.package.version}]`)
  }

  let releasing = false

  for (const arg of process.argv) {
    if (arg.startsWith('release=')) {
      const release = arg.slice(8)

      if (release !== npm.version) {
        throw new Error(`Release version mismatch [${release}]`)
      }

      console.log(`Release: ${release}`)
      releasing = true
    }
  }

  if (!releasing) {
    console.log('Release: \x1b[2mNone')
  }

  console.log()
  console.log('\x1b[32mSuccess')
}

run()
