<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { deviceApi, supervisionApi, storage } from '../lib/api';
  import type { DeviceMode, SupervisionRelation, SupervisionRequest, DeviceStatus, Device } from '../lib/api';
  import { fade } from 'svelte/transition';
  import { locale, _ } from 'svelte-i18n';
  import {
    LoginCard,
    QuoteModal,
    EditNameModal,
    DashboardHeader,
    SigninView,
    SupervisionView,
    TabNav
  } from '../lib/components';

  // Types
  interface Quote {
    text: string;
    author: string;
  }

  type View = 'login' | 'dashboard';
  type TabView = 'signin' | 'supervision';

  // State
  let view = $state<View>('login');
  let tabView = $state<TabView>('signin');
  let name = $state('');
  let deviceId = $state<string | null>(null);
  let deviceMode = $state<DeviceMode>('signin');
  let isSignedIn = $state(false);
  let streak = $state(0);
  let isLoading = $state(false);
  let showQuote = $state(false);
  let quote = $state<Quote | null>(null);

  // Supervision state
  let supervisionList = $state<SupervisionRelation[]>([]);
  let pendingRequests = $state<SupervisionRequest[]>([]);
  let supervisedDevices = $state<DeviceStatus[]>([]);
  let deviceRelationMap = $state<Map<string, string>>(new Map());
  let isLoadingSupervision = $state(false);

  // Search state
  let searchQuery = $state('');
  let searchResults = $state<Device[]>([]);
  let showSearchResults = $state(false);

  // Request state
  let targetDeviceId = $state('');
  let requestError = $state('');

  // Edit name modal state
  let showEditNameModal = $state(false);
  let newName = $state('');
  let editError = $state('');

  // Language menu state
  let showLanguageMenu = $state(false);

  // Utility functions
  function getTodayDate(): string {
    return new Date().toISOString().split('T')[0];
  }

  // Quote functions
  async function loadDailyQuote() {
    try {
      quote = await invoke<Quote>('get_daily_quote');
    } catch (error) {
      console.error('Failed to load quote:', error);
    }
  }

  // Registration handler
  async function handleRegister(event: Event) {
    event.preventDefault();
    if (!name.trim()) return;

    isLoading = true;

    try {
      let imei: string | undefined;
      try {
        imei = await invoke<string>('get_device_imei');
      } catch (e) {
        console.error('Failed to get IMEI:', e);
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
      console.error('Registration failed:', error);
      alert('Registration failed: ' + (error as Error).message);
    } finally {
      isLoading = false;
    }
  }

  // Storage loading
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
        view = 'dashboard';
        await loadSupervisionData();
      } catch (error) {
        console.error('Failed to load device:', error);
        storage.clearDevice();
        view = 'login';
      }
    }

    await loadDailyQuote();
  }

  // Supervision data loading
  async function loadSupervisionData() {
    if (!deviceId) return;

    isLoadingSupervision = true;
    try {
      const [relations, requests] = await Promise.all([
        supervisionApi.list(deviceId),
        supervisionApi.getPending(deviceId)
      ]);

      supervisionList = relations;
      pendingRequests = requests;

      const newDeviceRelationMap = new Map<string, string>();
      relations.forEach((rel) => {
        newDeviceRelationMap.set(rel.target_id, rel.relation_id);
      });
      deviceRelationMap = newDeviceRelationMap;

      if (relations.length > 0) {
        const deviceStatuses = await Promise.all(
          relations.map((rel) => deviceApi.getStatus(rel.target_id))
        );
        supervisedDevices = deviceStatuses;
      } else {
        supervisedDevices = [];
      }
    } catch (error) {
      console.error('Failed to load supervision data:', error);
    } finally {
      isLoadingSupervision = false;
    }
  }

  // Sign-in handler
  async function handleSignIn(event: Event) {
    event.preventDefault();
    if (!name.trim() || !deviceId) return;

    isLoading = true;

    try {
      await new Promise((r) => setTimeout(r, 800));
      const result = await deviceApi.signin(deviceId);
      isSignedIn = true;
      streak = result.streak;

      setTimeout(() => {
        showQuote = true;
      }, 500);
    } catch (error) {
      console.error(error);
      alert('Sign in failed: ' + (error as Error).message);
    } finally {
      isLoading = false;
    }
  }

  // Search handler
  async function searchDevices(event: Event) {
    event.preventDefault();
    if (searchQuery.length < 2) return;

    try {
      searchResults = await deviceApi.search(searchQuery);
      showSearchResults = true;
    } catch (error) {
      console.error('Search failed:', error);
    }
  }

  // Device selection
  function selectDevice(device: Device) {
    targetDeviceId = device.device_id;
    showSearchResults = false;
    searchQuery = device.device_name;
  }

  // Supervision request
  async function requestSupervision(event: Event) {
    event.preventDefault();
    if (!targetDeviceId || !deviceId) return;

    requestError = '';
    isLoading = true;

    try {
      await supervisionApi.request(deviceId, targetDeviceId);
      alert($_('supervision.requestSent'));
      targetDeviceId = '';
      searchQuery = '';
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

  // Accept supervision request
  async function acceptRequest(request: SupervisionRequest) {
    if (!deviceId) return;

    try {
      await supervisionApi.accept(request.supervisor_id, request.target_id);
      await loadSupervisionData();
    } catch (error) {
      alert('Failed to accept request: ' + (error as Error).message);
    }
  }

  // Reject supervision request
  async function rejectRequest(request: SupervisionRequest) {
    if (!deviceId) return;

    try {
      await supervisionApi.reject(request.supervisor_id, request.target_id);
      await loadSupervisionData();
    } catch (error) {
      alert('Failed to reject request: ' + (error as Error).message);
    }
  }

  // Remove supervision relation
  async function removeRelation(deviceIdToRemove: string) {
    const relationId = deviceRelationMap.get(deviceIdToRemove);
    if (!relationId) return;

    if (!confirm($_('supervision.confirmRemove'))) return;

    try {
      await supervisionApi.remove(relationId);
      await loadSupervisionData();
    } catch (error) {
      alert('Failed to remove supervision: ' + (error as Error).message);
    }
  }

  // Sign out
  async function signOut() {
    try {
      storage.clearDevice();
    } catch (error) {
      console.error('Failed to sign out:', error);
    }

    deviceId = null;
    name = '';
    streak = 0;
    isSignedIn = false;
    showQuote = false;
    view = 'login';
    supervisionList = [];
    pendingRequests = [];
    supervisedDevices = [];
  }

  // Quote modal handlers
  function closeQuote() {
    showQuote = false;
  }

  function handleShowQuote() {
    showQuote = true;
  }

  // Tab handlers
  function switchTab(newTab: TabView) {
    tabView = newTab;
  }

  // Language handlers
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

  // Name update handlers
  function canUpdateName(): boolean {
    const lastUpdate = storage.getLastNameUpdate();
    if (!lastUpdate) return true;

    const daysSinceUpdate = Math.floor((Date.now() - lastUpdate.getTime()) / (1000 * 60 * 60 * 24));
    return daysSinceUpdate >= 15;
  }

  function openEditNameModal() {
    newName = name;
    editError = '';
    showEditNameModal = true;
  }

  function closeEditNameModal() {
    showEditNameModal = false;
    newName = '';
    editError = '';
  }

  async function handleNameUpdate(event: Event) {
    event.preventDefault();
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
    editError = '';

    try {
      const device = await deviceApi.updateName(deviceId, newName);
      name = device.device_name;
      storage.setDeviceName(name);
      storage.setLastNameUpdate(new Date());

      showEditNameModal = false;
      newName = '';
    } catch (error) {
      console.error('Failed to update name:', error);
      editError = '修改昵称失败: ' + (error as Error).message;
    } finally {
      isLoading = false;
    }
  }

  // Initialize on mount
  loadFromStorage();
</script>

<main class="container">
  {#if view === 'login'}
    <LoginCard
      bind:name
      bind:deviceMode
      {isLoading}
      onSubmit={handleRegister}
    />
  {:else if view === 'dashboard'}
    <div class="dashboard" in:fade={{ duration: 400 }}>
      <DashboardHeader
        {name}
        {showLanguageMenu}
        onToggleLanguageMenu={toggleLanguageMenu}
        onCloseLanguageMenu={closeLanguageMenu}
        onSetLanguage={setLanguage}
        onEditName={openEditNameModal}
        onSignOut={signOut}
      />

      <TabNav activeTab={tabView} onSwitchTab={switchTab} />

      {#if tabView === 'signin'}
        <SigninView
          {streak}
          {isSignedIn}
          {isLoading}
          {pendingRequests}
          onSignIn={handleSignIn}
          onShowQuote={handleShowQuote}
          onAcceptRequest={acceptRequest}
          onRejectRequest={rejectRequest}
        />
      {:else if tabView === 'supervision'}
        <SupervisionView
          bind:searchQuery
          {searchResults}
          {showSearchResults}
          {targetDeviceId}
          {requestError}
          {isLoading}
          {isLoadingSupervision}
          {supervisedDevices}
          onSearch={searchDevices}
          onSelectDevice={selectDevice}
          onSubmitRequest={requestSupervision}
          onRemoveRelation={removeRelation}
        />
      {/if}
    </div>
  {/if}

  {#if showQuote && quote}
    <QuoteModal {quote} onClose={closeQuote} />
  {/if}

  {#if showEditNameModal}
    <EditNameModal
      currentName={name}
      bind:newName
      {isLoading}
      {editError}
      onClose={closeEditNameModal}
      onSubmit={handleNameUpdate}
      {canUpdateName}
    />
  {/if}
</main>

<style>
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

  .dashboard {
    width: 100%;
    max-width: 900px;
    height: 100%;
    display: flex;
    flex-direction: column;
    padding: 1rem;
    box-sizing: border-box;
  }
</style>