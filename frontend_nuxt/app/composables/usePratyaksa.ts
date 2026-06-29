export type PratyaksaBackendMode = 'live' | 'simulasi'
export type PratyaksaSourceMode = 'live-silent' | 'live-telegram' | 'hit-endpoint-sendiri' | 'hit-endpoint-ml'

interface PratyaksaFleetAsset {
  asset_id: string
  equipment_type: string
  risk_level: string
  lstm_rul_hours: number
  rul_uncertainty: number
  model_agreement: boolean
  drift_detected: boolean
  processed_at: number
}

interface PratyaksaStatus {
  mode: PratyaksaBackendMode
  api_reachable: boolean
  fleet_count: number
}

interface FleetHealth {
  total: number
  normal: number
  warning: number
  critical: number
  avg_rul_hours: number
}

const status = ref<PratyaksaStatus>({
  mode: 'simulasi',
  api_reachable: false,
  fleet_count: 0,
})

const fleetData = ref<PratyaksaFleetAsset[]>([])
const fleetHealth = ref<FleetHealth>({
  total: 0, normal: 0, warning: 0, critical: 0, avg_rul_hours: 0,
})

const isLoading = ref(false)
const error = ref<string | null>(null)

const sourceMode = ref<PratyaksaSourceMode>('live-silent')

let pollingTimer: any = null

export const usePratyaksa = () => {
  const config = useRuntimeConfig()
  const baseURL = config.public.apiBase as string
  const mlTargetUrl = config.public.mlTargetUrl as string
  const customTargetUrl = config.public.customTargetUrl as string

  const fetchStatus = async () => {
    try {
      const res: any = await $fetch(`${baseURL}/pratyaksa/status`)
      status.value = res.data
      return res.data
    } catch (e: any) {
      error.value = e?.data?.message || 'Gagal fetch status'
      return null
    }
  }

  const fetchFleet = async () => {
    try {
      const res: any = await $fetch(`${baseURL}/pratyaksa/fleet`)
      fleetData.value = res.data.fleet
      status.value.mode = res.data.mode
      return res.data
    } catch (e: any) {
      error.value = e?.data?.message || 'Gagal fetch fleet'
      return null
    }
  }

  const fetchFleetHealth = async () => {
    try {
      const res: any = await $fetch(`${baseURL}/pratyaksa/fleet/health`)
      fleetHealth.value = res.data
      return res.data
    } catch (e: any) {
      error.value = e?.data?.message || 'Gagal fetch fleet health'
      return null
    }
  }

  const fetchResult = async (assetId: string) => {
    try {
      const res: any = await $fetch(`${baseURL}/pratyaksa/result/${assetId}`)
      return res.data
    } catch (e: any) {
      return null
    }
  }

  const fetchExplain = async (predictionId: string) => {
    try {
      const res: any = await $fetch(`${baseURL}/pratyaksa/explain/${predictionId}`)
      return res.data
    } catch (e: any) {
      return null
    }
  }

  const setMode = async (mode: 'live' | 'simulasi' | 'auto') => {
    try {
      const body: any = mode === 'auto' ? { reset: true } : { mode }
      const res: any = await $fetch(`${baseURL}/pratyaksa/mode`, {
        method: 'POST',
        body,
      })
      await fetchStatus()
      return res
    } catch (e: any) {
      error.value = e?.data?.message || 'Gagal set mode'
      return null
    }
  }

  const fetchAll = async () => {
    isLoading.value = true
    error.value = null
    await Promise.all([fetchStatus(), fetchFleet(), fetchFleetHealth()])
    isLoading.value = false
  }

  const startPolling = (intervalMs = 5000) => {
    stopPolling()
    pollingTimer = setInterval(() => {
      fetchAll()
    }, intervalMs)
  }

  const stopPolling = () => {
    if (pollingTimer) {
      clearInterval(pollingTimer)
      pollingTimer = null
    }
  }

  const setSourceMode = (mode: PratyaksaSourceMode) => {
    sourceMode.value = mode
  }

  const isLive = computed(() => status.value.mode === 'live')
  const isSimulasi = computed(() => status.value.mode === 'simulasi')

  return {
    status,
    fleetData,
    fleetHealth,
    isLoading,
    error,
    isLive,
    isSimulasi,
    sourceMode,
    mlTargetUrl,
    customTargetUrl,
    fetchAll,
    fetchStatus,
    fetchFleet,
    fetchResult,
    fetchExplain,
    setMode,
    startPolling,
    stopPolling,
    setSourceMode,
  }
}