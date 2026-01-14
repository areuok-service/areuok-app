const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:3000';

export type DeviceMode = 'signin' | 'supervisor';

export interface Device {
  device_id: string;
  device_name: string;
  imei?: string;
  mode: DeviceMode;
  created_at: string;
  last_seen_at: string;
  last_name_updated_at?: string;
}

export interface SigninData {
  name: string;
  last_signin_date: string;
  streak: number;
  signin_history: string[];
}

export interface DeviceStatus {
  device_id: string;
  device_name: string;
  last_signin_date?: string;
  streak: number;
  signed_in_today: boolean;
}

export interface SupervisionRequest {
  request_id: string;
  supervisor_id: string;
  supervisor_name: string;
  target_id: string;
  target_name: string;
  status: 'pending' | 'accepted' | 'rejected';
  created_at: string;
}

export interface SupervisionRelation {
  relation_id: string;
  supervisor_id: string;
  supervisor_name: string;
  target_id: string;
  target_name: string;
  created_at: string;
}

interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
}

async function apiRequest<T>(endpoint: string, options: RequestInit = {}): Promise<T> {
  const url = `${API_BASE_URL}${endpoint}`;

  const defaultOptions: RequestInit = {
    headers: {
      'Content-Type': 'application/json',
      ...options.headers,
    },
  };

  try {
    const response = await fetch(url, { ...defaultOptions, ...options });

    if (!response.ok) {
      const errorData = await response.json().catch(() => ({}));
      throw new Error(errorData.error || `HTTP ${response.status}: ${response.statusText}`);
    }

    return await response.json();
  } catch (error) {
    if (error instanceof Error) {
      throw error;
    }
    throw new Error('Network error or request failed');
  }
}

export const deviceApi = {
  register: async (
    deviceName: string,
    imei?: string,
    mode: DeviceMode = 'signin'
  ): Promise<Device> => {
    return apiRequest<Device>('/devices/register', {
      method: 'POST',
      body: JSON.stringify({
        device_name: deviceName,
        imei: imei || null,
        mode,
      }),
    });
  },

  search: async (query: string): Promise<Device[]> => {
    return apiRequest<Device[]>(`/search/devices?q=${encodeURIComponent(query)}`);
  },

  getInfo: async (deviceId: string): Promise<Device> => {
    return apiRequest<Device>(`/devices/${deviceId}`);
  },

  updateName: async (deviceId: string, newName: string): Promise<Device> => {
    return apiRequest<Device>(`/devices/${deviceId}/name`, {
      method: 'PATCH',
      body: JSON.stringify({
        device_name: newName,
      }),
    });
  },

  signin: async (deviceId: string): Promise<{ streak: number }> => {
    return apiRequest<{ streak: number }>(`/devices/${deviceId}/signin`, {
      method: 'POST',
    });
  },

  getStatus: async (deviceId: string): Promise<DeviceStatus> => {
    return apiRequest<DeviceStatus>(`/devices/${deviceId}/status`);
  },
};

export const supervisionApi = {
  request: async (supervisorId: string, targetId: string): Promise<SupervisionRequest> => {
    return apiRequest<SupervisionRequest>('/supervision/request', {
      method: 'POST',
      body: JSON.stringify({
        supervisor_id: supervisorId,
        target_id: targetId,
      }),
    });
  },

  getPending: async (deviceId: string): Promise<SupervisionRequest[]> => {
    return apiRequest<SupervisionRequest[]>(`/supervision/pending/${deviceId}`);
  },

  accept: async (supervisorId: string, targetId: string): Promise<void> => {
    return apiRequest<void>('/supervision/accept', {
      method: 'POST',
      body: JSON.stringify({
        supervisor_id: supervisorId,
        target_id: targetId,
      }),
    });
  },

  reject: async (supervisorId: string, targetId: string): Promise<void> => {
    return apiRequest<void>('/supervision/reject', {
      method: 'POST',
      body: JSON.stringify({
        supervisor_id: supervisorId,
        target_id: targetId,
      }),
    });
  },

  list: async (deviceId: string): Promise<SupervisionRelation[]> => {
    return apiRequest<SupervisionRelation[]>(`/supervision/list/${deviceId}`);
  },

  remove: async (relationId: string): Promise<void> => {
    return apiRequest<void>(`/supervision/${relationId}`, {
      method: 'DELETE',
    });
  },
};

const DEVICE_ID_KEY = 'areuok_device_id';
const DEVICE_NAME_KEY = 'areuok_device_name';
const DEVICE_MODE_KEY = 'areuok_device_mode';
const DEVICE_IMEI_KEY = 'areuok_device_imei';
const LAST_NAME_UPDATE_KEY = 'areuok_last_name_update';

export const storage = {
  getDeviceId: (): string | null => {
    if (typeof window === 'undefined') return null;
    return localStorage.getItem(DEVICE_ID_KEY);
  },

  setDeviceId: (id: string): void => {
    if (typeof window !== 'undefined') {
      localStorage.setItem(DEVICE_ID_KEY, id);
    }
  },

  getDeviceName: (): string | null => {
    if (typeof window === 'undefined') return null;
    return localStorage.getItem(DEVICE_NAME_KEY);
  },

  setDeviceName: (name: string): void => {
    if (typeof window !== 'undefined') {
      localStorage.setItem(DEVICE_NAME_KEY, name);
    }
  },

  getDeviceMode: (): DeviceMode | null => {
    if (typeof window === 'undefined') return null;
    return localStorage.getItem(DEVICE_MODE_KEY) as DeviceMode | null;
  },

  setDeviceMode: (mode: DeviceMode): void => {
    if (typeof window !== 'undefined') {
      localStorage.setItem(DEVICE_MODE_KEY, mode);
    }
  },

  getDeviceImei: (): string | null => {
    if (typeof window === 'undefined') return null;
    return localStorage.getItem(DEVICE_IMEI_KEY);
  },

  setDeviceImei: (imei: string): void => {
    if (typeof window !== 'undefined') {
      localStorage.setItem(DEVICE_IMEI_KEY, imei);
    }
  },

  getLastNameUpdate: (): Date | null => {
    if (typeof window === 'undefined') return null;
    const value = localStorage.getItem(LAST_NAME_UPDATE_KEY);
    return value ? new Date(value) : null;
  },

  setLastNameUpdate: (date: Date): void => {
    if (typeof window !== 'undefined') {
      localStorage.setItem(LAST_NAME_UPDATE_KEY, date.toISOString());
    }
  },

  clearDevice: (): void => {
    if (typeof window !== 'undefined') {
      localStorage.removeItem(DEVICE_ID_KEY);
      localStorage.removeItem(DEVICE_NAME_KEY);
      localStorage.removeItem(DEVICE_MODE_KEY);
      localStorage.removeItem(DEVICE_IMEI_KEY);
      localStorage.removeItem(LAST_NAME_UPDATE_KEY);
    }
  },
};
