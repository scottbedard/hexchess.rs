import { read, resolve, write } from './utils.mjs'

function run() {
  const pkg = resolve('pkg/package.json')
  const json = JSON.parse(read(pkg))
  const base = JSON.parse(read('package.json'))

  // write module
  json.files.push('index.js')
  json.main = 'index.js'
  json.name = '@bedard/hexchess'
  json.sideEffects.push('./index.js')
  json.version = base.version
  write(pkg, JSON.stringify(json, null, 2) + '\n')
  write('pkg/index.js', read('src/pkg.mjs').replace('x.y.z', base.version))

  // append types
  const dts = read('pkg/hexchess.d.ts')
  const types = read('src/types.d.ts')
    .split('\n')
    .slice(
      read('src/types.d.ts')
        .split('\n')
        .findIndex(line => line.startsWith('// @append-types-start')) + 1
    )
    .join('\n')
    .replace('x.y.z', base.version)

  write('pkg/hexchess.d.ts', `${dts}\n${types}`)
}

run()
