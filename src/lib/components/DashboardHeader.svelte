<script lang="ts">
  import { _, locale } from 'svelte-i18n';

  interface Props {
    name: string;
    showLanguageMenu: boolean;
    onToggleLanguageMenu: () => void;
    onCloseLanguageMenu: () => void;
    onSetLanguage: (lang: string) => void;
    onEditName: () => void;
    onSignOut: () => void;
  }

  let {
    name,
    showLanguageMenu,
    onToggleLanguageMenu,
    onSetLanguage,
    onEditName,
    onSignOut
  }: Props = $props();
</script>

<header class="dashboard-header">
  <div class="user-info">
    <h2>{$_('dashboard.greeting', { values: { name } })}</h2>
    <p class="status-text">{$_('dashboard.status')}</p>
  </div>
  <div class="header-actions">
    <div class="language-btn-wrapper">
      <button class="icon-btn" onclick={onToggleLanguageMenu} aria-label={$_('language.title')}>
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <circle cx="12" cy="12" r="10"></circle>
          <line x1="2" y1="12" x2="22" y2="12"></line>
          <path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"></path>
        </svg>
      </button>
      {#if showLanguageMenu}
        <div class="language-menu" role="menu">
          <button
            class="language-option"
            class:active={$locale === 'en'}
            onclick={() => onSetLanguage('en')}
            role="menuitem"
          >
            {$_('language.en')}
          </button>
          <button
            class="language-option"
            class:active={$locale === 'zh-CN'}
            onclick={() => onSetLanguage('zh-CN')}
            role="menuitem"
          >
            {$_('language.zhCN')}
          </button>
        </div>
      {/if}
    </div>
    <button class="icon-btn" onclick={onEditName} aria-label="修改昵称">
      <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path>
        <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"></path>
      </svg>
    </button>
    <button class="icon-btn" onclick={onSignOut} aria-label={$_('actions.signOut')}>
      <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"></path>
        <polyline points="16 17 21 12 16 7"></polyline>
        <line x1="21" y1="12" x2="9" y2="12"></line>
      </svg>
    </button>
  </div>
</header>

<style>
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
    cursor: pointer;
    transition: all 0.2s;
    font-family: inherit;
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
    font-family: inherit;
  }

  .language-option:hover {
    background: var(--bg-color);
  }

  .language-option.active {
    background: var(--primary);
    color: white;
  }
</style>