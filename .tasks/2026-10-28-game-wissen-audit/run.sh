#!/bin/bash
out="$1"; : > "$out"
n=0
while IFS= read -r q; do
  n=$((n+1))
  ( r=$(/tmp/docs-refresh/ask.sh "$q" 2>/dev/null); jq -nc --argjson n $n --arg q "$q" --arg r "$r" '{n:$n,q:$q,r:($r|fromjson? // $r)}' >> "$out" ) &
  while [ "$(jobs -rp | wc -l)" -ge 4 ]; do sleep 2; done
done < questions.txt
wait
