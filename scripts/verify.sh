#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

mapfile -t sol_pkgs < <(cargo metadata --no-deps --format-version 1 \
  | grep -oE '"name":"sol-[^"]+"' | sed 's/"name":"//;s/"//')

if [ "${#sol_pkgs[@]}" -eq 0 ]; then
  echo "Не найдено ни одного крейта sol-*"; exit 1
fi

args=()
for p in "${sol_pkgs[@]}"; do args+=(-p "$p"); done

echo "Проверяю ${#sol_pkgs[@]} решений: ${sol_pkgs[*]}"
cargo test "${args[@]}"
echo "OK: все решения зелёные."
