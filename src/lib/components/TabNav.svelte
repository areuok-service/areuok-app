<script lang="ts">
  import { _ } from 'svelte-i18n';

  type TabView = 'signin' | 'supervision';

  interface Props {
    activeTab: TabView;
    onSwitchTab: (tab: TabView) => void;
  }

  let { activeTab, onSwitchTab }: Props = $props();
</script>

<nav class="tab-nav">
  <button
    class="tab-btn"
    class:active={activeTab === 'signin'}
    onclick={() => onSwitchTab('signin')}
  >
    <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"></path>
      <path d="M13.73 21a2 2 0 0 1-3.46 0"></path>
    </svg>
    {$_('tabs.signin')}
  </button>
  <button
    class="tab-btn"
    class:active={activeTab === 'supervision'}
    onclick={() => onSwitchTab('supervision')}
  >
    <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path>
      <circle cx="12" cy="12" r="3"></circle>
    </svg>
    {$_('tabs.supervision')}
  </button>
</nav>

<style>
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
    cursor: pointer;
    font-family: inherit;
    transition: all 0.2s;
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
</style>