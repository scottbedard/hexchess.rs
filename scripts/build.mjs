import { fileURLToPath } from 'url'
import fs from 'fs'
import path from 'path'

const read = (file) => fs.readFileSync(resolve(file), 'utf-8')

const resolve  = (file) => {
  const __filename = fileURLToPath(import.meta.url)
  const __dirname = path.dirname(__filename)

  return path.resolve(__dirname, '..', file)
}

const write = (file, content) => fs.writeFileSync(resolve(file), content)

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
