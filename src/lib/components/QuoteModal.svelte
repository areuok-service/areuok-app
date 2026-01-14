<script lang="ts">
  import { _ } from 'svelte-i18n';
  import { fade, scale } from 'svelte/transition';

  interface Quote {
    text: string;
    author: string;
  }

  interface Props {
    quote: Quote;
    onClose: () => void;
  }

  let { quote, onClose }: Props = $props();

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
    class="quote-card"
    in:scale={{ start: 0.9 }}
    role="dialog"
    aria-modal="true"
    tabindex="-1"
    onclick={stopPropagation}
    onkeydown={stopPropagation}
  >
    <button class="close-btn" onclick={onClose} aria-label={$_('actions.close')}>&times;</button>
    <blockquote>
      "{quote.text}"
    </blockquote>
    <cite>{$_('quote.authorPrefix')} {quote.author}</cite>
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
    cursor: pointer;
    font-family: inherit;
  }

  .close-btn:hover {
    background: var(--bg-color);
    color: var(--text-main);
    border-color: var(--border-color);
  }
</style>