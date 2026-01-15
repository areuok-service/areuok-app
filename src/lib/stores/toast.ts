import { writable } from 'svelte/store';

export type ToastType = 'success' | 'error' | 'info' | 'warning';

export interface ToastMessage {
  id: number;
  message: string;
  type: ToastType;
  duration: number;
}

function createToastStore() {
  const { subscribe, update } = writable<ToastMessage[]>([]);

  function show(message: string, type: ToastType = 'info', duration = 3000) {
    const id = Date.now() + Math.random();

    update((toasts) => [...toasts, { id, message, type, duration }]);

    if (duration > 0) {
      setTimeout(() => {
        dismiss(id);
      }, duration);
    }
  }

  function dismiss(id: number) {
    update((toasts) => toasts.filter((t) => t.id !== id));
  }

  function clear() {
    update(() => []);
  }

  return {
    subscribe,
    show,
    success: (message: string, duration = 3000) => show(message, 'success', duration),
    error: (message: string, duration = 5000) => show(message, 'error', duration),
    warning: (message: string, duration = 4000) => show(message, 'warning', duration),
    info: (message: string, duration = 3000) => show(message, 'info', duration),
    dismiss,
    clear,
  };
}

export const toastStore = createToastStore();
