(() => {
  if (window !== window.top) return;
  const boot = () => {
    if (document.documentElement.hasAttribute('data-crawlingo-launcher')) return;
    if (document.getElementById('crawlingo-selector-studio')) return;
    const host = document.createElement('div');
    host.id = 'crawlingo-selector-studio';
    host.style.cssText = 'all:initial;position:fixed;inset:0;z-index:2147483647;pointer-events:none';
    document.documentElement.append(host);
    const root = host.attachShadow({ mode: 'open' });
    root.innerHTML = `
      <style>
      :host{color-scheme:dark}*{box-sizing:border-box}button,input,select{font:inherit}
      .panel{pointer-events:auto;position:fixed;right:20px;top:20px;width:354px;max-width:calc(100vw - 24px);max-height:calc(100dvh - 40px);overflow:auto;border:1px solid #33484b;border-radius:18px;background:#101c22;color:#ecf7f5;box-shadow:0 16px 60px #0007;font:13px/1.5 system-ui,sans-serif}
      header{display:flex;align-items:center;gap:10px;padding:17px;border-bottom:1px solid #2b3d44;background:#16262c;position:sticky;top:0;z-index:2}
      header strong{font-size:15px;display:block}header small{font-size:10px;color:#93b0b8;letter-spacing:.12em;text-transform:uppercase}header svg{color:#5ee8c0;width:30px;height:30px}header button{margin-left:auto}
      .body{padding:16px}.mode{display:flex;gap:8px;margin-bottom:12px}button{cursor:pointer;border:1px solid #3d565e;border-radius:8px;color:#dbeeea;background:#20333b;padding:8px 11px}button:hover{background:#304951}button:focus-visible,input:focus-visible,select:focus-visible{outline:2px solid #5ee8c0;outline-offset:2px}button[aria-pressed=true],.primary{background:#5ee8c0;color:#09271e;border-color:#5ee8c0;font-weight:700}button:disabled{opacity:.4;cursor:not-allowed}
      .mode button{flex:1}.hint{color:#a9bec4;font-size:12px;margin:10px 0}.label{display:block;font-size:11px;color:#b3c9cf;margin:14px 0 5px}.input{width:100%;border:1px solid #3d565e;background:#0b151a;color:#edf8f6;border-radius:7px;padding:9px;font:12px/1.5 ui-monospace,monospace}.inline{display:flex;gap:8px;margin-top:8px}.inline>*{flex:1;min-width:0}.small{padding:4px 8px;font-size:11px}.row-note{color:#5ee8c0;font:11px/1.5 ui-monospace,monospace;overflow-wrap:anywhere}.preview{border:1px solid #2b424b;border-radius:8px;padding:10px;background:#0b151a;max-height:120px;overflow:auto;white-space:pre-wrap;overflow-wrap:anywhere;font:11px/1.6 ui-monospace,monospace;margin:10px 0}.status{font-size:11px;color:#a9c8cf;min-height:18px;overflow-wrap:anywhere}.fields{list-style:none;padding:0;margin:14px 0}.fields li{display:flex;align-items:center;gap:8px;border-top:1px solid #2b424b;padding:9px 0}.fields li span{flex:1;min-width:0;overflow-wrap:anywhere}.fields code{display:block;font-size:10px;color:#9cb5bd}.notice{font-size:10px;line-height:1.6;color:#93abb4;border-top:1px solid #2b424b;padding-top:12px}.required{display:flex;align-items:center;gap:7px;font-size:12px;margin:10px 0}.hidden{display:none!important}
      .outline{position:fixed;pointer-events:none;border:2px solid #5ee8c0;box-shadow:0 0 0 1px #09271e88;border-radius:4px;will-change:transform;width:0;height:0}.match{position:fixed;pointer-events:none;border:1px dashed #b88721;border-radius:3px}.spider{position:fixed;width:144px;height:144px;color:#142f2b;transition:transform 130ms cubic-bezier(.2,.8,.2,1);will-change:transform;pointer-events:none;filter:drop-shadow(0 0 1px #fff) drop-shadow(0 3px 3px #0004)}.spider svg{width:100%;height:100%;overflow:visible}.spider .leg{transform-origin:16px 18px;animation:scuttle 600ms ease-in-out infinite alternate}.spider .leg.even{animation-delay:-600ms}.spider .body-shape{transform-origin:16px 18px;animation:breathe 1200ms ease-in-out infinite alternate}.spider .silk{stroke:#718b88;stroke-width:.45;opacity:0;fill:none}.spider.catching .body-shape{animation:grab 560ms ease-out}.spider.catching .leg{animation:clasp 560ms ease-out}.spider.catching .silk{animation:web-catch 650ms ease-out}.outline.catching{animation:catch-glow 650ms ease-out}.badge{position:fixed;background:#102d26;color:#caffee;padding:3px 7px;border-radius:4px;font:11px ui-monospace,monospace;pointer-events:none;max-width:260px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
      @keyframes scuttle{from{transform:rotate(-6deg)}to{transform:rotate(6deg)}}@keyframes breathe{from{transform:translateY(-.4px)}to{transform:translateY(.4px)}}@keyframes grab{0%{transform:translateY(-3px) scale(.95)}45%{transform:translateY(3px) scale(1.12)}100%{transform:translateY(0) scale(1)}}@keyframes clasp{0%{transform:scale(.9,1.1)}45%{transform:scale(1.16,.8)}100%{transform:scale(1)}}@keyframes web-catch{0%{opacity:0;stroke-dasharray:0 80}35%{opacity:.9;stroke-dasharray:80 0}100%{opacity:0;stroke-dasharray:80 0}}@keyframes catch-glow{35%{box-shadow:0 0 0 5px #5ee8c044}100%{box-shadow:0 0 0 1px #09271e88}}
      @media(prefers-reduced-motion:reduce){.spider{transition:none}.spider .leg,.spider .body-shape,.spider.catching .leg,.spider.catching .body-shape,.spider.catching .silk,.outline.catching{animation:none}}@media(max-width:600px){.panel{right:12px;top:12px;width:300px;max-height:60dvh}.spider{width:112px;height:112px}}
      .panel{top:auto;bottom:20px;left:50%;right:auto;transform:translateX(-50%);width:560px;max-height:70dvh;border-radius:16px;box-shadow:0 12px 42px #0004;background:#102027f5;backdrop-filter:blur(16px)}header{padding:11px 16px}header strong{font-size:14px}header svg{width:25px;height:25px}header small{font-size:9px}.body{padding:12px 16px}.hint{margin:6px 0;font-size:11px}.mode{margin-bottom:8px}.mode #auto{flex:3}.mode #stop-auto{flex:1}.preview{max-height:88px}details{margin-top:10px;border-top:1px solid #30474d}summary{cursor:pointer;padding:10px 0 2px;color:#b4c9cd;font-size:12px}details[open]>summary{padding-bottom:12px}.panel:has(details[open]){left:auto;right:20px;transform:none;width:354px;max-height:calc(100dvh - 40px)}#auto-progress:empty{display:none}#auto-results{margin:8px 0}#auto-results:empty{display:none}@media(max-width:600px){.panel,.panel:has(details[open]){left:12px;right:12px;bottom:12px;top:auto;transform:none;width:auto;max-height:65dvh}}
      .reach{position:fixed;inset:0;width:100vw;height:100vh;overflow:visible;pointer-events:none;filter:drop-shadow(0 0 1px #fff) drop-shadow(0 2px 2px #0003)}.reach path{fill:none;stroke:#142f2b;stroke-width:4;stroke-linecap:round;stroke-linejoin:round}.spider .leg.active-reach{visibility:hidden}
      .spider{left:50%;top:50%;transform:translate(-50%,-50%);transition:none}.spider .leg{animation-duration:1600ms}.spider .leg.even{animation-delay:-1600ms}.logo .silk{display:none}.phase{font-size:10px;font-weight:600;color:#a8c1c2;border:1px solid #365158;border-radius:20px;padding:3px 8px;margin-left:auto}.phase[data-phase=gripping]{color:#5ee8c0;border-color:#378d78}header button{margin-left:0}.studio-options{display:flex;align-items:center;justify-content:space-between;gap:12px;margin-top:8px;color:#a9bec4;font-size:11px}.studio-options select{background:#162b31;color:#dbeeea;border:1px solid #3d565e;border-radius:6px;padding:4px 8px}#collection-progress{width:100%;height:4px;accent-color:#5ee8c0;margin:8px 0 0}#results>summary{display:flex;align-items:center;justify-content:space-between}#record-count{font:11px ui-monospace,monospace;color:#5ee8c0}#results .preview{max-height:120px}.panel:has(#results[open]):not(:has(#manual[open])){left:50%;right:auto;transform:translateX(-50%);width:560px;max-height:70dvh}@media(max-width:600px){.panel:has(#results[open]):not(:has(#manual[open])){left:12px;right:12px;transform:none;width:auto;max-height:65dvh}}
      .panel{background:#111c27;border-color:#314153;border-radius:18px;box-shadow:0 20px 70px #0005;backdrop-filter:none}header{background:#15222e;border-color:#29394a;padding:14px 18px}header strong{letter-spacing:-.3px;font-size:15px}header small{color:#87a2b6;letter-spacing:.14em}.body{padding:16px 18px}button{border-color:#34485a;background:#1b2b3a;border-radius:9px;padding:9px 12px}button:hover{background:#293e50}button.primary,button[aria-pressed=true]{background:#76e2be;border-color:#76e2be;color:#09251c}.small{padding:5px 9px}#toggle-spider{color:#a8bdcc;background:transparent;border-color:#34485a;font-weight:400}#stop-auto:not(:disabled){color:#ffc2b7;border-color:#8a5149;background:#3a2428}.source{display:flex;align-items:center;gap:8px;color:#a1b4c4;font:11px ui-monospace,monospace;margin-bottom:14px;overflow:hidden}.source::before{content:'';width:6px;height:6px;border-radius:50%;background:#76e2be;flex-shrink:0}.source span{overflow:hidden;text-overflow:ellipsis;white-space:nowrap}.hint{color:#92a8ba;font-size:11px}.phase{background:#101b25;border-color:#34485a}.studio-options{padding:10px 0 2px}.studio-options select{background:#0d1721;border-color:#34485a}details{border-color:#2c3d4d}summary{padding:13px 0 3px;color:#bfceda}#results>summary{font-size:12px}#record-count{background:#213e37;border-radius:5px;padding:2px 7px}#collection-progress{appearance:none;border:0;border-radius:4px;overflow:hidden;background:#293b48}#collection-progress::-webkit-progress-bar{background:#293b48}#collection-progress::-webkit-progress-value{background:#76e2be}.input,.preview{background:#0c1620;border-color:#304456}.notice{border-color:#2c3d4d}
      .spider.catching .leg{animation:scuttle 1600ms ease-in-out infinite alternate}.spider.catching .leg.even{animation-delay:-1600ms}@media(prefers-reduced-motion:reduce){.spider.catching .leg{animation:none}}
      </style>
      <div class="outline hidden"></div><div class="spider hidden" aria-hidden="true"></div><div class="badge hidden"></div>
      <section class="panel" aria-label="Crawlingo selector inspector">
        <header><span class="logo"></span><div><small>Live collection</small><strong>Crawlingo Studio</strong></div><span id="phase" class="phase" data-phase="idle">Ready</span><button class="small" id="minimize" aria-label="Collapse inspector" aria-expanded="true">−</button></header>
        <div class="body">
          <div class="source"><span id="source-host"></span></div>
          <div class="mode"><button class="primary" id="auto">Watch spider collect</button><button id="stop-auto" disabled>Stop</button></div>
          <p class="hint">Auto-scroll · Catch up to 30 records · Save JSON · Review detected data</p>
          <div class="studio-options"><label for="pace">Animation <select id="pace"><option value="gentle">Gentle</option><option value="normal" selected>Normal</option><option value="quick">Quick</option></select></label><button id="toggle-spider" class="small" aria-pressed="true">Hide spider</button></div>
          <progress id="collection-progress" class="hidden" max="30" value="0" aria-label="Records collected"></progress>
          <div id="auto-progress" class="row-note" role="status" aria-live="polite"></div>
          <details id="results" class="hidden"><summary>Collected records <span id="record-count">0</span></summary>
          <div id="auto-results" class="preview hidden"></div>
          <button id="save-preview" class="small hidden">Save collected JSON</button>
          </details>
          <details id="manual"><summary>Choose selectors & configure a dataset job</summary>
          <div class="mode"><button id="pick" aria-pressed="true">◎ Pick element</button><button id="browse" aria-pressed="false">↗ Browse page</button></div>
          <p class="hint">The spider stays in the screen center. Hover to highlight; click to reach and grip. Escape switches modes.</p>
          <div class="inline"><button id="parent" class="small" disabled>Select parent ↑</button><button id="rows" class="small" disabled>Use as rows</button></div>
          <label class="label" for="row">Row selector · optional</label><input class="input" id="row" placeholder="e.g. article.product" autocomplete="off" spellcheck="false">
          <div class="inline"><button class="small" id="apply-row">Apply rows</button><button class="small" id="clear-row">Clear rows</button></div>
          <div class="row-note" id="row-status">One record per page</div>
          <label class="label" for="selector">Field selector</label><input class="input" id="selector" placeholder="Pick an element on the page" autocomplete="off" spellcheck="false">
          <div class="inline"><button class="small" id="test">Preview matches</button><button class="small" id="copy-selector">Copy selector</button></div>
          <label class="label" for="name">Dataset field name</label><input class="input" id="name" placeholder="title" value="title" maxlength="64" autocomplete="off">
          <label class="label" for="extract">Extract</label><select class="input" id="extract"><option value="text">Text</option><option value="attr:href">Link · href</option><option value="attr:src">Image · src</option><option value="attr:title">Attribute · title</option><option value="custom">Other attribute…</option></select>
          <input class="input hidden" id="attribute" aria-label="Attribute name" placeholder="data-id" autocomplete="off">
          <label class="required"><input type="checkbox" id="required" checked>Required field</label>
          <div class="preview" id="preview">Pick an element to see its value.</div>
          <button class="primary" id="add">+ Add field</button>
          <ul class="fields" id="fields" aria-label="Dataset fields"></ul>
          <div class="inline"><button id="copy-job">Copy job JSON</button><button class="primary" id="export">Export files</button></div>
          <p class="status" id="status" role="status" aria-live="polite">Ready · top-level page only</p>
          <p class="notice">Rendered-DOM preview, not a Rust extraction test. The HTTP job may receive different HTML. Selectors support tags, classes, IDs, descendant spaces and positive :nth-child(N). Iframes and shadow-root content are not supported. Navigation starts a new selection.</p>
          </details>
        </div>
      </section>`;
    const svg = '<svg viewBox="0 0 32 32" fill="none" stroke="currentColor" stroke-width="1.25" stroke-linecap="round" stroke-linejoin="round"><g class="silk"><path d="M16 21 0 32M16 21 8 36M16 21 24 36M16 21 32 32M6 28Q16 31 26 28M3 30Q16 36 29 30"/></g><path class="leg" d="M12 13 7 9 6 3 3 1"/><path class="leg even" d="M11 16 5 13 2 7 0 6"/><path class="leg" d="M11 19 4 21 2 27 0 29"/><path class="leg even" d="M13 22 8 26 8 30 6 32"/><path class="leg even" d="M20 13 25 9 26 3 29 1"/><path class="leg" d="M21 16 27 13 30 7 32 6"/><path class="leg even" d="M21 19 28 21 30 27 32 29"/><path class="leg" d="M19 22 24 26 24 30 26 32"/><g class="body-shape"><ellipse cx="16" cy="20" rx="5" ry="7" fill="currentColor"/><ellipse cx="16" cy="11" rx="3.5" ry="4" fill="currentColor"/><path d="M14 7 13 5M18 7 19 5"/><path d="M14 18Q16 16 18 18M14 21Q16 19 18 21" stroke="#5ee8c0" stroke-width=".7"/><circle cx="14.8" cy="10" r=".65" fill="#b8ffe9" stroke="none"/><circle cx="17.2" cy="10" r=".65" fill="#b8ffe9" stroke="none"/></g></svg>';
    root.querySelector('.logo').innerHTML = svg;
    root.querySelector('.spider').innerHTML = svg;
    const $ = id => root.getElementById(id);
    $('source-host').textContent = location.host;
    const outline = root.querySelector('.outline');
    const spider = root.querySelector('.spider');
    const reach = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
    reach.setAttribute('class', 'reach hidden'); reach.setAttribute('aria-hidden', 'true');
    const reachLeg = document.createElementNS('http://www.w3.org/2000/svg', 'path');
    reach.append(reachLeg);
    const restingLegs = [...spider.querySelectorAll('.leg')];
    spider.before(reach);
    const badge = root.querySelector('.badge');
    let picking = true, selected = null, hovered = null, rowSelector = '', matches = [], fields = [];
    let point = null, lastUrl = location.href, frameId, catchTimer;
    let catchStarted = 0;
    let activeLeg = 0;
    let catchDuration = 1200, showSpider = true;
    const reducedMotion = matchMedia('(prefers-reduced-motion: reduce)');
    let collecting = false, runId = 0, collected = [];
    const boxes = [];
    const say = message => { $('status').textContent = message; };
    function phase(label) {
      if ($('phase').textContent !== label) { $('phase').textContent = label; $('phase').dataset.phase = label.toLowerCase(); }
    }
    const validName = value => /^[a-zA-Z_][\w-]*$/.test(value);
    function supported(selector) {
      const atom = /^(?:[a-zA-Z][\w-]*|\*)?(?:(?:[.#][a-zA-Z_][\w-]*)|(?::nth-child\([1-9][0-9]*\)))*$/;
      return !!selector.trim() && selector.length <= 2048 && selector.trim().split(/\s+/).every(part => atom.test(part));
    }
    function find(selector, scope = document) {
      if (!supported(selector)) throw new Error('Use tags, classes, IDs, spaces and positive :nth-child(N).');
      const found = [...scope.querySelectorAll(selector)].filter(el => el !== host && !host.contains(el));
      if (scope instanceof Element && scope.matches(selector)) found.unshift(scope);
      return found;
    }
    function simple(el) {
      return el.localName + [...el.classList].filter(validName).slice(0, 3).map(c => '.' + c).join('');
    }
    function selectorFor(el, scope = document, repeated = false) {
      if (el.getRootNode() !== document) throw new Error('Shadow-root elements are not supported.');
      const candidates = [simple(el)];
      if (el.id && validName(el.id)) candidates.push('#' + el.id);
      candidates.push(el.localName);
      for (const candidate of candidates) {
        const found = find(candidate, scope);
        if ((repeated && found.includes(el)) || (found.length === 1 && found[0] === el)) return candidate;
      }
      const parts = [];
      for (let cursor = el; cursor && cursor !== scope; cursor = cursor.parentElement) {
        const position = cursor.parentElement ? [...cursor.parentElement.children].indexOf(cursor) + 1 : 1;
        parts.unshift(cursor.localName + ':nth-child(' + position + ')');
        const candidate = parts.join(' ');
        const found = find(candidate, scope);
        if (found.length === 1 && found[0] === el) return candidate;
      }
      if (scope === el) return simple(el);
      throw new Error('Could not generate a selector; choose a different element.');
    }
    const extractType = () => $('extract').value === 'custom' ? 'attr:' + $('attribute').value.trim() : $('extract').value;
    function preview() {
      const selector = $('selector').value.trim();
      const scopes = rowSelector ? find(rowSelector) : [document];
      const type = extractType();
      if (type !== 'text' && !/^attr:[a-zA-Z_][\w:.-]*$/.test(type)) throw new Error('Enter an attribute name, such as data-id.');
      let total = 0;
      matches = [];
      const values = [];
      for (const scope of scopes) {
        const found = find(selector, scope);
        total += found.length;
        matches.push(...found.slice(0, Math.max(0, 40 - matches.length)));
        if (values.length < 5) {
          const value = type === 'text' ? found.map(el => el.textContent || '').join(' ').replace(/\s+/g, ' ').trim() : found[0]?.getAttribute(type.slice(5)) || '';
          values.push(value ? value.slice(0, 350) : '(missing)');
        }
      }
      $('preview').textContent = values.map((value, i) => (rowSelector ? 'Row ' + (i + 1) + ': ' : '') + value).join('\n\n') || 'No matching rows.';
      say(total + ' element matches across ' + scopes.length + ' record scope(s). Showing up to 5 values / 40 outlines.');
      return total;
    }
    function select(el) {
      if (!(el instanceof Element) || el === host) return;
      selected = el;
      hovered = null;
      $('parent').disabled = !el.parentElement || el.parentElement === document.documentElement;
      $('rows').disabled = false;
      const scope = rowSelector ? el.closest(rowSelector) : document;
      if (!scope) { $('selector').value = ''; matches = []; throw new Error('Choose a field inside one of the selected rows.'); }
      $('selector').value = selectorFor(el, scope);
      preview();
      beginCatch();
    }
    function legPoints(index, body) {
      const joints = [[12,13],[11,16],[11,19],[13,22],[20,13],[21,16],[21,19],[19,22]];
      const tips = [[3,1],[0,6],[0,29],[6,32],[29,1],[32,6],[32,29],[26,32]];
      const point = coordinates => ({ x: body.left + coordinates[0] * body.width / 32, y: body.top + coordinates[1] * body.height / 32 });
      return { joint: point(joints[index]), tip: point(tips[index]) };
    }
    function nearestEdge(point, rect) {
      const x = Math.max(rect.left, Math.min(rect.right, point.x));
      const y = Math.max(rect.top, Math.min(rect.bottom, point.y));
      // Inside the box, choose its closest boundary instead of grabbing empty space.
      if (point.x >= rect.left && point.x <= rect.right && point.y >= rect.top && point.y <= rect.bottom) {
        return [{ x: rect.left, y }, { x: rect.right, y }, { x, y: rect.top }, { x, y: rect.bottom }]
          .sort((a, b) => Math.hypot(a.x - point.x, a.y - point.y) - Math.hypot(b.x - point.x, b.y - point.y))[0];
      }
      return { x, y };
    }
    function beginCatch() {
      clearTimeout(catchTimer);
      if (selected?.isConnected) {
        const rect = selected.getBoundingClientRect(), body = spider.getBoundingClientRect();
        let distance = Infinity;
        restingLegs.forEach((leg, index) => {
          const { tip } = legPoints(index, body), edge = nearestEdge(tip, rect);
          const candidate = Math.hypot(tip.x - edge.x, tip.y - edge.y);
          if (candidate < distance) { distance = candidate; activeLeg = index; }
        });
      }
      catchDuration = { gentle: 1800, normal: 1200, quick: 800 }[$('pace').value];
      catchStarted = performance.now();
      for (const node of [spider, outline]) node.classList.remove('catching');
      // Restart the short capture animation even when selecting the same element.
      void spider.offsetWidth;
      for (const node of [spider, outline]) node.classList.add('catching');
      catchTimer = setTimeout(() => { for (const node of [spider, outline]) node.classList.remove('catching'); }, catchDuration);
    }
    function guard(fn) { return (...args) => { try { fn(...args); } catch (error) { matches = []; say(error.message); } }; }
    function mode(value) {
      if (collecting) stopCollection();
      picking = value; hovered = null;
      $('pick').setAttribute('aria-pressed', String(value));
      $('browse').setAttribute('aria-pressed', String(!value));
      say(value ? 'Pick mode · click an element to pin it.' : 'Browse mode · links and controls work normally.');
    }
    function applyRows(value) {
      const next = value.trim();
      if (next) find(next);
      if (fields.length && next !== rowSelector) throw new Error('Remove saved fields before changing their row scope.');
      rowSelector = next;
      $('row').value = next;
      $('row-status').textContent = next ? find(next).length + ' rows · ' + next : 'One record per page';
      if (selected?.isConnected) select(selected);
    }
    function renderFields() {
      $('fields').replaceChildren();
      fields.forEach((field, index) => {
        const li = document.createElement('li');
        const label = document.createElement('span');
        label.textContent = field.name + (field.required ? ' *' : '');
        const code = document.createElement('code'); code.textContent = field.selector + ' → ' + field.extract_type; label.append(code);
        const remove = document.createElement('button'); remove.className = 'small'; remove.textContent = '×'; remove.setAttribute('aria-label', 'Remove ' + field.name);
        remove.onclick = () => { fields.splice(index, 1); renderFields(); };
        li.append(label, remove); $('fields').append(li);
      });
    }
    function job() {
      if (!fields.length) throw new Error('Add a field first.');
      return { urls_file: 'urls.txt', output_dir: 'datasets/selected', ...(rowSelector ? { row_selector: rowSelector } : {}), fields, concurrency: 2, respect_robots: true, format: 'jsonl' };
    }
    async function copy(text) {
      try { await navigator.clipboard.writeText(text); say('Copied to clipboard.'); }
      catch { say('Clipboard unavailable on this page. Select the text manually or export files.'); }
    }
    $('pick').onclick = () => mode(true);
    $('toggle-spider').onclick = () => {
      showSpider = !showSpider;
      $('toggle-spider').textContent = showSpider ? 'Hide spider' : 'Show spider';
      $('toggle-spider').setAttribute('aria-pressed', String(showSpider));
    };
    $('manual').ontoggle = () => { if (!collecting) mode($('manual').open); };
    $('browse').onclick = () => mode(false);
    $('parent').onclick = guard(() => select(selected.parentElement));
    $('rows').onclick = guard(() => applyRows(selectorFor(selected, document, true)));
    $('apply-row').onclick = guard(() => applyRows($('row').value));
    $('clear-row').onclick = guard(() => applyRows(''));
    $('test').onclick = guard(preview);
    $('extract').onchange = guard(() => { $('attribute').classList.toggle('hidden', $('extract').value !== 'custom'); if ($('selector').value) preview(); });
    $('copy-selector').onclick = guard(() => { if (!supported($('selector').value)) throw new Error('Pick or enter a supported selector first.'); void copy($('selector').value); });
    $('copy-job').onclick = guard(() => { void copy(JSON.stringify(job(), null, 2)); });
    $('add').onclick = guard(() => {
      const name = $('name').value.trim();
      if (!/^[a-zA-Z][a-zA-Z0-9_]{0,63}$/.test(name)) throw new Error('Use a field name starting with a letter, then letters, numbers or underscores.');
      if (fields.some(field => field.name === name)) throw new Error('This field name already exists.');
      if (fields.length >= 50) throw new Error('This inspector supports up to 50 fields.');
      if (!preview()) throw new Error('The selector has no matches.');
      fields.push({ name, selector: $('selector').value.trim(), extract_type: extractType(), required: $('required').checked });
      renderFields(); say('Added ' + name + '. Pick the next field or export.');
    });
    $('export').onclick = async () => {
      try {
        const config = job();
        if (typeof window.__crawlingoExport !== 'function') throw new Error('Start this page through the Selector Studio CLI to export files.');
        $('export').disabled = true;
        say(await window.__crawlingoExport(config));
      } catch (error) { say(error.message); }
      finally { $('export').disabled = false; }
    };
    $('minimize').onclick = () => {
      const collapsed = root.querySelector('.body').classList.toggle('hidden');
      $('minimize').textContent = collapsed ? '+' : '−';
      $('minimize').setAttribute('aria-expanded', String(!collapsed));
      $('minimize').setAttribute('aria-label', collapsed ? 'Expand inspector' : 'Collapse inspector');
    };
    const pause = ms => new Promise(resolve => setTimeout(resolve, ms));
    function stopCollection() {
      collecting = false; runId++;
      for (const node of [spider, outline]) node.classList.remove('catching');
      $('auto').disabled = false; $('stop-auto').disabled = true;
      $('auto-progress').textContent = 'Stopped. ' + collected.length + ' records kept for review.';
      $('save-preview').classList.toggle('hidden', !collected.length);
    }
    function detectRecords() {
      if (rowSelector) return find(rowSelector).filter(el => el.getClientRects().length).slice(0, 30);
      // Prefer repeated siblings in content regions over navigation or arbitrary containers.
      const groups = new Map();
      for (const el of [...document.querySelectorAll('article, tr, li, div[class]')].slice(0, 5000)) {
        if (el.closest('nav,header,footer,aside,[role="navigation"],#crawlingo-selector-studio') || !el.getClientRects().length) continue;
        const text = (el.innerText || '').trim();
        if (text.length < 20 || text.length > 12000 || !el.querySelector('a,h2,h3,td')) continue;
        if (el.localName === 'div' && !el.querySelector('h2,h3')) continue;
        const parent = el.parentElement;
        if (!groups.has(parent)) groups.set(parent, new Map());
        const key = simple(el), siblings = groups.get(parent);
        if (!siblings.has(key)) siblings.set(key, []);
        siblings.get(key).push(el);
      }
      const candidates = [...groups.values()].flatMap(group => [...group.values()]).filter(group => group.length >= 3);
      candidates.sort((a, b) => {
        const score = group => Math.min(group.length, 30) * (group[0].closest('main,[role="main"]') ? 2 : 1) * (group[0].localName === 'div' ? 0.5 : 1);
        return score(b) - score(a);
      });
      return candidates[0]?.slice(0, 30) || [];
    }
    async function saveCollected() {
      if (!collected.length) throw new Error('No records collected yet.');
      if (typeof window.__crawlingoSavePreview !== 'function') throw new Error('Use the Selector Studio launcher to save data.');
      $('save-preview').disabled = true;
      try { $('auto-progress').textContent = await window.__crawlingoSavePreview(collected); }
      finally { $('save-preview').disabled = false; }
    }
    $('save-preview').onclick = () => { void saveCollected().catch(error => { $('auto-progress').textContent = error.message; }); };
    $('stop-auto').onclick = stopCollection;
    root.addEventListener('click', event => {
      if (collecting && event.target.closest('button') && !event.target.closest('#auto,#stop-auto,#save-preview,#minimize,#toggle-spider')) stopCollection();
    }, true);
    $('auto').onclick = async () => {
      if (collecting) return;
      mode(false);
      const id = ++runId, startUrl = location.href;
      collecting = true; collected = []; matches = [];
      $('collection-progress').value = 0; $('collection-progress').classList.remove('hidden');
      $('results').classList.add('hidden'); $('record-count').textContent = '0';
      $('auto').disabled = true; $('stop-auto').disabled = false;
      $('auto-results').textContent = ''; $('auto-results').classList.remove('hidden');
      $('save-preview').classList.add('hidden');
      try {
        const rows = detectRecords();
        if (!rows.length) throw new Error('No repeated records found. Select a record and use “Use as rows”, then try again.');
        $('collection-progress').max = rows.length;
        for (let index = 0; index < rows.length; index++) {
          if (!collecting || id !== runId || startUrl !== location.href) return;
          const row = rows[index];
          if (!row.isConnected) continue;
          selected = row; hovered = null;
          $('auto-progress').textContent = 'Reaching for record ' + (index + 1) + ' of ' + rows.length + '…';
          row.scrollIntoView({ behavior: reducedMotion.matches ? 'instant' : 'smooth', block: 'center' });
          await pause(800);
          if (!collecting || id !== runId || startUrl !== location.href) return;
          if (!row.isConnected) continue;
          beginCatch();
          await pause(catchDuration);
          if (!collecting || id !== runId || startUrl !== location.href) return;
          for (const node of [spider, outline]) node.classList.remove('catching');
          const link = row.querySelector('a[href]');
          const href = link ? new URL(link.getAttribute('href'), document.baseURI) : null;
          const record = {
            title: (row.querySelector('h1,h2,h3,h4')?.innerText || link?.innerText || '').trim().slice(0, 1000),
            text: (row.innerText || '').replace(/\s+/g, ' ').trim().slice(0, 12000),
            url: href && ['http:', 'https:'].includes(href.protocol) ? href.href.slice(0, 12000) : '',
          };
          collected.push(record); matches.push(row);
          $('results').classList.remove('hidden'); $('record-count').textContent = String(collected.length);
          $('collection-progress').value = collected.length;
          $('auto-results').textContent = collected.map((item, i) => (i + 1) + '. ' + (item.title || item.text.slice(0, 120))).join('\n\n');
          $('auto-progress').textContent = 'Caught ' + collected.length + ' / ' + rows.length + ' records.';
          $('save-preview').classList.remove('hidden');
        }
        await saveCollected();
      } catch (error) { $('auto-progress').textContent = error.message; }
      finally {
        if (id === runId) { collecting = false; $('auto').disabled = false; $('stop-auto').disabled = true; }
      }
    };
    document.addEventListener('pointermove', event => {
      point = { x: event.clientX, y: event.clientY };
      hovered = picking && !event.composedPath().includes(host) ? event.target : null;
    }, true);
    document.addEventListener('click', event => {
      if (!picking || event.composedPath().includes(host)) return;
      event.preventDefault(); event.stopImmediatePropagation();
      guard(select)(event.target);
    }, true);
    document.addEventListener('keydown', event => {
      if (event.key === 'Escape') { event.preventDefault(); mode(!picking); }
    }, true);
    document.addEventListener('scroll', () => {
      if (picking && point) {
        const el = document.elementFromPoint(point.x, point.y);
        hovered = el === host ? null : el;
      }
    }, { capture: true, passive: true });
    function box(el, node) {
      const rect = el.getBoundingClientRect();
      node.style.transform = 'translate(' + rect.left + 'px,' + rect.top + 'px)';
      node.style.width = rect.width + 'px'; node.style.height = rect.height + 'px';
      return rect;
    }
    function draw() {
      if (!host.isConnected) return;
      if (location.href !== lastUrl) {
        if (collecting) stopCollection();
        collected = []; $('auto-results').textContent = ''; $('save-preview').classList.add('hidden');
        $('results').classList.add('hidden'); $('record-count').textContent = '0'; $('collection-progress').classList.add('hidden');
        lastUrl = location.href; fields = []; rowSelector = ''; selected = null; hovered = null; matches = [];
        $('row').value = ''; $('selector').value = ''; $('row-status').textContent = 'One record per page'; renderFields(); say('URL changed. Start a new selection.');
      }
      const target = spider.classList.contains('catching') ? selected : (picking && hovered) || selected;
      const visible = target instanceof Element && target.isConnected && target.getRootNode() === document && target !== host;
      for (const el of [outline, badge]) el.classList.toggle('hidden', !visible);
      spider.classList.toggle('hidden', !showSpider);
      const elapsed = Math.min(1, (performance.now() - catchStarted) / catchDuration);
      const catching = spider.classList.contains('catching');
      phase(catching ? (elapsed < .4 ? 'Reaching' : elapsed < .65 ? 'Gripping' : 'Retracting') : collecting ? 'Scrolling' : collected.length ? 'Collected' : picking ? 'Picking' : 'Ready');
      let stretching = false;
      if (visible) {
        const rect = box(target, outline);
        const onScreen = rect.bottom > 0 && rect.right > 0 && rect.top < innerHeight && rect.left < innerWidth && rect.width > 0 && rect.height > 0;
        badge.classList.toggle('hidden', !onScreen);
        badge.style.transform = 'translate(' + Math.max(0, Math.min(innerWidth - 265, rect.left + 22)) + 'px,' + Math.max(0, Math.min(innerHeight - 22, rect.top - 24)) + 'px)';
        badge.textContent = simple(target);
        stretching = showSpider && onScreen && catching && !reducedMotion.matches;
        if (stretching) {
          // Keep seven legs at rest; only the nearest leg reaches to the moving target.
          const body = spider.getBoundingClientRect();
          {
            const index = activeLeg;
            const side = index < 4 ? -1 : 1, row = index % 4, t = row / 3;
            const extension = elapsed < .4 ? elapsed / .4 : elapsed < .65 ? 1 : Math.max(0, (1 - elapsed) / .35);
            const amount = extension * extension * (3 - 2 * extension);
            const { joint, tip } = legPoints(index, body), edge = nearestEdge(tip, rect);
            const sx = joint.x, sy = joint.y;
            const x = tip.x + (edge.x - tip.x) * amount;
            const y = tip.y + (edge.y - tip.y) * amount;
            const dx = x - sx, dy = y - sy, length = Math.max(1, Math.hypot(dx, dy));
            // Preserve the top-to-bottom leg order instead of crossing adjacent limbs.
            const kneeX = sx + dx * .48 + side * Math.min(40, length * .12);
            const kneeY = sy + dy * .48 + (t - .5) * Math.min(150, 36 + length * .4);
            const inwardX = (rect.left + rect.right) / 2 - edge.x;
            const inwardY = (rect.top + rect.bottom) / 2 - edge.y;
            const inwardLength = Math.max(1, Math.hypot(inwardX, inwardY));
            const hookX = x + inwardX / inwardLength * 8 * amount;
            const hookY = y + inwardY / inwardLength * 8 * amount;
            // Rounded joints, a long shin and a curled toe make the contact readable.
            reachLeg.setAttribute('d', `M${sx},${sy} L${kneeX - dx * .04},${kneeY - dy * .04} Q${kneeX},${kneeY} ${kneeX + dx * .04},${kneeY + dy * .04} L${x},${y} Q${hookX},${y} ${hookX},${hookY}`);
          }
        }
      }
      reach.classList.toggle('hidden', !stretching);
      spider.classList.toggle('reaching', stretching);
      restingLegs.forEach((leg, index) => leg.classList.toggle('active-reach', stretching && index === activeLeg));
      while (boxes.length < matches.length) { const node = document.createElement('div'); node.className = 'match'; root.prepend(node); boxes.push(node); }
      boxes.forEach((node, index) => { const el = matches[index]; node.classList.toggle('hidden', !el?.isConnected); if (el?.isConnected) box(el, node); });
      frameId = requestAnimationFrame(draw);
    }
    document.addEventListener('visibilitychange', () => { cancelAnimationFrame(frameId); if (!document.hidden) draw(); });
    mode(false);
    draw();
  };
  if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', boot, { once: true });
  else boot();
})();
