import { read, resolve, write } from './utils.mjs'

function run() {
  const file = resolve('pkg/package.json')
  const pkg = JSON.parse(read(file))
  const base = JSON.parse(read('package.json'))

  pkg.files.push('index.js')
  pkg.main = 'index.js'
  pkg.name = '@bedard/hexchess'
  pkg.sideEffects.push('./index.js')
  pkg.version = base.version
  write(file, JSON.stringify(pkg, null, 2) + '\n')

  // write module files
  write('pkg/index.js', read('src/pkg.mjs').replace('x.y.z', base.version))
  write('pkg/hexchess.d.ts', read('pkg/hexchess.d.ts').replace('x.y.z', base.version))
}

run()
