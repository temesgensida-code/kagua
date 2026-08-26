import { tick } from 'svelte';

let isDarkState = $state(true);

if (typeof window !== 'undefined') {
  const saved = localStorage.getItem('kagua_theme');
  if (saved === 'light') {
    isDarkState = false;
  }
  // Apply immediately on load
  document.documentElement.setAttribute('data-theme', isDarkState ? 'dark' : 'light');
}

export const themeState = {
  get isDark() {
    return isDarkState;
  },
  async toggle(event?: MouseEvent | TouchEvent | HTMLElement) {
    const isAppearanceTransition =
      typeof document !== 'undefined' &&
      'startViewTransition' in document &&
      !window.matchMedia('(prefers-reduced-motion: reduce)').matches;

    const applyThemeChange = async () => {
      isDarkState = !isDarkState;
      if (typeof window !== 'undefined') {
        const theme = isDarkState ? 'dark' : 'light';
        localStorage.setItem('kagua_theme', theme);
        document.documentElement.setAttribute('data-theme', theme);
      }
      await tick();
    };

    if (!isAppearanceTransition) {
      await applyThemeChange();
      return;
    }

    // Determine center coordinates of origin (theme button or click event)
    let x = window.innerWidth / 2;
    let y = window.innerHeight / 2;

    if (event && 'clientX' in event && typeof event.clientX === 'number' && event.clientX > 0) {
      x = event.clientX;
      y = event.clientY;
    } else if (event && 'getBoundingClientRect' in event && typeof event.getBoundingClientRect === 'function') {
      const rect = event.getBoundingClientRect();
      x = rect.left + rect.width / 2;
      y = rect.top + rect.height / 2;
    } else if (typeof document !== 'undefined') {
      const btn = document.querySelector('.theme-toggle-card');
      if (btn) {
        const rect = btn.getBoundingClientRect();
        x = rect.left + rect.width / 2;
        y = rect.top + rect.height / 2;
      }
    }

    const endRadius = Math.hypot(
      Math.max(x, window.innerWidth - x),
      Math.max(y, window.innerHeight - y)
    );

    const doc = document as unknown as {
      startViewTransition: (callback: () => Promise<void>) => { ready: Promise<void> };
    };

    const transition = doc.startViewTransition(async () => {
      await applyThemeChange();
    });

    await transition.ready;

    document.documentElement.animate(
      {
        clipPath: [
          `circle(0px at ${x}px ${y}px)`,
          `circle(${endRadius}px at ${x}px ${y}px)`
        ]
      },
      {
        duration: 400,
        easing: 'cubic-bezier(0.4, 0, 0.2, 1)',
        pseudoElement: '::view-transition-new(root)'
      }
    );
  }
};

