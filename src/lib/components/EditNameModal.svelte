<script lang="ts">
  import { _ } from 'svelte-i18n';
  import { fade, scale } from 'svelte/transition';

  interface Props {
    currentName: string;
    newName: string;
    isLoading: boolean;
    editError: string;
    onClose: () => void;
    onSubmit: (event: Event) => void;
    canUpdateName: () => boolean;
    daysSinceUpdate: number;
  }

  let {
    currentName,
    newName = $bindable(),
    isLoading,
    editError,
    onClose,
    onSubmit,
    canUpdateName,
    daysSinceUpdate
  }: Props = $props();

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      onClose();
    }
  }

  function handleOverlayClick() {
    onClose();
  }

  function stopPropagation(e: Event) {
    e.stopPropagation();
  }
</script>

<div
  class="modal-overlay"
  role="button"
  tabindex="0"
  onclick={handleOverlayClick}
  onkeydown={handleKeydown}
  transition:fade
>
  <div
    class="edit-name-card"
    in:scale={{ start: 0.9 }}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
    onclick={stopPropagation}
    onkeydown={stopPropagation}
  >
    <button class="close-btn" onclick={onClose} aria-label={$_('actions.close')}>&times;</button>
    <h3>修改昵称</h3>

    <div class="edit-name-info">
      <p>当前昵称：<strong>{currentName}</strong></p>
      {#if daysSinceUpdate >= 0}
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

    <form onsubmit={onSubmit}>
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

<style>
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

  .input-group {
    position: relative;
    margin-bottom: 1.5rem;
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

  input:disabled {
    opacity: 0.6;
    cursor: not-allowed;
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

  .error-message {
    color: var(--error);
    margin-bottom: 1rem;
    padding: 1rem;
    background: rgba(239, 68, 68, 0.1);
    border-radius: 8px;
    border: 1px solid var(--error);
    text-align: left;
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
    cursor: pointer;
    font-family: inherit;
  }

  .close-btn:hover {
    background: var(--bg-color);
    color: var(--text-main);
    border-color: var(--border-color);
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