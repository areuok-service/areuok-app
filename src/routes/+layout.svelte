<script>
  import { waitLocale } from 'svelte-i18n';
  import '../lib/i18n';
  import '../lib/styles/global.css';

  let { children } = $props();
</script>

{#await waitLocale()}
  <!-- Loading... -->
{:then}
  {@render children()}
{/await}