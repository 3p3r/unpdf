import { strict as assert } from 'node:assert'
import { createRequire } from 'node:module'
import { test } from 'node:test'

const require = createRequire(import.meta.url)
const api = require('../unpdf.wasi.cjs')

test('version', () => {
  assert.equal(api.version(), '0.6.4')
})

test('isPdfBytes', () => {
  assert.equal(api.isPdfBytes(Buffer.from('%PDF-1.4\n')), true)
  assert.equal(api.isPdfBytes(Buffer.from('not a pdf')), false)
})

test('toMarkdown rejects invalid pdf', () => {
  assert.throws(() => api.toMarkdown(Buffer.from('x')), /error/i)
})
