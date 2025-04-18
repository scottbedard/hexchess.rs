import { copy, dim, green, read, resolve, write } from './utils'
import { execSync } from 'child_process'

function run() {
  const startAt = Date.now()

  // cleanup
  execSync(`rm -rf ${resolve('dist')}`)

  // generate types
  execSync('tsc --project ./tsconfig.json --emitDeclarationOnly')

  // build npm package
  execSync('rollup --config rollup.config.ts --configPlugin @rollup/plugin-typescript')
  copy('LICENSE', 'dist/LICENSE')
  copy('package.json', 'dist/package.json')
  copy('README.md', 'dist/README.md')

  // build wasm package
  execSync(`${resolve('node_modules/.bin/wasm-pack')} build --out-dir ${resolve('dist/wasm')} --out-name index`)

  write(
    'dist/wasm/index.d.ts',
    `${read('src/wasm/prepend.post-build')}\n${read('dist/wasm/index.d.ts')}`
  )

  ;[
    'dist/wasm/README.md',
    'dist/wasm/.gitignore',
    'dist/wasm/LICENSE',
    'dist/wasm/package.json',
  ].forEach(file => execSync(`rm ${resolve(file)}`))

  // set version numbers
  const pkg = JSON.parse(read('package.json'))
  write('dist/index.mjs', read('dist/index.mjs').replace('x.y.z', pkg.version))

  console.log()
  console.log(`${green('Done')} ${dim(`${(Date.now() - startAt) / 1000}s`)}`)
}

run()
