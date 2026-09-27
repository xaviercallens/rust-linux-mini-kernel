#!/usr/bin/env bash
set -euo pipefail
N=10
WORK="$HOME/ml_traces"
mkdir -p "$WORK/device_enum" "$WORK/matmul"

echo "iter,wall_seconds" > "$WORK/device_enum_timing.csv"
for i in $(seq 1 "$N"); do
  START=$(date +%s.%N)
  strace -f -tt -o "$WORK/device_enum/trace_$i.log" python3 "$HOME/ml_workload_device_enum.py" > "$WORK/device_enum/stdout_$i.log" 2>&1
  END=$(date +%s.%N)
  echo "$i,$(echo "$END - $START" | bc)" >> "$WORK/device_enum_timing.csv"
  echo "device_enum run $i/$N done"
done

echo "iter,wall_seconds" > "$WORK/matmul_timing.csv"
for i in $(seq 1 "$N"); do
  START=$(date +%s.%N)
  strace -f -tt -o "$WORK/matmul/trace_$i.log" python3 "$HOME/ml_workload_matmul.py" > "$WORK/matmul/stdout_$i.log" 2>&1
  END=$(date +%s.%N)
  echo "$i,$(echo "$END - $START" | bc)" >> "$WORK/matmul_timing.csv"
  echo "matmul run $i/$N done"
done

tar -C "$HOME" -czf "$HOME/ml_traces.tar.gz" ml_traces
echo "Wrote $HOME/ml_traces.tar.gz"
du -sh "$HOME/ml_traces.tar.gz"
