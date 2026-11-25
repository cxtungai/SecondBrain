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

// Nút chép một đoạn chữ có sẵn (vd. lời nhờ Claude sửa bản nháp).
document.querySelectorAll('[data-copy-text]').forEach((btn) => {
  btn.addEventListener('click', async () => {
    const original = btn.textContent;
    try {
      await navigator.clipboard.writeText(btn.dataset.copyText);
      btn.textContent = btn.dataset.copied || 'Đã chép';
    } catch {
      window.prompt('Chép đoạn này rồi dán vào Claude:', btn.dataset.copyText);
    }
    setTimeout(() => (btn.textContent = original), 2500);
  });
});

// Khu duyệt bài: ai đã mở /duyet/ trên trình duyệt này thì thấy nút "Chờ duyệt" trên mọi trang.
try {
  if (location.pathname.includes('/duyet/')) localStorage.setItem('sb-reviewer', '1');
  if (localStorage.getItem('sb-reviewer') === '1') {
    document.querySelectorAll('[data-reviewer-link]').forEach((el) => (el.hidden = false));
  }
} catch {
  /* trình duyệt chặn bộ nhớ — nút vẫn có ở chân trang */
}

// Bấm "Hướng dẫn chi tiết" thì mở phần hướng dẫn đang thu gọn.
document.querySelectorAll('a[href="#cach-duyet"]').forEach((a) =>
  a.addEventListener('click', () => {
    const d = document.getElementById('cach-duyet');
    if (d) d.open = true;
  })
);
