/**
 * connectionStatus.svelte.ts
 *
 * Reactive network connection watcher for Kagua.
 * Exposes isOnline, tracks connection drops, and notifies components.
 */

let isOnline = $state<boolean>(typeof navigator !== 'undefined' ? navigator.onLine : true);

if (typeof window !== 'undefined') {
  window.addEventListener('online', () => {
    isOnline = true;
  });

  window.addEventListener('offline', () => {
    isOnline = false;
  });
}

export function getConnectionStatus() {
  return {
    get isOnline() {
      return isOnline;
    },
  };
}
