import { fileURLToPath } from 'url'
import fs from 'node:fs'
import path from 'node:path'

export function dim(text) {
  return `\x1b[2m${text}\x1b[0m`
}

export function green(text) {
  return `\x1b[32m${text}\x1b[0m`
}

export function read(file) {
  return fs.readFileSync(resolve(file), 'utf-8')
}

export function resolve(file) {
  const __filename = fileURLToPath(import.meta.url)
  const __dirname = path.dirname(__filename)

  return path.resolve(__dirname, '..', file)
}

export function write(file, content) {
  fs.writeFileSync(resolve(file), content)
}
