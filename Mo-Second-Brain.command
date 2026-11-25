#!/usr/bin/env bash
# Bấm đúp để mở Second Brain (macOS / Linux). Lần đầu sẽ tự tải chương trình (~5 MB).
cd "$(dirname "$0")"
REPO="cxtungai/SecondBrain"
case "$(uname -s)" in
  Darwin) if [ "$(uname -m)" = arm64 ]; then FILE=secondbrain-macos-arm64; else FILE=secondbrain-macos-x64; fi ;;
  *) FILE=secondbrain-linux-x64 ;;
esac
if [ ! -x bin/secondbrain ]; then
  echo "Đang tải chương trình Second Brain lần đầu…"
  mkdir -p bin
  if ! curl -fL --progress-bar "https://github.com/$REPO/releases/download/ban-moi-nhat/$FILE" -o bin/secondbrain; then
    echo "Không tải được. Kiểm tra mạng, hoặc nhờ Claude: \"cài chương trình Second Brain giúp tôi\"."
    read -r -p "Nhấn Enter để đóng…"
    exit 1
  fi
  chmod +x bin/secondbrain
fi
./bin/secondbrain mo
