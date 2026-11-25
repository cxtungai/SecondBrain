// Tiện ích chung: nút sao chép, và (khi xem trên máy) tự tải lại trang khi file thay đổi.
document.querySelectorAll('[data-copy]').forEach((btn) => {
  btn.addEventListener('click', async () => {
    const el = document.querySelector(btn.dataset.copy);
    if (!el) return;
    const original = btn.textContent;
    try {
      await navigator.clipboard.writeText(el.textContent || '');
      btn.textContent = 'Đã chép';
    } catch {
      btn.textContent = 'Không chép được';
    }
    setTimeout(() => (btn.textContent = original), 1500);
  });
});

if (document.body.dataset.dev && !document.body.dataset.noReload) {
  let version = null;
  const poll = async () => {
    try {
      const r = await fetch('/__version', { cache: 'no-store' });
      const v = await r.text();
      if (version !== null && v !== version && !window.__dirty) location.reload();
      version = v;
    } catch {
      /* chương trình đã tắt — thử lại sau */
    }
    setTimeout(poll, 1200);
  };
  poll();
}
