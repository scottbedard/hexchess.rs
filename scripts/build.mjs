import { dim, green, read, resolve, write } from './utils.mjs'
import { execSync } from 'child_process'

function run() {
  const startAt = Date.now()

  // cleanup
  execSync(`rm -rf ${resolve('dist')}`)

  // generate types
  execSync('tsc --project ./tsconfig.json --emitDeclarationOnly')

  // build npm package
  execSync('rollup --config rollup.config.ts --configPlugin @rollup/plugin-typescript')

  // build wasm package
  execSync(`wasm-pack build ${resolve('src/wasm')} --out-dir ${resolve('dist/wasm')} --out-name index`)
  execSync(`rm ${resolve('dist/wasm/.gitignore')}`)
  execSync(`rm ${resolve('dist/wasm/package.json')}`)

  // update version numbers
  const pkg = JSON.parse(read('package.json'))
  write('dist/index.mjs', read('dist/index.mjs').replace('x.y.z', pkg.version))

  console.log()
  console.log(`${green('Done')} ${dim(`${(Date.now() - startAt) / 1000}s`)}`)
}

run()
