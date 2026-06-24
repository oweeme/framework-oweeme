// Framework Oweeme - JS mínimo para hidratación ligera

// Hidratación: actualiza datos desde la API sin recargar la página
async function hydrate(endpoint, selector, renderer) {
  try {
    const res = await fetch(endpoint, { headers: { Accept: 'application/json' } });
    if (!res.ok) return;
    const data = await res.json();
    const el = document.querySelector(selector);
    if (el) el.innerHTML = renderer(data);
  } catch (_) {}
}

// Registro de Service Worker (también en base.html por SSR)
if ('serviceWorker' in navigator) {
  navigator.serviceWorker.register('/sw.js').catch(() => {});
}
