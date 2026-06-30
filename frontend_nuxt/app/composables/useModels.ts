/**
 * useModels — pemetaan jenis alat berat → model 3D (.glb) yang realistis.
 * Aset CC0 (Poly by Google / poly.pizza) disimpan lokal di public/media/models.
 * Dipakai sebagai fallback bila sebuah unit belum punya model3d_url sendiri.
 */
const MODELS = {
  excavator: '/media/models/crane.glb',
  haul_truck: '/media/models/dump_truck.glb',
  bulldozer: '/media/models/bulldozer.glb',
  wheel_loader: '/media/models/tractor.glb',
  forklift: '/media/models/forklift.glb',
  default: '/media/models/bulldozer.glb',
} as const

export const useModels = () => {
  const modelForType = (nama?: string | null): string => {
    const j = (nama || '').toLowerCase()
    if (j === 'haul_truck' || j === 'dump_truck' || j.includes('dump') || j.includes('haul') || j.includes('truck')) return MODELS.haul_truck
    if (j === 'excavator' || j.includes('excav') || j.includes('zaxis') || j.includes('digger') || j.includes('crane')) return MODELS.excavator
    if (j === 'bulldozer' || j.includes('dozer')) return MODELS.bulldozer
    if (j === 'wheel_loader' || j.includes('loader')) return MODELS.wheel_loader
    if (j.includes('forklift')) return MODELS.forklift
    return MODELS.default
  }

  /** URL model untuk dipakai viewer: pakai milik unit, jika kosong pakai fallback per jenis. */
  const resolveModel = (model3dUrl?: string | null, jenis?: string | null): string =>
    model3dUrl && model3dUrl.trim() ? model3dUrl : modelForType(jenis)

  return { modelForType, resolveModel, MODELS }
}
