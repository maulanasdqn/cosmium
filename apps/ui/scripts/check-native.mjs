import { readdirSync, readFileSync, statSync } from 'node:fs'
import { join, relative } from 'node:path'

const root = new URL('../src', import.meta.url).pathname
const allowed = [join(root, 'components', 'ui')]
const tag = /(?<![\w.$])<([a-z][a-z0-9]*)(?=[\s/>])/g

function walk(dir) {
  return readdirSync(dir).flatMap((name) => {
    const path = join(dir, name)
    return statSync(path).isDirectory() ? walk(path) : [path]
  })
}

const offenders = walk(root)
  .filter((file) => file.endsWith('.tsx'))
  .filter((file) => !allowed.some((dir) => file.startsWith(dir)))
  .flatMap((file) =>
    readFileSync(file, 'utf8')
      .split('\n')
      .flatMap((line, index) =>
        [...line.matchAll(tag)].map((match) => `${relative(root, file)}:${index + 1}: <${match[1]}>`),
      ),
  )

if (offenders.length > 0) {
  console.error('Native HTML elements are only allowed in src/components/ui:')
  console.error(offenders.join('\n'))
  process.exit(1)
}
console.log('No native HTML elements outside src/components/ui')
