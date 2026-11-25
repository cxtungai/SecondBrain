// Đồ thị tri thức: mỗi chấm là một ghi chú, mỗi đường là một [[liên kết]]. SVG thuần, không thư viện.
(() => {
  const SVGNS = 'http://www.w3.org/2000/svg';

  function initGraph(root) {
    const data = JSON.parse(root.dataset.graph);
    const svg = root.querySelector('svg');
    const W = root.clientWidth || 300;
    const H = root.clientHeight || 300;
    const byId = new Map();
    const nodes = data.nodes.map((n, i) => {
      const angle = (i / data.nodes.length) * Math.PI * 2;
      const node = { ...n, x: W / 2 + Math.cos(angle) * W * 0.3, y: H / 2 + Math.sin(angle) * H * 0.3, vx: 0, vy: 0, deg: 0 };
      byId.set(n.id, node);
      return node;
    });
    const edges = data.edges.map(([a, b]) => [byId.get(a), byId.get(b)]).filter(([a, b]) => a && b);
    edges.forEach(([a, b]) => { a.deg++; b.deg++; });
    const focus = data.focus ? byId.get(data.focus) : undefined;
    if (focus) { focus.x = W / 2; focus.y = H / 2; }

    // Mô phỏng lực đơn giản: đẩy nhau, lò xo theo liên kết, kéo về tâm.
    const k = Math.sqrt((W * H) / Math.max(nodes.length, 1)) * 0.75;
    const ticks = nodes.length > 200 ? 150 : 300;
    for (let t = 0; t < ticks; t++) {
      const alpha = 1 - t / ticks;
      for (let i = 0; i < nodes.length; i++) {
        for (let j = i + 1; j < nodes.length; j++) {
          const a = nodes[i], b = nodes[j];
          let dx = a.x - b.x, dy = a.y - b.y;
          let d2 = dx * dx + dy * dy;
          if (d2 < 0.01) { dx = Math.random() - 0.5; dy = Math.random() - 0.5; d2 = 0.5; }
          const f = (k * k) / d2;
          a.vx += dx * f * 0.05; a.vy += dy * f * 0.05;
          b.vx -= dx * f * 0.05; b.vy -= dy * f * 0.05;
        }
      }
      for (const [a, b] of edges) {
        const dx = b.x - a.x, dy = b.y - a.y;
        const d = Math.sqrt(dx * dx + dy * dy) || 1;
        const f = (d - k) * 0.04;
        a.vx += (dx / d) * f; a.vy += (dy / d) * f;
        b.vx -= (dx / d) * f; b.vy -= (dy / d) * f;
      }
      for (const n of nodes) {
        n.vx += (W / 2 - n.x) * 0.01; n.vy += (H / 2 - n.y) * 0.01;
        if (n === focus) { n.vx = n.vy = 0; continue; }
        const speed = Math.sqrt(n.vx * n.vx + n.vy * n.vy);
        const max = 20 * alpha + 0.5;
        if (speed > max) { n.vx = (n.vx / speed) * max; n.vy = (n.vy / speed) * max; }
        n.x += n.vx; n.y += n.vy;
        n.vx *= 0.6; n.vy *= 0.6;
      }
    }

    const view = document.createElementNS(SVGNS, 'g');
    svg.appendChild(view);
    const lines = edges.map(([a, b]) => {
      const l = document.createElementNS(SVGNS, 'line');
      view.appendChild(l);
      return { l, a, b };
    });
    const showAllLabels = nodes.length <= 40;
    for (const n of nodes) {
      const g = document.createElementNS(SVGNS, 'g');
      g.setAttribute('class', `node${n.inbox ? ' inbox' : ''}${n === focus ? ' focus' : ''}`);
      g.setAttribute('data-type', n.type);
      const r = Math.min(4 + Math.sqrt(n.deg) * 2.2, 14) + (n === focus ? 2 : 0);
      const c = document.createElementNS(SVGNS, 'circle');
      c.setAttribute('r', String(r));
      const title = document.createElementNS(SVGNS, 'title');
      title.textContent = n.title + (n.inbox ? ' (chờ duyệt)' : '');
      c.appendChild(title);
      g.appendChild(c);
      if (showAllLabels || n.deg >= 3 || n === focus) {
        const text = document.createElementNS(SVGNS, 'text');
        text.setAttribute('x', String(r + 4));
        text.setAttribute('y', '4');
        text.textContent = n.title.length > 38 ? n.title.slice(0, 36) + '…' : n.title;
        g.appendChild(text);
      }
      n.el = g;
      view.appendChild(g);
    }
    const draw = () => {
      for (const { l, a, b } of lines) {
        l.setAttribute('x1', a.x); l.setAttribute('y1', a.y);
        l.setAttribute('x2', b.x); l.setAttribute('y2', b.y);
      }
      for (const n of nodes) n.el.setAttribute('transform', `translate(${n.x},${n.y})`);
    };
    draw();

    const xs = nodes.map((n) => n.x), ys = nodes.map((n) => n.y);
    const pad = 40;
    let vb = nodes.length
      ? { x: Math.min(...xs) - pad, y: Math.min(...ys) - pad, w: Math.max(...xs) - Math.min(...xs) + pad * 2 + 120, h: Math.max(...ys) - Math.min(...ys) + pad * 2 }
      : { x: 0, y: 0, w: W, h: H };
    const ratio = W / H;
    if (vb.w / vb.h < ratio) { const w = vb.h * ratio; vb.x -= (w - vb.w) / 2; vb.w = w; } else { const h = vb.w / ratio; vb.y -= (h - vb.h) / 2; vb.h = h; }
    const applyVb = () => svg.setAttribute('viewBox', `${vb.x} ${vb.y} ${vb.w} ${vb.h}`);
    applyVb();

    const neighbors = new Map();
    nodes.forEach((n) => neighbors.set(n, new Set([n])));
    edges.forEach(([a, b]) => { neighbors.get(a).add(b); neighbors.get(b).add(a); });
    const highlight = (n) => {
      root.classList.toggle('dim', !!n);
      nodes.forEach((m) => m.el.classList.toggle('hl', !!n && neighbors.get(n).has(m)));
      lines.forEach(({ l, a, b }) => l.classList.toggle('hl', !!n && (a === n || b === n)));
    };
    const toSvg = (e) => {
      const rect = svg.getBoundingClientRect();
      return { x: vb.x + ((e.clientX - rect.left) / rect.width) * vb.w, y: vb.y + ((e.clientY - rect.top) / rect.height) * vb.h };
    };
    let drag = null;
    nodes.forEach((n) => {
      n.el.addEventListener('pointerenter', () => !drag && highlight(n));
      n.el.addEventListener('pointerleave', () => !drag && highlight(null));
      n.el.addEventListener('pointerdown', (e) => {
        e.stopPropagation();
        svg.setPointerCapture(e.pointerId);
        drag = { node: n, start: toSvg(e), moved: false, vb0: { ...vb } };
      });
    });
    svg.addEventListener('pointerdown', (e) => {
      svg.setPointerCapture(e.pointerId);
      drag = { start: { x: e.clientX, y: e.clientY }, moved: false, vb0: { ...vb } };
    });
    svg.addEventListener('pointermove', (e) => {
      if (!drag) return;
      if (drag.node) {
        const p = toSvg(e);
        if (Math.abs(p.x - drag.start.x) + Math.abs(p.y - drag.start.y) > 3) drag.moved = true;
        drag.node.x = p.x; drag.node.y = p.y;
        draw();
      } else {
        const rect = svg.getBoundingClientRect();
        const dx = ((e.clientX - drag.start.x) / rect.width) * drag.vb0.w;
        const dy = ((e.clientY - drag.start.y) / rect.height) * drag.vb0.h;
        if (Math.abs(dx) + Math.abs(dy) > 2) drag.moved = true;
        vb = { ...drag.vb0, x: drag.vb0.x - dx, y: drag.vb0.y - dy };
        applyVb();
      }
    });
    svg.addEventListener('pointerup', () => {
      if (drag && drag.node && !drag.moved) location.href = drag.node.url;
      drag = null;
    });
    svg.addEventListener('wheel', (e) => {
      e.preventDefault();
      const rect = svg.getBoundingClientRect();
      const mx = vb.x + ((e.clientX - rect.left) / rect.width) * vb.w;
      const my = vb.y + ((e.clientY - rect.top) / rect.height) * vb.h;
      const s = e.deltaY > 0 ? 1.12 : 1 / 1.12;
      vb = { x: mx - (mx - vb.x) * s, y: my - (my - vb.y) * s, w: vb.w * s, h: vb.h * s };
      applyVb();
    }, { passive: false });
  }

  document.querySelectorAll('.graph[data-graph]').forEach(initGraph);
})();
