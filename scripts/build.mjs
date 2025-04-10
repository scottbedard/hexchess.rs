import { read, resolve, write } from './utils.mjs'

function run() {
  const file = resolve('pkg/package.json')
  const pkg = JSON.parse(read(file))
  const base = JSON.parse(read('package.json'))

  pkg.main = 'hexchess.js'
  pkg.name = '@bedard/hexchess'
  pkg.version = base.version
  write(file, JSON.stringify(pkg, null, 2) + '\n')

  const bundle = read('src/bundle.mjs')
    .replace('x.y.z', base.version)
    .replace('// @wasm-bindgen', read('pkg/hexchess.js'))

  write('pkg/hexchess.js', bundle)
}

run()
