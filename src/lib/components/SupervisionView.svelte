<script lang="ts">
  import { _ } from 'svelte-i18n';

  type Device = {
    device_id: string;
    device_name: string;
    imei?: string;
    mode: 'signin' | 'supervisor';
    created_at: string;
    last_seen_at: string;
    last_name_updated_at?: string;
  };

  type DeviceStatus = {
    device_id: string;
    device_name: string;
    mode: 'signin' | 'supervisor';
    last_signin?: string;
    streak: number;
  };

  // Helper function to check if signed in today
  function isSignedInToday(lastSignin?: string): boolean {
    if (!lastSignin) return false;
    const signinDate = new Date(lastSignin);
    const today = new Date();
    return signinDate.toDateString() === today.toDateString();
  }

  interface Props {
    searchQuery: string;
    searchResults: Device[];
    showSearchResults: boolean;
    targetDeviceId: string;
    requestError: string;
    isLoading: boolean;
    isLoadingSupervision: boolean;
    supervisedDevices: DeviceStatus[];
    onSearch: (event: Event) => void;
    onSelectDevice: (device: Device) => void;
    onSubmitRequest: (event: Event) => void;
    onRemoveRelation: (deviceId: string) => void;
  }

  let {
    searchQuery = $bindable(),
    searchResults,
    showSearchResults,
    targetDeviceId,
    requestError,
    isLoading,
    isLoadingSupervision,
    supervisedDevices,
    onSearch,
    onSelectDevice,
    onSubmitRequest,
    onRemoveRelation
  }: Props = $props();

  function handleSelectDevice(device: Device) {
    onSelectDevice(device);
  }

  function handleKeydown(e: KeyboardEvent, device: Device) {
    if (e.key === 'Enter') {
      handleSelectDevice(device);
    }
  }
</script>

<div class="supervision-view">
  <div class="add-supervision-card">
    <h3>{$_('supervision.addDevice')}</h3>
    <form onsubmit={onSubmitRequest}>
      <div class="search-container">
        <div class="input-group">
          <input
            id="search_query"
            type="text"
            placeholder={$_('supervision.searchPlaceholder')}
            bind:value={searchQuery}
            oninput={onSearch}
            disabled={isLoading}
            autocomplete="off"
          />
          <label for="search_query">{$_('supervision.search')}</label>
        </div>

        {#if showSearchResults && searchResults.length > 0}
          <div class="search-results" role="listbox">
            {#each searchResults as device (device.device_id)}
              <button
                type="button"
                class="search-result-item"
                class:selected={targetDeviceId === device.device_id}
                onclick={() => handleSelectDevice(device)}
                onkeydown={(e) => handleKeydown(e, device)}
                role="option"
                aria-selected={targetDeviceId === device.device_id}
              >
                <div class="result-info">
                  <strong>{device.device_name}</strong>
                  <span class="result-id">{device.device_id}</span>
                </div>
                <span class="result-mode">{device.mode}</span>
              </button>
            {/each}
          </div>
        {/if}
      </div>

      <div class="selected-device" class:has-selection={targetDeviceId}>
        <div class="selected-device-label">{$_('supervision.selectedDevice')}</div>
        <div class="device-display">
          {#if targetDeviceId}
            {targetDeviceId}
          {:else}
            {$_('supervision.noDeviceSelected')}
          {/if}
        </div>
      </div>

      {#if requestError}
        <div class="error-message">{requestError}</div>
      {/if}

      <button type="submit" class="primary-btn" disabled={!targetDeviceId || isLoading}>
        {#if isLoading}
          <span class="loader"></span>
        {:else}
          {$_('supervision.sendRequest')}
        {/if}
      </button>
    </form>
  </div>

  <div class="supervised-devices">
    <h3>{$_('supervision.supervisedDevices')}</h3>
    {#if isLoadingSupervision}
      <div class="loading">{$_('common.loading')}</div>
    {:else if supervisedDevices.length === 0}
      <p class="no-devices">{$_('supervision.noSupervisedDevices')}</p>
    {:else}
      {#each supervisedDevices as device}
        <div class="device-card">
          <div class="device-info">
            <h4>{device.device_name}</h4>
            <p class="device-id-display">{$_('supervision.deviceId')}: {device.device_id}</p>
            <p class="streak-info">{$_('supervision.streak', { values: { streak: device.streak } })}</p>
            <p class="status">
              {#if isSignedInToday(device.last_signin)}
                <span class="status-badge status-checked-in">✓ {$_('supervision.checkedInToday')}</span>
              {:else}
                <span class="status-badge status-not-checked-in">✗ {$_('supervision.notCheckedInToday')}</span>
              {/if}
            </p>
          </div>
          <button class="remove-btn" onclick={() => onRemoveRelation(device.device_id)}>
            {$_('supervision.remove')}
          </button>
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>
  .supervision-view {
    animation: fadeIn 0.3s ease;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .add-supervision-card {
    background: var(--card-bg);
    border-radius: 12px;
    padding: 2rem;
    margin-bottom: 2rem;
    border: 2px solid var(--border-color);
  }

  .add-supervision-card h3 {
    margin: 0 0 1.5rem;
    font-size: 1.5rem;
    color: var(--text-main);
  }

  .search-container {
    position: relative;
    margin-bottom: 1.5rem;
  }

  .input-group {
    position: relative;
    margin-bottom: 0;
    text-align: left;
  }

  input {
    width: 100%;
    padding: 1.25rem;
    border: 2px solid var(--border-color);
    background: var(--card-bg);
    font-size: 1.25rem;
    color: var(--text-main);
    transition: all 0.2s ease;
    box-sizing: border-box;
    border-radius: 12px;
    height: 64px;
  }

  input:focus {
    outline: none;
    border-color: var(--primary);
    box-shadow: 0 0 0 4px rgba(79, 70, 229, 0.1);
  }

  input::placeholder {
    color: transparent;
  }

  label {
    position: absolute;
    left: 1rem;
    top: 1.25rem;
    color: var(--text-muted);
    font-size: 1.25rem;
    pointer-events: none;
    transition: 0.2s ease all;
    background: var(--card-bg);
    padding: 0 0.5rem;
  }

  input:focus ~ label,
  input:not(:placeholder-shown) ~ label {
    top: -0.75rem;
    font-size: 1rem;
    color: var(--primary);
    font-weight: 600;
  }

  .search-results {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    background: var(--card-bg);
    border: 2px solid var(--border-color);
    border-radius: 8px;
    box-shadow: var(--shadow-lg);
    max-height: 300px;
    overflow-y: auto;
    z-index: 100;
    margin-top: 0.5rem;
  }

  .search-result-item {
    padding: 1rem;
    border-bottom: 1px solid var(--border-color);
    cursor: pointer;
    display: flex;
    justify-content: space-between;
    align-items: center;
    transition: background 0.2s;
    width: 100%;
    background: transparent;
    border-left: none;
    border-right: none;
    border-top: none;
    font-family: inherit;
    text-align: left;
  }

  .search-result-item:hover {
    background: var(--bg-color);
  }

  .search-result-item:last-child {
    border-bottom: none;
  }

  .result-info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .result-info strong {
    font-size: 1.1rem;
    color: var(--text-main);
  }

  .result-id {
    font-size: 0.9rem;
    color: var(--text-muted);
    font-family: monospace;
  }

  .result-mode {
    padding: 0.25rem 0.75rem;
    background: var(--primary);
    color: white;
    border-radius: 4px;
    font-size: 0.85rem;
    font-weight: 600;
    text-transform: capitalize;
  }

  .selected-device {
    margin-bottom: 1.5rem;
  }

  .selected-device-label {
    display: block;
    margin-bottom: 0.5rem;
    font-size: 1.1rem;
    font-weight: 600;
    color: var(--text-main);
  }

  .device-display {
    padding: 1rem;
    background: var(--bg-color);
    border-radius: 8px;
    border: 2px solid var(--border-color);
    font-family: monospace;
    font-size: 1.1rem;
    min-height: 60px;
    display: flex;
    align-items: center;
  }

  .selected-device.has-selection .device-display {
    border-color: var(--primary);
    background: var(--primary);
    color: white;
  }

  .error-message {
    color: var(--error);
    margin-bottom: 1rem;
    padding: 1rem;
    background: rgba(239, 68, 68, 0.1);
    border-radius: 8px;
    border: 1px solid var(--error);
  }

  .supervised-devices {
    background: var(--card-bg);
    border-radius: 12px;
    padding: 2rem;
    border: 2px solid var(--border-color);
  }

  .supervised-devices h3 {
    margin: 0 0 1.5rem;
    font-size: 1.5rem;
    color: var(--text-main);
  }

  .loading {
    text-align: center;
    color: var(--text-muted);
    padding: 2rem;
  }

  .no-devices {
    color: var(--text-muted);
    font-size: 1.1rem;
  }

  .device-card {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem;
    background: var(--bg-color);
    border-radius: 8px;
    margin-bottom: 1rem;
  }

  .device-info h4 {
    margin: 0 0 0.5rem;
    font-size: 1.3rem;
    color: var(--text-main);
  }

  .device-id-display {
    margin: 0.25rem 0;
    color: var(--text-muted);
    font-size: 0.9rem;
    font-family: monospace;
  }

  .streak-info {
    margin: 0.25rem 0;
    color: var(--text-muted);
  }

  .status {
    margin: 0;
  }

  .status-badge {
    display: inline-block;
    padding: 0.25rem 0.75rem;
    border-radius: 4px;
    font-size: 0.9rem;
    font-weight: 600;
    margin-top: 0.5rem;
  }

  .status-checked-in {
    background: rgba(34, 197, 94, 0.2);
    color: var(--success);
  }

  .status-not-checked-in {
    background: rgba(239, 68, 68, 0.2);
    color: var(--error);
  }

  button {
    cursor: pointer;
    border: none;
    font-family: inherit;
    transition: all 0.2s;
  }

  .primary-btn {
    width: 100%;
    padding: 1.25rem;
    background: var(--primary);
    color: white;
    font-weight: 700;
    font-size: 1.25rem;
    border-radius: 12px;
    display: flex;
    justify-content: center;
    align-items: center;
    box-shadow: none;
    border: 2px solid transparent;
    min-height: 64px;
  }

  .primary-btn:not(:disabled):hover {
    background: var(--primary-hover);
  }

  .primary-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
    background: var(--text-muted);
  }

  .remove-btn {
    padding: 0.5rem 1rem;
    background: transparent;
    color: var(--error);
    border: 1px solid var(--error);
    border-radius: 6px;
    font-weight: 600;
  }

  .remove-btn:hover {
    background: var(--error);
    color: white;
  }

  .loader {
    width: 24px;
    height: 24px;
    border: 3px solid #ffffff;
    border-bottom-color: transparent;
    border-radius: 50%;
    display: inline-block;
    box-sizing: border-box;
    animation: rotation 1s linear infinite;
  }

  @keyframes rotation {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }
</style>