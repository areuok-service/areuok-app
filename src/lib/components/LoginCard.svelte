<script lang="ts">
  import { _ } from 'svelte-i18n';
  import { fly } from 'svelte/transition';

  type DeviceMode = 'signin' | 'supervisor';

  interface Props {
    name: string;
    deviceMode: DeviceMode;
    isLoading: boolean;
    onSubmit: (event: Event) => void;
  }

  let { name = $bindable(), deviceMode = $bindable(), isLoading, onSubmit }: Props = $props();
</script>

<div class="login-card" in:fly={{ y: 20, duration: 600 }}>
  <header>
    <h1>{$_('app.title')}</h1>
    <p class="subtitle">{$_('app.subtitle')}</p>
  </header>

  <form onsubmit={onSubmit}>
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

<style>
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

  header h1 {
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