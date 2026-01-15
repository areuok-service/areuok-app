<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
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
  import Toast from '../lib/components/Toast.svelte';
  import { toastStore } from '../lib/stores/toast';

  type DeviceMode = 'signin' | 'supervisor';

  const DEVICE_ID_KEY = 'areuok_device_id';
  const DEVICE_NAME_KEY = 'areuok_device_name';
  const DEVICE_MODE_KEY = 'areuok_device_mode';
  const DEVICE_IMEI_KEY = 'areuok_device_imei';
  const LAST_NAME_UPDATE_KEY = 'areuok_last_name_update';

  const storage = {
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

    getDeviceMode: (): 'signin' | 'supervisor' | null => {
      if (typeof window === 'undefined') return null;
      return localStorage.getItem(DEVICE_MODE_KEY) as 'signin' | 'supervisor' | null;
    },

    setDeviceMode: (mode: 'signin' | 'supervisor'): void => {
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

  // Remote API types (matching server responses)
  type Device = {
    device_id: string;
    device_name: string;
    imei?: string;
    mode: 'signin' | 'supervisor';
    created_at: string;
    last_seen_at: string;
    last_name_updated_at?: string;
  };

  type SupervisionRequest = {
    request_id: string;
    supervisor_id: string;
    supervisor_name?: string;
    target_id: string;
    target_name?: string;
    status: 'pending' | 'accepted' | 'rejected';
    created_at: string;
  };

  type SupervisionRelation = {
    relation_id: string;
    supervisor_id: string;
    supervisor_name?: string;
    target_id: string;
    target_name?: string;
    created_at: string;
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

  // Toast state
  let toasts = $state<
    Array<{ id: number; message: string; type: 'success' | 'error' | 'info' | 'warning'; duration: number }>
  >([]);

  // Toast store subscription
  $effect(() => {
    const unsubscribe = toastStore.subscribe((messages) => {
      toasts = messages;
    });
    return () => {
      unsubscribe();
    };
  });

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

      const device = await invoke<Device>('device_register', {
        deviceName: name,
        imei: imei || null,
        mode: deviceMode
      });
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
      toastStore.error($_('error.registrationFailed') + ': ' + (error as Error).message);
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
        const device = await invoke<Device>('device_get_info', { deviceId: savedDeviceId });
        view = 'dashboard';

        // Check today's sign-in status
        if (deviceMode === 'signin') {
          const deviceStatus = await invoke<DeviceStatus>('device_get_status', { deviceId: savedDeviceId });
          isSignedIn = isSignedInToday(deviceStatus.last_signin);
          streak = deviceStatus.streak;
        }

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
        invoke<SupervisionRelation[]>('supervision_list_api', { deviceId }),
        invoke<SupervisionRequest[]>('supervision_get_pending', { deviceId })
      ]);

      supervisionList = relations;
      pendingRequests = requests;

      const newDeviceRelationMap = new Map<string, string>();
      relations.forEach((rel: SupervisionRelation) => {
        newDeviceRelationMap.set(rel.target_id, rel.relation_id);
      });
      deviceRelationMap = newDeviceRelationMap;

      if (relations.length > 0) {
        const deviceStatuses = await Promise.all(
          relations.map((rel: SupervisionRelation) =>
            invoke<DeviceStatus>('device_get_status', { deviceId: rel.target_id })
          )
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
      const result = await invoke<{ streak: number }>('device_signin_api', { deviceId });
      isSignedIn = true;
      streak = result.streak;

      setTimeout(() => {
        showQuote = true;
      }, 500);
    } catch (error) {
      console.error(error);
      toastStore.error($_('error.signinFailed') + ': ' + (error as Error).message);
    } finally {
      isLoading = false;
    }
  }

  // Search handler
  async function searchDevices(event: Event) {
    event.preventDefault();
    if (searchQuery.length < 2) return;

    try {
      searchResults = await invoke<Device[]>('device_search', { query: searchQuery });
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
      await invoke('supervision_request_api', {
        supervisorId: deviceId,
        targetId: targetDeviceId
      });
      toastStore.success($_('supervision.requestSent'));
      targetDeviceId = '';
      searchQuery = '';
      searchResults = [];
      showSearchResults = false;
      await loadSupervisionData();
      tabView = 'supervision';
    } catch (error) {
      requestError = (error as Error).message;
      toastStore.error($_('error.requestFailed') + ': ' + (error as Error).message);
    } finally {
      isLoading = false;
    }
  }

  // Accept supervision request
  async function acceptRequest(request: SupervisionRequest) {
    if (!deviceId) return;

    try {
      await invoke('supervision_accept_api', {
        supervisorId: request.supervisor_id,
        targetId: request.target_id
      });
      toastStore.success($_('supervision.requestAccepted'));
      await loadSupervisionData();
    } catch (error) {
      toastStore.error($_('error.acceptFailed') + ': ' + (error as Error).message);
    }
  }

  // Reject supervision request
  async function rejectRequest(request: SupervisionRequest) {
    if (!deviceId) return;

    try {
      await invoke('supervision_reject_api', {
        supervisorId: request.supervisor_id,
        targetId: request.target_id
      });
      toastStore.success($_('supervision.requestRejected'));
      await loadSupervisionData();
    } catch (error) {
      toastStore.error($_('error.rejectFailed') + ': ' + (error as Error).message);
    }
  }

  // Remove supervision relation
  async function removeRelation(deviceIdToRemove: string) {
    const relationId = deviceRelationMap.get(deviceIdToRemove);
    if (!relationId) return;

    if (!confirm($_('supervision.confirmRemove'))) return;

    try {
      await invoke('supervision_remove_api', { relationId });
      toastStore.success($_('supervision.relationRemoved'));
      await loadSupervisionData();
    } catch (error) {
      toastStore.error($_('error.removeFailed') + ': ' + (error as Error).message);
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
      const device = await invoke<Device>('device_update_name_api', {
        deviceId,
        newName
      });
      name = device.device_name;
      storage.setDeviceName(name);
      storage.setLastNameUpdate(new Date());

      showEditNameModal = false;
      newName = '';
      toastStore.success($_('success.nameUpdated'));
    } catch (error) {
      console.error('Failed to update name:', error);
      editError = $_('error.nameUpdateFailed') + ': ' + (error as Error).message;
      toastStore.error($_('error.nameUpdateFailed') + ': ' + (error as Error).message);
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
        daysSinceUpdate={
          (() => {
            const lastUpdate = storage.getLastNameUpdate();
            if (!lastUpdate) return -1;
            return Math.floor((Date.now() - lastUpdate.getTime()) / (1000 * 60 * 60 * 24));
          })()
        }
      />
  {/if}

  {#each toasts as toast (toast.id)}
    <Toast
      message={toast.message}
      type={toast.type}
      duration={toast.duration}
      onDismiss={() => toastStore.dismiss(toast.id)}
    />
  {/each}
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