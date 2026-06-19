import { randomUUID } from 'node:crypto'
import { mkdir, writeFile } from 'node:fs/promises'
import { join } from 'node:path'

/**
 * Upload file model 3D (.glb / .gltf) ke media frontend.
 * File disimpan di public/media/models/ dan dikembalikan URL publiknya.
 */
const ALLOWED_EXT = ['.glb', '.gltf']
const MAX_BYTES = 50 * 1024 * 1024 // 50 MB

export default defineEventHandler(async (event) => {
  const form = await readMultipartFormData(event)
  const file = form?.find((f) => f.name === 'file' && f.filename)

  if (!file || !file.data) {
    throw createError({ statusCode: 400, statusMessage: 'File tidak ditemukan.' })
  }

  const original = (file.filename || 'model.glb').toLowerCase()
  const ext = original.slice(original.lastIndexOf('.'))
  if (!ALLOWED_EXT.includes(ext)) {
    throw createError({ statusCode: 400, statusMessage: 'Format harus .glb atau .gltf.' })
  }
  if (file.data.length > MAX_BYTES) {
    throw createError({ statusCode: 413, statusMessage: 'Ukuran file maksimal 50 MB.' })
  }

  const dir = join(process.cwd(), 'public', 'media', 'models')
  await mkdir(dir, { recursive: true })

  const safeName = `${Date.now()}-${randomUUID().slice(0, 8)}${ext}`
  await writeFile(join(dir, safeName), file.data)

  return {
    status: 'success',
    url: `/media/models/${safeName}`,
    filename: original,
    size: file.data.length,
  }
})
