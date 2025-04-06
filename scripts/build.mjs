import { read, resolve, write } from './utils.mjs'

function run() {
  const file = resolve('pkg/package.json')
  const pkg = JSON.parse(read(file))
  const base = JSON.parse(read('package.json'))

  pkg.main = 'hexchess.js'
  pkg.name = '@bedard/hexchess'
  pkg.version = base.version
  write(file, JSON.stringify(pkg, null, 2) + '\n')

  const append = read('src/append.ts').replace('x.y.z', base.version)
  write('pkg/hexchess.js', read('pkg/hexchess.js') + '\n' + append)
  write('pkg/hexchess.d.ts', read('pkg/hexchess.d.ts') + '\n' + append)
}

run()
