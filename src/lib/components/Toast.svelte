<script lang="ts">
  import { fly } from 'svelte/transition';

  let {
    message,
    type,
    duration,
    onDismiss
  }: {
    message: string;
    type: 'success' | 'error' | 'info' | 'warning';
    duration: number;
    onDismiss?: () => void;
  } = $props();

  function getIcon() {
    if (type === 'success') return '✓';
    if (type === 'error') return '✕';
    if (type === 'warning') return '⚠';
    return 'ℹ';
  }

  function handleClick() {
    onDismiss?.();
  }
</script>

<button
  class="toast {type}"
  in:fly={{ y: -20, duration: 300 }}
  out:fly={{ y: -20, duration: 300 }}
  onclick={handleClick}
  aria-live="polite"
>
  <span class="icon">{getIcon()}</span>
  <span class="text">{message}</span>
</button>

<style>
  .toast {
    position: fixed;
    top: 20px;
    left: 50%;
    transform: translateX(-50%);
    padding: 12px 24px;
    border-radius: 8px;
    border: none;
    color: white;
    font-size: 14px;
    font-weight: 500;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    display: flex;
    align-items: center;
    gap: 8px;
    z-index: 9999;
    min-width: 200px;
    max-width: 90%;
    cursor: pointer;
  }

  .toast.success { background-color: #10b981; }
  .toast.error { background-color: #ef4444; }
  .toast.warning { background-color: #f59e0b; }
  .toast.info { background-color: #3b82f6; }

  .icon {
    font-size: 18px;
    font-weight: bold;
    flex-shrink: 0;
  }

  .text {
    word-break: break-word;
  }
</style>
