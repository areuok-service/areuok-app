<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { deviceApi, supervisionApi, storage } from '../lib/api';
  import type { DeviceMode, SupervisionRelation, SupervisionRequest, DeviceStatus, Device } from '../lib/api';
  import { fade, fly, scale } from "svelte/transition";
  import { locale, _ } from 'svelte-i18n';

  interface Quote {
    text: string;
    author: string;
  }

  type View = 'login' | 'dashboard' | 'signin-request' | 'supervision-request' | 'supervised-devices';
  type TabView = 'signin' | 'supervision';

  let view = $state<View>('login');
  let tabView = $state<TabView>('signin');
  let name = $state("");
  let deviceId = $state<string | null>(null);
  let deviceMode = $state<DeviceMode>('signin');
  let isSignedIn = $state(false);
  let streak = $state(0);
  let isLoading = $state(false);
  let showQuote = $state(false);
  let quote = $state<Quote | null>(null);

  let supervisionList = $state<SupervisionRelation[]>([]);
  let pendingRequests = $state<SupervisionRequest[]>([]);
  let supervisedDevices = $state<DeviceStatus[]>([]);
  let deviceRelationMap = $state<Map<string, string>>(new Map());
  let isLoadingSupervision = $state(false);

  let searchQuery = $state("");
  let searchResults = $state<Device[]>([]);
  let isSearching = $state(false);
  let showSearchResults = $state(false);

  let targetDeviceId = $state("");
  let requestError = $state("");
  let showEditNameModal = $state(false);
  let newName = $state("");
  let editError = $state("");

  function getTodayDate(): string {
    return new Date().toISOString().split('T')[0];
  }

  async function loadDailyQuote() {
    try {
      quote = await invoke<Quote>("get_daily_quote");
    } catch (error) {
      console.error("Failed to load quote:", error);
    }
  }

  async function handleRegister(event: Event) {
    event.preventDefault();
    if (!name.trim()) return;

    isLoading = true;

    try {
      let imei: string | undefined;
      try {
        imei = await invoke<string>("get_device_imei");
      } catch (e) {
        console.error("Failed to get IMEI:", e);
      }

      const device = await deviceApi.register(name, imei, deviceMode);
      deviceId = device.device_id;
      storage.setDeviceId(device.device_id);
      storage.setDeviceName(name);
      storage.setDeviceMode(deviceMode);
      if (imei) {
        storage.setDeviceImei(imei);
      }

      view = 'dashboard';
      await loadSupervisionData();
      await loadDailyQuote();
    } catch (error) {
      console.error("Registration failed:", error);
      alert("Registration failed: " + (error as Error).message);
    } finally {
      isLoading = false;
    }
  }

  async function loadFromStorage() {
    const savedDeviceId = storage.getDeviceId();
    const savedName = storage.getDeviceName();
    const savedMode = storage.getDeviceMode();

    if (savedDeviceId && savedName && savedMode) {
      deviceId = savedDeviceId;
      name = savedName;
      deviceMode = savedMode;

      try {
        const device = await deviceApi.getInfo(savedDeviceId);
        const today = getTodayDate();
        const lastSeen = new Date(device.last_seen_at);

        view = 'dashboard';
        await loadSupervisionData();
      } catch (error) {
        console.error("Failed to load device:", error);
        storage.clearDevice();
        view = 'login';
      }
    }

    await loadDailyQuote();
  }

  async function loadSupervisionData() {
    if (!deviceId) return;

    isLoadingSupervision = true;
    try {
      const [relations, requests] = await Promise.all([
        supervisionApi.list(deviceId),
        supervisionApi.getPending(deviceId),
      ]);

      supervisionList = relations;
      pendingRequests = requests;

      const newDeviceRelationMap = new Map<string, string>();
      relations.forEach(rel => {
        newDeviceRelationMap.set(rel.target_id, rel.relation_id);
      });
      deviceRelationMap = newDeviceRelationMap;

      if (relations.length > 0) {
        const deviceStatuses = await Promise.all(
          relations.map(rel => deviceApi.getStatus(rel.target_id))
        );
        supervisedDevices = deviceStatuses;
      } else {
        supervisedDevices = [];
      }
    } catch (error) {
      console.error("Failed to load supervision data:", error);
    } finally {
      isLoadingSupervision = false;
    }
  }

  async function handleSignIn(event: Event) {
    event.preventDefault();
    if (!name.trim() || !deviceId) return;

    isLoading = true;

    try {
      await new Promise(r => setTimeout(r, 800));

      const result = await deviceApi.signin(deviceId);
      isSignedIn = true;
      streak = result.streak;

      setTimeout(() => {
        showQuote = true;
      }, 500);
    } catch (error) {
      console.error(error);
      alert("Sign in failed: " + (error as Error).message);
    } finally {
      isLoading = false;
    }
  }

  async function searchDevices(event: Event) {
    event.preventDefault();
    if (searchQuery.length < 2) return;

    isSearching = true;
    try {
      searchResults = await deviceApi.search(searchQuery);
      showSearchResults = true;
    } catch (error) {
      console.error("Search failed:", error);
    } finally {
      isSearching = false;
    }
  }

  function selectDevice(device: Device) {
    targetDeviceId = device.device_id;
    showSearchResults = false;
    searchQuery = device.device_name;
  }

  async function requestSupervision(event: Event) {
    event.preventDefault();
    if (!targetDeviceId || !deviceId) return;

    requestError = "";
    isLoading = true;

    try {
      await supervisionApi.request(deviceId, targetDeviceId);
      alert($_('supervision.requestSent'));
      targetDeviceId = "";
      searchQuery = "";
      searchResults = [];
      showSearchResults = false;
      await loadSupervisionData();
      tabView = 'supervision';
    } catch (error) {
      requestError = (error as Error).message;
    } finally {
      isLoading = false;
    }
  }

  async function acceptRequest(request: SupervisionRequest) {
    if (!deviceId) return;

    try {
      await supervisionApi.accept(request.supervisor_id, request.target_id);
      await loadSupervisionData();
    } catch (error) {
      alert("Failed to accept request: " + (error as Error).message);
    }
  }

  async function rejectRequest(request: SupervisionRequest) {
    if (!deviceId) return;

    try {
      await supervisionApi.reject(request.supervisor_id, request.target_id);
      await loadSupervisionData();
    } catch (error) {
      alert("Failed to reject request: " + (error as Error).message);
    }
  }

  async function removeRelation(deviceIdToRemove: string) {
    const relationId = deviceRelationMap.get(deviceIdToRemove);
    if (!relationId) return;

    if (!confirm($_('supervision.confirmRemove'))) return;

    try {
      await supervisionApi.remove(relationId);
      await loadSupervisionData();
    } catch (error) {
      alert("Failed to remove supervision: " + (error as Error).message);
    }
  }

  async function signOut() {
    try {
      storage.clearDevice();
    } catch (error) {
      console.error("Failed to sign out:", error);
    }

    deviceId = null;
    name = "";
    streak = 0;
    isSignedIn = false;
    showQuote = false;
    view = 'login';
    supervisionList = [];
    pendingRequests = [];
    supervisedDevices = [];
  }

  function closeQuote() {
    showQuote = false;
  }

  function switchTab(newTab: TabView) {
    tabView = newTab;
  }

  function toggleLanguageMenu() {
    showLanguageMenu = !showLanguageMenu;
  }

  function closeLanguageMenu() {
    showLanguageMenu = false;
  }

  function setLanguage(newLocale: string) {
    locale.set(newLocale);
    if (typeof window !== 'undefined') {
      localStorage.setItem('locale', newLocale);
    }
    closeLanguageMenu();
  }

  function canUpdateName(): boolean {
    const lastUpdate = storage.getLastNameUpdate();
    if (!lastUpdate) return true;

    const daysSinceUpdate = Math.floor((Date.now() - lastUpdate.getTime()) / (1000 * 60 * 60 * 24));
    return daysSinceUpdate >= 15;
  }

  function openEditNameModal() {
    newName = name;
    editError = "";
    showEditNameModal = true;
  }

  function closeEditNameModal() {
    showEditNameModal = false;
    newName = "";
    editError = "";
  }

  async function handleNameUpdate() {
    if (!newName.trim() || !deviceId) return;

    if (!canUpdateName()) {
      const lastUpdate = storage.getLastNameUpdate();
      const daysSinceUpdate = lastUpdate
        ? Math.floor((Date.now() - lastUpdate.getTime()) / (1000 * 60 * 60 * 24))
        : 0;
      editError = `上次修改昵称是在 ${daysSinceUpdate} 天前，需要等待 ${15 - daysSinceUpdate} 天后才能再次修改`;
      return;
    }

    isLoading = true;
    editError = "";

    try {
      const device = await deviceApi.updateName(deviceId, newName);
      name = device.device_name;
      storage.setDeviceName(name);
      storage.setLastNameUpdate(new Date());

      showEditNameModal = false;
      newName = "";
    } catch (error) {
      console.error("Failed to update name:", error);
      editError = "修改昵称失败: " + (error as Error).message;
    } finally {
      isLoading = false;
    }
  }

  let showLanguageMenu = $state(false);

  loadFromStorage();
</script>

<main class="container">

  {#if view === 'login'}
    <div class="login-card" in:fly={{ y: 20, duration: 600 }}>
      <header>
        <h1>{$_('app.title')}</h1>
        <p class="subtitle">{$_('app.subtitle')}</p>
      </header>

      <form onsubmit={handleRegister}>
        <div class="input-group">
          <input
            id="name"
            type="text"
            placeholder={$_('login.namePlaceholder')}
            bind:value={name}
            disabled={isLoading}
            autocomplete="off"
          />
          <label for="name" class:has-value={name.length > 0}>{$_('login.nameLabel')}</label>
        </div>

        <div class="mode-selection">
          <label class="radio-option">
            <input type="radio" bind:group={deviceMode} value="signin" />
            <span class="radio-label">{$_('mode.signin')}</span>
          </label>
          <label class="radio-option">
            <input type="radio" bind:group={deviceMode} value="supervisor" />
            <span class="radio-label">{$_('mode.supervisor')}</span>
          </label>
        </div>

        <button type="submit" class="primary-btn" disabled={!name.trim() || isLoading}>
          {#if isLoading}
            <span class="loader"></span>
          {:else}
            {$_('login.registerButton')}
          {/if}
        </button>
      </form>
    </div>

  {:else if view === 'dashboard'}
    <div class="dashboard" in:fade={{ duration: 400 }}>
      <header class="dashboard-header">
        <div class="user-info">
          <h2>{$_('dashboard.greeting', { values: { name } })}</h2>
          <p class="status-text">{$_('dashboard.status')}</p>
        </div>
        <div class="header-actions">
          <div class="language-btn-wrapper">
            <button class="icon-btn" onclick={toggleLanguageMenu} aria-label={$_('language.title')}>
              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"></circle><line x1="2" y1="12" x2="22" y2="12"></line><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"></path></svg>
            </button>
            {#if showLanguageMenu}
              <div class="language-menu" role="menu">
                <button
                  class="language-option"
                  class:active={$locale === 'en'}
                  onclick={() => setLanguage('en')}
                  role="menuitem"
                >
                  {$_('language.en')}
                </button>
                <button
                  class="language-option"
                  class:active={$locale === 'zh-CN'}
                  onclick={() => setLanguage('zh-CN')}
                  role="menuitem"
                >
                  {$_('language.zhCN')}
                </button>
              </div>
            {/if}
          </div>
          <button class="icon-btn" onclick={openEditNameModal} aria-label="修改昵称">
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path><path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"></path></svg>
          </button>
          <button class="icon-btn" onclick={signOut} aria-label={$_('actions.signOut')}>
            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"></path><polyline points="16 17 21 12 16 7"></polyline><line x1="21" y1="12" x2="9" y2="12"></line></svg>
          </button>
        </div>
      </header>

      <nav class="tab-nav">
        <button
          class="tab-btn"
          class:active={tabView === 'signin'}
          onclick={() => switchTab('signin')}
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"></path><path d="M13.73 21a2 2 0 0 1-3.46 0"></path></svg>
          {$_('tabs.signin')}
        </button>
        <button
          class="tab-btn"
          class:active={tabView === 'supervision'}
          onclick={() => switchTab('supervision')}
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path><circle cx="12" cy="12" r="3"></circle></svg>
          {$_('tabs.supervision')}
        </button>
      </nav>

      {#if tabView === 'signin'}
        <div class="signin-view">
          <div class="streak-card" in:scale={{ delay: 200, start: 0.95 }}>
            <div class="fire-icon">🔥</div>
            <div class="streak-count">{streak}</div>
            <div class="streak-label">{$_('dashboard.streakLabel')}</div>
          </div>

          <div class="actions">
            {#if !isSignedIn}
              <button class="primary-btn" onclick={handleSignIn} disabled={isLoading}>
                {#if isLoading}
                  <span class="loader"></span>
                {:else}
                  {$_('dashboard.signinButton')}
                {/if}
              </button>
            {:else}
              <button class="secondary-btn" onclick={() => showQuote = true}>
                {$_('dashboard.viewQuote')}
              </button>
            {/if}
          </div>

          <div class="requests-section">
            <h3>{$_('supervision.pendingRequests')}</h3>
            {#if pendingRequests.length === 0}
              <p class="no-requests">{$_('supervision.noPendingRequests')}</p>
            {:else}
              {#each pendingRequests as request}
                <div class="request-card">
                  <span class="request-info">
                    <strong>{request.supervisor_id}</strong>
                    {$_('supervision.wantsToSupervise')}
                  </span>
                  <div class="request-actions">
                    <button class="accept-btn" onclick={() => acceptRequest(request)}>
                      {$_('supervision.accept')}
                    </button>
                    <button class="reject-btn" onclick={() => rejectRequest(request)}>
                      {$_('supervision.reject')}
                    </button>
                  </div>
                </div>
              {/each}
            {/if}
          </div>
        </div>

      {:else if tabView === 'supervision'}
        <div class="supervision-view">
          <div class="add-supervision-card">
            <h3>{$_('supervision.addDevice')}</h3>
            <form onsubmit={requestSupervision}>
              <div class="search-container">
                <div class="input-group">
                  <input
                    id="search_query"
                    type="text"
                    placeholder={$_('supervision.searchPlaceholder')}
                    bind:value={searchQuery}
                    oninput={searchDevices}
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
                        onclick={() => selectDevice(device)}
                        onkeydown={(e) => e.key === 'Enter' && selectDevice(device)}
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
                      {#if device.signed_in_today}
                        <span class="status-badge status-checked-in">✓ {$_('supervision.checkedInToday')}</span>
                      {:else}
                        <span class="status-badge status-not-checked-in">✗ {$_('supervision.notCheckedInToday')}</span>
                      {/if}
                    </p>
                  </div>
                  <button class="remove-btn" onclick={() => removeRelation(device.device_id)}>
                    {$_('supervision.remove')}
                  </button>
                </div>
              {/each}
            {/if}
          </div>
        </div>
      {/if}
    </div>
  {/if}

  {#if showQuote && quote}
    <div
      class="modal-overlay"
      role="button"
      tabindex="0"
      onclick={closeQuote}
      onkeydown={(e) => e.key === 'Escape' && closeQuote()}
      transition:fade
    >
      <div
        class="quote-card"
        in:scale={{ start: 0.9 }}
        role="dialog"
        aria-modal="true"
        tabindex="-1"
        onclick={(e) => e.stopPropagation()}
        onkeydown={(e) => e.stopPropagation()}
      >
        <button class="close-btn" onclick={closeQuote} aria-label={$_('actions.close')}>&times;</button>
        <blockquote>
          "{quote.text}"
        </blockquote>
        <cite>{$_('quote.authorPrefix')} {quote.author}</cite>
      </div>
    </div>
   {/if}

  {#if showEditNameModal}
    <div
      class="modal-overlay"
      role="button"
      tabindex="0"
      onclick={closeEditNameModal}
      onkeydown={(e) => e.key === 'Escape' && closeEditNameModal()}
      transition:fade
    >
      <div
        class="edit-name-card"
        in:scale={{ start: 0.9 }}
        role="dialog"
        aria-modal="true"
        tabindex="-1"
        onclick={(e) => e.stopPropagation()}
        onkeydown={(e) => e.stopPropagation()}
      >
        <button class="close-btn" onclick={closeEditNameModal} aria-label={$_('actions.close')}>&times;</button>
        <h3>修改昵称</h3>

        <div class="edit-name-info">
          <p>当前昵称：<strong>{name}</strong></p>
          {#if storage.getLastNameUpdate()}
            {@const daysSinceUpdate = Math.floor((Date.now() - storage.getLastNameUpdate()!.getTime()) / (1000 * 60 * 60 * 24))}
            <p class="cooldown-info">
              距离上次修改已经过去 {daysSinceUpdate} 天
              {#if daysSinceUpdate < 15}
                <br />
                <span class="cooldown-warning">还需要等待 {15 - daysSinceUpdate} 天才能再次修改</span>
              {:else}
                <span class="cooldown-ok">✓ 可以修改</span>
              {/if}
            </p>
          {:else}
            <p class="cooldown-info cooldown-ok">您还未修改过昵称，可以修改</p>
          {/if}
        </div>

        <form onsubmit={handleNameUpdate}>
          <div class="input-group">
            <input
              id="new_name"
              type="text"
              placeholder="新昵称"
              bind:value={newName}
              disabled={isLoading || !canUpdateName()}
              autocomplete="off"
            />
            <label for="new_name" class:has-value={newName.length > 0}>新昵称</label>
          </div>

          {#if editError}
            <div class="error-message">{editError}</div>
          {/if}

          <button type="submit" class="primary-btn" disabled={!newName.trim() || isLoading || !canUpdateName()}>
            {#if isLoading}
              <span class="loader"></span>
            {:else}
              确认修改
            {/if}
          </button>
        </form>
      </div>
    </div>
  {/if}
 </main>

<style>
  :global(:root) {
    --primary: #3730a3;
    --primary-hover: #312e81;
    --bg-color: #f1f5f9;
    --card-bg: #ffffff;
    --text-main: #020617;
    --text-muted: #334155;
    --border-color: #94a3b8;
    --focus-ring: #4f46e5;
    --shadow-sm: none;
    --shadow-md: 0 4px 6px -1px rgb(0 0 0 / 0.1);
    --shadow-lg: 0 10px 15px -3px rgb(0 0 0 / 0.1);
    --success: #22c55e;
    --error: #ef4444;
    font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
    font-size: 18px;
  }

  @media (prefers-color-scheme: dark) {
    :global(:root) {
      --primary: #818cf8;
      --primary-hover: #a5b4fc;
      --bg-color: #020617;
      --card-bg: #1e293b;
      --text-main: #ffffff;
      --text-muted: #e2e8f0;
      --border-color: #64748b;
      --focus-ring: #6366f1;
    }
  }

  :global(body) {
    margin: 0;
    padding: 0;
    background-color: var(--bg-color);
    color: var(--text-main);
    line-height: 1.6;
    overflow-x: hidden;
  }

  .container {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 2rem;
    position: relative;
    box-sizing: border-box;
  }

  .login-card {
    background: var(--card-bg);
    padding: 3rem;
    border-radius: 12px;
    width: 100%;
    max-width: 480px;
    box-shadow: var(--shadow-md);
    text-align: center;
    border: 2px solid var(--border-color);
  }

  h1 {
    font-size: 2.5rem;
    font-weight: 800;
    margin: 0 0 0.75rem;
    letter-spacing: -0.025em;
    color: var(--text-main);
  }

  .subtitle {
    color: var(--text-muted);
    margin: 0 0 2rem;
    font-size: 1.25rem;
    line-height: 1.5;
  }

  .mode-selection {
    display: flex;
    gap: 1rem;
    margin-bottom: 2rem;
    justify-content: center;
  }

  .radio-option {
    flex: 1;
    padding: 1rem;
    border: 2px solid var(--border-color);
    background: var(--card-bg);
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s ease;
    color: var(--text-muted);
    font-weight: 600;
    font-size: 1rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .radio-option:hover {
    border-color: var(--primary);
    background: var(--bg-color);
  }

  .radio-option input[type="radio"] {
    width: 20px;
    height: 20px;
    cursor: pointer;
  }

  .radio-option:has(input:checked) {
    border-color: var(--primary);
    background: var(--primary);
    color: white;
  }

  .input-group {
    position: relative;
    margin-bottom: 2rem;
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

  .secondary-btn {
    width: 100%;
    padding: 1.25rem;
    background: var(--card-bg);
    color: var(--text-main);
    font-weight: 700;
    font-size: 1.25rem;
    border: 2px solid var(--border-color);
    border-radius: 12px;
    min-height: 64px;
  }

  .secondary-btn:hover {
    background: var(--bg-color);
    border-color: var(--text-muted);
  }

  .dashboard {
    width: 100%;
    max-width: 900px;
    height: 100%;
    display: flex;
    flex-direction: column;
    padding: 1rem;
    box-sizing: border-box;
  }

  .dashboard-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem 0;
    margin-bottom: 1rem;
    border-bottom: 2px solid var(--border-color);
  }

  .header-actions {
    display: flex;
    gap: 1rem;
  }

  .language-btn-wrapper {
    position: relative;
  }

  .user-info h2 {
    margin: 0;
    font-size: 2rem;
    font-weight: 700;
  }

  .status-text {
    margin: 0.5rem 0 0;
    color: var(--text-muted);
    font-size: 1.1rem;
  }

  .icon-btn {
    background: var(--card-bg);
    color: var(--text-main);
    padding: 0.75rem;
    border-radius: 12px;
    border: 2px solid var(--border-color);
    width: 48px;
    height: 48px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .icon-btn:hover {
    background: var(--bg-color);
    border-color: var(--text-muted);
  }

  .language-menu {
    position: absolute;
    top: calc(100% + 8px);
    right: 0;
    background: var(--card-bg);
    border-radius: 12px;
    box-shadow: var(--shadow-lg);
    border: 2px solid var(--border-color);
    overflow: hidden;
    min-width: 160px;
    z-index: 50;
  }

  .language-option {
    width: 100%;
    padding: 1rem;
    background: transparent;
    border: none;
    color: var(--text-main);
    font-size: 1.1rem;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.2s ease;
    text-align: left;
  }

  .language-option:hover {
    background: var(--bg-color);
  }

  .language-option.active {
    background: var(--primary);
    color: white;
  }

  .tab-nav {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 2rem;
    border-bottom: 2px solid var(--border-color);
  }

  .tab-btn {
    flex: 1;
    padding: 1rem;
    background: transparent;
    color: var(--text-muted);
    font-weight: 600;
    font-size: 1.1rem;
    border-radius: 8px 8px 0 0;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    border: 2px solid transparent;
  }

  .tab-btn:hover {
    background: var(--bg-color);
  }

  .tab-btn.active {
    background: var(--primary);
    color: white;
  }

  .tab-btn svg {
    width: 20px;
    height: 20px;
  }

  .signin-view, .supervision-view {
    animation: fadeIn 0.3s ease;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .streak-card {
    background: var(--card-bg);
    border-radius: 20px;
    padding: 3rem;
    text-align: center;
    box-shadow: var(--shadow-md);
    border: 2px solid var(--border-color);
    margin: 1rem 0 2rem;
  }

  .fire-icon {
    font-size: 5rem;
    margin-bottom: 0.5rem;
  }

  .streak-count {
    font-size: 8rem;
    font-weight: 900;
    line-height: 1;
    color: var(--primary);
    background: none;
    -webkit-text-fill-color: var(--primary);
    margin: 1rem 0;
  }

  .streak-label {
    font-size: 1.5rem;
    font-weight: 600;
    color: var(--text-muted);
    margin-top: 0.5rem;
  }

  .actions {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    margin-bottom: 2rem;
  }

  .requests-section {
    background: var(--card-bg);
    border-radius: 12px;
    padding: 2rem;
    border: 2px solid var(--border-color);
  }

  .requests-section h3 {
    margin: 0 0 1.5rem;
    font-size: 1.5rem;
    color: var(--text-main);
  }

  .no-requests, .no-devices {
    color: var(--text-muted);
    font-size: 1.1rem;
  }

  .request-card {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    background: var(--bg-color);
    border-radius: 8px;
    margin-bottom: 1rem;
  }

  .request-info {
    font-size: 1.1rem;
  }

  .request-actions {
    display: flex;
    gap: 0.5rem;
  }

  .accept-btn {
    padding: 0.5rem 1rem;
    background: var(--success);
    color: white;
    border-radius: 6px;
    font-weight: 600;
  }

  .reject-btn {
    padding: 0.5rem 1rem;
    background: var(--error);
    color: white;
    border-radius: 6px;
    font-weight: 600;
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
    position: static;
    font-size: 1.1rem;
    font-weight: 600;
    color: var(--text-main);
    margin-bottom: 0.5rem;
    display: block;
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

  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.7);
    backdrop-filter: blur(6px);
    z-index: 100;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2rem;
    box-sizing: border-box;
    cursor: pointer;
  }

  .quote-card {
    background: var(--card-bg);
    padding: 3.5rem;
    border-radius: 24px;
    max-width: 600px;
    width: 100%;
    position: relative;
    box-shadow: var(--shadow-lg);
    text-align: center;
    cursor: default;
    border: 2px solid var(--border-color);
  }

  blockquote {
    margin: 1.5rem 0 2rem;
    font-size: 2.25rem;
    font-weight: 500;
    font-family: serif;
    line-height: 1.4;
    color: var(--text-main);
  }

  cite {
    font-style: normal;
    font-weight: 700;
    color: var(--text-muted);
    font-size: 1.25rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    display: block;
    margin-top: 1rem;
  }

  .close-btn {
    position: absolute;
    top: 1rem;
    right: 1rem;
    background: transparent;
    font-size: 2.5rem;
    color: var(--text-muted);
    width: 64px;
    height: 64px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    border: 2px solid transparent;
    transition: all 0.2s;
  }

  .close-btn:hover {
    background: var(--bg-color);
    color: var(--text-main);
    border-color: var(--border-color);
  }

  .edit-name-card {
    background: var(--card-bg);
    padding: 2.5rem;
    border-radius: 20px;
    max-width: 480px;
    width: 100%;
    position: relative;
    box-shadow: var(--shadow-lg);
    text-align: center;
    cursor: default;
    border: 2px solid var(--border-color);
  }

  .edit-name-card h3 {
    margin: 0 0 1.5rem;
    font-size: 1.75rem;
    color: var(--text-main);
  }

  .edit-name-info {
    background: var(--bg-color);
    padding: 1.25rem;
    border-radius: 12px;
    margin-bottom: 1.5rem;
    text-align: left;
    border: 2px solid var(--border-color);
  }

  .edit-name-info p {
    margin: 0.5rem 0;
    color: var(--text-muted);
    font-size: 1rem;
  }

  .edit-name-info strong {
    color: var(--text-main);
  }

  .cooldown-info {
    line-height: 1.6;
  }

  .cooldown-warning {
    color: var(--error);
    font-weight: 600;
  }

  .cooldown-ok {
    color: var(--success);
    font-weight: 600;
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
