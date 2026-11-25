// Tìm kiếm phía trình duyệt: tải search-index.json, so khớp không dấu, tô sáng, lọc theo loại.
(() => {
  const root = document.getElementById('search');
  if (!root) return;
  const input = document.getElementById('q');
  const statusEl = document.getElementById('status');
  const results = document.getElementById('results');
  const chips = [...root.querySelectorAll('[data-filter]')];
  const TYPE_LABELS = JSON.parse(root.dataset.types);
  const STATUS_LABELS = JSON.parse(root.dataset.statuses);

  const norm = (s) => s.normalize('NFD').replace(/[̀-ͯ]/g, '').replace(/đ/g, 'd').replace(/Đ/g, 'D').toLowerCase();
  const esc = (s) => String(s).replace(/[&<>"']/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' })[c]);
  const fmt = (iso) => iso.split('-').reverse().join('/');

  /** Tô sáng từ khoá trên chuỗi gốc (có dấu): so khớp trên bản đã bỏ dấu, ánh xạ vị trí từng ký tự. */
  function highlight(original, terms) {
    const chars = [...original];
    const normChars = chars.map((c) => norm(c));
    const flat = normChars.join('');
    const marks = new Array(chars.length).fill(false);
    const offsets = [];
    let pos = 0;
    for (const nc of normChars) {
      offsets.push(pos);
      pos += nc.length;
    }
    for (const term of terms) {
      let from = 0;
      while (term && (from = flat.indexOf(term, from)) !== -1) {
        const end = from + term.length;
        chars.forEach((_, i) => {
          if (offsets[i] >= from && offsets[i] < end) marks[i] = true;
        });
        from = end;
      }
    }
    let out = '';
    let open = false;
    chars.forEach((c, i) => {
      if (marks[i] && !open) { out += '<mark>'; open = true; }
      if (!marks[i] && open) { out += '</mark>'; open = false; }
      out += esc(c);
    });
    return out + (open ? '</mark>' : '');
  }

  function snippet(text, terms) {
    const n = norm(text);
    const idx = Math.min(...terms.map((t) => n.indexOf(t)).filter((i) => i >= 0));
    if (!isFinite(idx)) return '';
    const start = Math.max(0, idx - 60);
    return (start > 0 ? '…' : '') + text.slice(start, start + 180) + '…';
  }

  let index = [];
  let typeFilter = new URLSearchParams(location.search).get('type') || '';

  function render() {
    const raw = input.value.trim();
    const terms = norm(raw).split(/\s+/).filter(Boolean);
    const list = index.filter((e) => !typeFilter || e.type === typeFilter);
    let scored = list.map((e) => ({ e, score: 0 }));
    if (terms.length) {
      scored = list
        .map((e) => {
          let score = 0;
          for (const t of terms) {
            const s = (e._title.includes(t) ? 10 : 0) + (e._meta.includes(t) ? 4 : 0) + (e._text.includes(t) ? 1 : 0);
            if (!s) return null;
            score += s;
          }
          return { e, score };
        })
        .filter(Boolean)
        .sort((a, b) => b.score - a.score);
    }
    statusEl.textContent = terms.length
      ? `${scored.length} kết quả cho “${raw}”`
      : `${scored.length} kết quả${typeFilter ? ` loại ${TYPE_LABELS[typeFilter]}` : ''}`;
    results.innerHTML = scored
      .slice(0, 100)
      .map(({ e }) => {
        const snip = terms.length && !norm(e.title + e.summary).includes(terms[0]) ? snippet(e.text, terms) : '';
        return `<li class="note-item" data-type="${esc(e.type)}">
          <div class="meta-row">
            <span class="badge badge-type">${esc(TYPE_LABELS[e.type] || e.type)}</span>
            ${e.status ? `<span class="badge badge-status-${esc(e.status)}">${esc(STATUS_LABELS[e.status] || e.status)}</span>` : ''}
            <span>Cập nhật ${fmt(e.updated)}</span>
          </div>
          <h3><a href="${esc(e.url)}">${highlight(e.title, terms)}</a></h3>
          <p>${highlight(e.summary, terms)}</p>
          ${snip ? `<p style="margin-top:.4rem;font-size:.88rem">${highlight(snip, terms)}</p>` : ''}
          ${e.tags.length ? `<div class="meta-row" style="margin-top:.5rem">${e.tags.map((t) => `<span class="tag">#${esc(t)}</span>`).join('')}</div>` : ''}
        </li>`;
      })
      .join('');
    const params = new URLSearchParams();
    if (raw) params.set('q', raw);
    if (typeFilter) params.set('type', typeFilter);
    history.replaceState(null, '', params.toString() ? `?${params}` : location.pathname);
  }

  const syncChips = () => chips.forEach((c) => c.setAttribute('aria-pressed', String(c.dataset.filter === typeFilter)));
  chips.forEach((chip) =>
    chip.addEventListener('click', () => {
      typeFilter = chip.dataset.filter || '';
      syncChips();
      render();
    }),
  );
  input.addEventListener('input', render);
  input.value = new URLSearchParams(location.search).get('q') || '';
  syncChips();

  fetch(root.dataset.index)
    .then((r) => r.json())
    .then((data) => {
      index = data.map((e) => ({
        ...e,
        _title: norm([e.title, ...e.aliases].join(' ')),
        _meta: norm([e.summary, ...e.tags].join(' ')),
        _text: norm(e.text),
      }));
      render();
    })
    .catch(() => (statusEl.textContent = 'Không tải được chỉ mục tìm kiếm.'));
})();
