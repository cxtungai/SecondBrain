#!/usr/bin/env bash
# Duyệt / bỏ bản nháp theo tiêu đề issue — chạy trong .github/workflows/duyet.yml.
#
# Tiêu đề hợp lệ (do nút ở Khu duyệt bài /duyet/ điền sẵn):
#   "Duyệt: knowledge/_inbox/<slug>.md"   "Duyệt: articles/<slug>.md"
#   "Bỏ: knowledge/_inbox/<slug>.md"      "Bỏ: articles/<slug>.md"
#   "Duyệt tất cả bản nháp"
#
# Vào:  TITLE (tiêu đề issue), SB (chương trình secondbrain), PAGES_URL (địa chỉ web, có / cuối),
#       OUT_DIR (nơi ghi comment.md + commit-msg), GITHUB_OUTPUT (tuỳ chọn).
# Ra:   OUT_DIR/comment.md — trả lời issue; OUT_DIR/commit-msg; "changed=true|false" vào GITHUB_OUTPUT.
# Chạy thử trên máy: TITLE="Duyệt: knowledge/_inbox/x.md" SB=./target/release/secondbrain OUT_DIR=/tmp/x bash .github/scripts/duyet.sh
set -uo pipefail

SB="${SB:-./target/release/secondbrain}"
OUT="${OUT_DIR:-${RUNNER_TEMP:-/tmp}}"
WEB="${PAGES_URL:-}"
mkdir -p "$OUT"
COMMENT="$OUT/comment.md"
: >"$COMMENT"
changed=false
published=false
done_lines=()
fail_lines=()
commit_parts=()

say() { printf '%s\n' "$*" >>"$COMMENT"; }
finish() {
  [[ -n "${GITHUB_OUTPUT:-}" ]] && echo "changed=$changed" >>"$GITHUB_OUTPUT"
  say ""
  say "---"
  say "_Tự động bởi workflow “Duyệt bài trên web”._"
  exit 0
}

# Đường dẫn bản nháp hợp lệ: chỉ trong knowledge/_inbox/ hoặc articles/, tên file an toàn, file có thật.
is_inbox() { [[ "$1" =~ ^knowledge/_inbox/[a-z0-9][a-z0-9-]*\.md$ ]]; }
is_article() { [[ "$1" =~ ^articles/([a-z0-9][a-z0-9-]*/)*[a-z0-9][a-z0-9-]*\.md$ && "$1" != */readme.md ]]; }
article_is_draft() { ! grep -q -E '^status:[[:space:]]*"?published' "$1"; }
valid_draft() {
  local p="$1"
  [[ -f "$p" ]] || return 1
  is_inbox "$p" && return 0
  is_article "$p" && article_is_draft "$p" && return 0
  return 1
}

title_of() { sed -n 's/^title:[[:space:]]*//p' "$1" | head -1 | sed 's/^"\(.*\)"$/\1/; s/^'"'"'\(.*\)'"'"'$/\1/'; }

publish_one() {
  local p="$1" name slug log url
  name="$(title_of "$p")"
  slug="$(basename "$p" .md)"
  log="$(mktemp)"
  if "$SB" publish "$p" >"$log" 2>&1; then
    changed=true
    published=true
    if is_article "$p"; then url="${WEB}bai-viet/${slug}/"; else url="${WEB}notes/${slug}/"; fi
    done_lines+=("- ✅ Đã đăng **${name:-$slug}** → ${url}")
    commit_parts+=("${name:-$slug}")
  else
    fail_lines+=("- ❌ Chưa đăng được **${name:-$slug}** (\`$p\`):")
    while IFS= read -r line; do [[ -n "$line" ]] && fail_lines+=("  - ${line}"); done < <(sed 's/\x1b\[[0-9;]*m//g' "$log" | head -15)
  fi
  rm -f "$log"
}

title="$(printf '%s' "${TITLE:-}" | tr -d '\r' | sed 's/^[[:space:]]*//; s/[[:space:]]*$//')"

case "$title" in
  "Duyệt tất cả"*)
    files=()
    for f in knowledge/_inbox/*.md; do [[ -f "$f" ]] && is_inbox "$f" && files+=("$f"); done
    for f in $(find articles -name '*.md' 2>/dev/null | sort); do is_article "$f" && article_is_draft "$f" && files+=("$f"); done
    if [[ ${#files[@]} -eq 0 ]]; then
      say "Không có bản nháp nào đang chờ duyệt — không có gì để đăng."
      finish
    fi
    for f in "${files[@]}"; do publish_one "$f"; done
    ;;
  "Duyệt:"*)
    p="$(printf '%s' "${title#Duyệt:}" | sed 's/^[[:space:]]*//')"
    if ! valid_draft "$p"; then
      say "Không tìm thấy bản nháp \`$p\` (có thể đã được đăng hoặc bỏ trước đó). Không làm gì cả."
      finish
    fi
    publish_one "$p"
    ;;
  "Bỏ:"*)
    p="$(printf '%s' "${title#Bỏ:}" | sed 's/^[[:space:]]*//')"
    if ! valid_draft "$p"; then
      say "Không tìm thấy bản nháp \`$p\` (có thể đã được đăng hoặc bỏ trước đó). Không làm gì cả."
      finish
    fi
    name="$(title_of "$p")"
    git rm -q -- "$p"
    changed=true
    done_lines+=("- 🗑 Đã bỏ bản nháp **${name:-$p}**. Cần lấy lại thì nhờ Claude: “khôi phục bản nháp \`$p\` đã bỏ”.")
    commit_parts+=("bỏ ${name:-$p}")
    ;;
  *)
    say "Tiêu đề không đúng mẫu nên mình không làm gì. Hãy dùng các nút ở **Khu duyệt bài** trên web${WEB:+ (${WEB}duyet/)}."
    finish
    ;;
esac

# Đăng xong mà kho còn lỗi (vd. liên kết hỏng) thì không đưa lên — trả lại như cũ.
if [[ "$changed" == true ]]; then
  check_log="$(mktemp)"
  if ! "$SB" check >"$check_log" 2>&1; then
    git checkout -q -- knowledge articles 2>/dev/null
    git clean -q -fd -- knowledge articles 2>/dev/null
    git reset -q 2>/dev/null
    changed=false
    say "❌ Sau khi đăng, kiểm tra kho báo lỗi nên mình **chưa đưa gì lên web**. Nhờ Claude sửa giúp: “sửa lỗi kiểm tra kho rồi duyệt lại”."
    say ""
    say '```'
    sed 's/\x1b\[[0-9;]*m//g' "$check_log" | tail -20 >>"$COMMENT"
    say '```'
    rm -f "$check_log"
    finish
  fi
  rm -f "$check_log"
fi

for l in "${done_lines[@]+"${done_lines[@]}"}"; do say "$l"; done
for l in "${fail_lines[@]+"${fail_lines[@]}"}"; do say "$l"; done
if [[ "$published" == true ]]; then
  say ""
  say "Web sẽ cập nhật sau khoảng 2–3 phút. Gợi ý: nhờ Claude “kết nối các bài vừa đăng” để thêm liên kết hai chiều với tri thức sẵn có."
fi
if [[ "$changed" == true ]]; then
  msg="Duyệt trên web:"
  sep=" "
  for part in "${commit_parts[@]}"; do msg+="$sep$part"; sep="; "; done
  printf '%s\n' "$msg" | cut -c1-200 >"$OUT/commit-msg"
fi
if [[ ${#fail_lines[@]} -gt 0 ]]; then
  say ""
  say "Bài chưa đăng được: nhờ Claude sửa theo lỗi ở trên rồi bấm duyệt lại."
fi
finish
