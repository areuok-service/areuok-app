<script lang="ts">
  import { _ } from 'svelte-i18n';
  import { scale } from 'svelte/transition';

  type SupervisionRequest = {
    request_id: string;
    supervisor_id: string;
    supervisor_name?: string;
    target_id: string;
    target_name?: string;
    status: 'pending' | 'accepted' | 'rejected';
    created_at: string;
  };

  interface Props {
    streak: number;
    isSignedIn: boolean;
    isLoading: boolean;
    pendingRequests: SupervisionRequest[];
    onSignIn: (event: Event) => void;
    onShowQuote: () => void;
    onAcceptRequest: (request: SupervisionRequest) => void;
    onRejectRequest: (request: SupervisionRequest) => void;
  }

  let {
    streak,
    isSignedIn,
    isLoading,
    pendingRequests,
    onSignIn,
    onShowQuote,
    onAcceptRequest,
    onRejectRequest
  }: Props = $props();
</script>

<div class="signin-view">
  <div class="streak-card" in:scale={{ delay: 200, start: 0.95 }}>
    <div class="fire-icon">🔥</div>
    <div class="streak-count">{streak}</div>
    <div class="streak-label">{$_('dashboard.streakLabel')}</div>
  </div>

  <div class="actions">
    {#if !isSignedIn}
      <button class="primary-btn" onclick={onSignIn} disabled={isLoading}>
        {#if isLoading}
          <span class="loader"></span>
        {:else}
          {$_('dashboard.signinButton')}
        {/if}
      </button>
    {:else}
      <button class="secondary-btn" onclick={onShowQuote}>
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
            <button class="accept-btn" onclick={() => onAcceptRequest(request)}>
              {$_('supervision.accept')}
            </button>
            <button class="reject-btn" onclick={() => onRejectRequest(request)}>
              {$_('supervision.reject')}
            </button>
          </div>
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>
  .signin-view {
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

  .no-requests {
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