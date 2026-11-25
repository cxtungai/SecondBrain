// Bàn làm việc: các nút Duyệt & đăng, Bỏ, Đưa lên web, tạo ghi chú, cài đặt, và trình sửa ghi chú.
(() => {
  const post = async (path, data) => {
    const r = await fetch('/api/' + path, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json', 'X-Second-Brain': '1' },
      body: JSON.stringify(data),
    });
    let json;
    try {
      json = await r.json();
    } catch {
      json = { ok: false, errors: ['Chương trình không phản hồi — kiểm tra cửa sổ Second Brain còn mở không.'] };
    }
    return json;
  };

  const toastEl = () => {
    let t = document.getElementById('toast');
    if (!t) {
      t = document.createElement('div');
      t.id = 'toast';
      t.className = 'toast';
      document.body.appendChild(t);
    }
    return t;
  };
  const toast = (html, kind = 'ok', ms = 6000) => {
    const t = toastEl();
    t.innerHTML = html;
    t.className = 'toast toast-' + kind;
    t.hidden = false;
    clearTimeout(t._timer);
    if (ms) t._timer = setTimeout(() => (t.hidden = true), ms);
  };
  const esc = (s) => String(s).replace(/[&<>"']/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' })[c]);
  const list = (items) => (items && items.length ? '<ul>' + items.map((x) => `<li>${esc(x)}</li>`).join('') + '</ul>' : '');

  async function publish(path, btn) {
    if (btn) btn.disabled = true;
    const res = await post('dang', { path });
    if (btn) btn.disabled = false;
    if (!res.ok) {
      toast('<strong>Chưa đăng được</strong>' + list(res.errors), 'bad', 0);
      return false;
    }
    const r = res.report;
    const html =
      `<strong>Đã đăng: ${esc(r.title)}</strong> — <a href="/${esc(r.page)}">xem trang</a>. ` +
      'Khi xong việc, bấm <em>Đưa lên web</em> trên Bàn làm việc để cập nhật Internet.' +
      (r.suggestions.length ? '<br>Gợi ý kết nối (nhờ Claude: “kết nối ghi chú vừa đăng”):' + list(r.suggestions.slice(0, 5)) : '');
    // Giữ thông báo qua lần tải lại trang.
    try {
      sessionStorage.setItem('sb-toast', html);
    } catch {}
    window.__dirty = true;
    location.href = location.pathname.startsWith('/ban-lam-viec/') ? '/ban-lam-viec/' : '/' + r.page;
    return true;
  }

  try {
    const pending = sessionStorage.getItem('sb-toast');
    if (pending) {
      sessionStorage.removeItem('sb-toast');
      toast(pending, 'ok', 15000);
    }
  } catch {}

  document.addEventListener('click', async (e) => {
    const btn = e.target.closest('[data-action]');
    if (!btn) return;
    const action = btn.dataset.action;
    if (action === 'publish') {
      e.preventDefault();
      if (!confirm('Đăng ghi chú này? Bạn xác nhận đã đọc và đồng ý với nội dung.')) return;
      await publish(btn.dataset.path, btn);
    } else if (action === 'delete') {
      e.preventDefault();
      if (!confirm(`Bỏ bản nháp “${btn.dataset.title}”? File được chuyển vào thùng rác (knowledge/_thung-rac), có thể lấy lại.`)) return;
      const res = await post('xoa', { path: btn.dataset.path });
      if (res.ok) location.reload();
      else toast(list(res.errors), 'bad', 0);
    } else if (action === 'push') {
      e.preventDefault();
      btn.disabled = true;
      btn.textContent = 'Đang đưa lên…';
      const log = document.getElementById('push-log');
      const res = await post('day-len-web', {});
      btn.textContent = '🚀 Đưa lên web';
      log.hidden = false;
      log.textContent = res.log || '';
      if (res.ok) {
        toast('<strong>Đã đưa lên GitHub.</strong> Trang web sẽ tự cập nhật sau 1–2 phút.', 'ok', 0);
        setTimeout(() => location.reload(), 3000);
      } else {
        btn.disabled = false;
        toast('<strong>Chưa đưa lên được.</strong> ' + esc(res.message || '') + ' Có thể nhờ Claude: “đưa thay đổi lên GitHub giúp tôi”.', 'bad', 0);
      }
    } else if (action === 'add-source') {
      const tpl = document.getElementById('source-template');
      document.getElementById('sources').appendChild(tpl.content.cloneNode(true));
      window.__dirty = true;
    } else if (action === 'remove-source') {
      btn.closest('.source-row').remove();
      window.__dirty = true;
    } else if (action === 'save-publish') {
      e.preventDefault();
      if (!confirm('Lưu và đăng? Bạn xác nhận đã đọc và đồng ý với nội dung.')) return;
      const path = await save();
      if (path) await publish(path, btn);
    }
  });

  // ----- Tạo ghi chú mới -----
  const newForm = document.getElementById('new-note');
  if (newForm) {
    newForm.addEventListener('submit', async (e) => {
      e.preventDefault();
      const data = Object.fromEntries(new FormData(newForm));
      const res = await post('tao', data);
      if (res.ok) location.href = res.edit_url;
      else toast(list(res.errors), 'bad', 0);
    });
  }

  // ----- Cài đặt -----
  const settings = document.getElementById('settings');
  if (settings) {
    settings.addEventListener('submit', async (e) => {
      e.preventDefault();
      const res = await post('cai-dat', Object.fromEntries(new FormData(settings)));
      toast(res.ok ? 'Đã lưu cài đặt.' : list(res.errors), res.ok ? 'ok' : 'bad');
    });
  }

  // ----- Trình sửa -----
  const editor = document.getElementById('editor');
  if (!editor) return;
  window.__dirty = true; // không tự tải lại trang khi đang sửa
  const body = editor.querySelector('[name=body]');
  const preview = document.getElementById('preview');
  let timer;
  const refreshPreview = async () => {
    const res = await post('xem-truoc', { body: body.value, kind: editor.dataset.kind });
    if (res.ok) preview.innerHTML = res.html;
  };
  body.addEventListener('input', () => {
    clearTimeout(timer);
    timer = setTimeout(refreshPreview, 400);
  });

  const collect = () => {
    const v = (n) => (editor.querySelector(`[name=${n}]`) || {}).value || '';
    const data = { path: editor.dataset.path, kind: editor.dataset.kind, body: body.value, fields: {} };
    for (const n of ['title', 'type', 'summary', 'tags', 'aliases', 'description', 'keyword', 'keywords', 'intent', 'based_on']) {
      const el = editor.querySelector(`[name=${n}]`);
      if (el) data.fields[n] = el.value;
    }
    data.sources = [...editor.querySelectorAll('.source-row')].map((row) => {
      const g = (n) => row.querySelector(`[name=${n}]`).value.trim();
      let extra = {};
      try {
        extra = JSON.parse(g('s_extra') || '{}');
      } catch {}
      return { ...extra, title: g('s_title'), url: g('s_url'), kind: g('s_kind'), credibility: g('s_cred'), credibility_note: g('s_note') };
    });
    return data;
  };

  async function save() {
    const res = await post('luu', collect());
    if (!res.ok) {
      toast('<strong>Chưa lưu được</strong>' + list(res.errors), 'bad', 0);
      return null;
    }
    const problems = (res.issues || []).filter((i) => i.level === 'error');
    toast(
      '<strong>Đã lưu.</strong>' +
        (problems.length ? ' Còn lỗi cần sửa trước khi đăng:' + list(problems.map((i) => i.message)) : '') +
        (res.page ? ` <a href="${esc(res.page)}">Xem trang →</a>` : ''),
      problems.length ? 'warn' : 'ok',
      problems.length ? 0 : 4000,
    );
    return res.path;
  }

  editor.addEventListener('submit', async (e) => {
    e.preventDefault();
    await save();
  });
})();
