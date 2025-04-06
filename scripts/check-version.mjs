import { read } from './utils.mjs'
import toml from 'smol-toml'

function run() {
  const cargo = toml.parse(read('Cargo.toml'))
  const pkg = JSON.parse(read('package.json'))

  console.log('Checking versions...')
  console.log()
  console.log(`Cargo:   ${cargo.package.version}`)
  console.log(`NPM:     ${pkg.version}`)

  if (cargo.package.version !== pkg.version) {
    throw new Error(`Version mismatch [npm: ${pkg.version}, cargo: ${cargo.package.version}]`)
  }

  let releasing = false

  for (const arg of process.argv) {
    if (arg.startsWith('release=')) {
      const release = arg.slice(8)

      if (release !== pkg.version) {
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
