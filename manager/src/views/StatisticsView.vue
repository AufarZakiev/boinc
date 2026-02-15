<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from "vue";
import { useStatisticsStore } from "../stores/statistics";
import type { DailyStats, ProjectStatistics } from "../types/boinc";
import PageHeader from "../components/PageHeader.vue";
import EmptyState from "../components/EmptyState.vue";

const store = useStatisticsStore();

type ViewMode = "single" | "all" | "total";

const viewMode = ref<ViewMode>("single");
const selectedProjectUrl = ref("");
const hoveredPoint = ref<{ x: number; y: number; label: string; value: number } | null>(null);

const seriesConfig = [
  { key: "user_total_credit" as const, label: "User Total", color: "#3b82f6" },
  { key: "user_expavg_credit" as const, label: "User Avg", color: "#8b5cf6" },
  { key: "host_total_credit" as const, label: "Host Total", color: "#10b981" },
  { key: "host_expavg_credit" as const, label: "Host Avg", color: "#f59e0b" },
];

const enabledSeries = ref<Set<string>>(new Set(seriesConfig.map((s) => s.key)));

function toggleSeries(key: string) {
  const next = new Set(enabledSeries.value);
  if (next.has(key)) {
    if (next.size > 1) next.delete(key);
  } else {
    next.add(key);
  }
  enabledSeries.value = next;
}

const projectUrls = computed(() => store.projectStats.map((p) => p.master_url));

const activeProject = computed(() => {
  if (!selectedProjectUrl.value && store.projectStats.length > 0) {
    selectedProjectUrl.value = store.projectStats[0].master_url;
  }
  return store.projectStats.find((p) => p.master_url === selectedProjectUrl.value);
});

const chartData = computed(() => {
  if (store.projectStats.length === 0) return null;

  let stats: DailyStats[];

  if (viewMode.value === "single") {
    if (!activeProject.value) return null;
    stats = activeProject.value.daily_statistics;
  } else if (viewMode.value === "all") {
    // Merge all projects by day
    const dayMap = new Map<number, DailyStats>();
    for (const project of store.projectStats) {
      for (const ds of project.daily_statistics) {
        const existing = dayMap.get(ds.day);
        if (existing) {
          existing.user_total_credit += ds.user_total_credit;
          existing.user_expavg_credit += ds.user_expavg_credit;
          existing.host_total_credit += ds.host_total_credit;
          existing.host_expavg_credit += ds.host_expavg_credit;
        } else {
          dayMap.set(ds.day, { ...ds });
        }
      }
    }
    stats = Array.from(dayMap.values()).sort((a, b) => a.day - b.day);
  } else {
    // "total" mode same as "all" but summed
    const dayMap = new Map<number, DailyStats>();
    for (const project of store.projectStats) {
      for (const ds of project.daily_statistics) {
        const existing = dayMap.get(ds.day);
        if (existing) {
          existing.user_total_credit += ds.user_total_credit;
          existing.user_expavg_credit += ds.user_expavg_credit;
          existing.host_total_credit += ds.host_total_credit;
          existing.host_expavg_credit += ds.host_expavg_credit;
        } else {
          dayMap.set(ds.day, { ...ds });
        }
      }
    }
    stats = Array.from(dayMap.values()).sort((a, b) => a.day - b.day);
  }

  if (stats.length === 0) return null;
  return stats;
});

const SVG_WIDTH = 700;
const SVG_HEIGHT = 340;
const PADDING = { top: 20, right: 20, bottom: 50, left: 70 };

const chartW = SVG_WIDTH - PADDING.left - PADDING.right;
const chartH = SVG_HEIGHT - PADDING.top - PADDING.bottom;

function dayToDate(day: number): Date {
  return new Date(day * 86400 * 1000);
}

function formatDateShort(day: number): string {
  const d = dayToDate(day);
  return d.toLocaleDateString(undefined, { month: "short", day: "numeric" });
}

const yMax = computed(() => {
  if (!chartData.value) return 1;
  let max = 0;
  for (const ds of chartData.value) {
    for (const s of seriesConfig) {
      if (enabledSeries.value.has(s.key)) {
        max = Math.max(max, ds[s.key]);
      }
    }
  }
  return max > 0 ? max : 1;
});

const xRange = computed(() => {
  if (!chartData.value || chartData.value.length === 0) return { min: 0, max: 1 };
  const days = chartData.value.map((d) => d.day);
  const min = Math.min(...days);
  const max = Math.max(...days);
  return { min, max: max === min ? min + 1 : max };
});

function scaleX(day: number): number {
  const range = xRange.value;
  return PADDING.left + ((day - range.min) / (range.max - range.min)) * chartW;
}

function scaleY(value: number): number {
  return PADDING.top + chartH - (value / yMax.value) * chartH;
}

function polylinePoints(key: keyof DailyStats): string {
  if (!chartData.value) return "";
  return chartData.value
    .map((ds) => `${scaleX(ds.day)},${scaleY(ds[key] as number)}`)
    .join(" ");
}

const xTicks = computed(() => {
  if (!chartData.value || chartData.value.length === 0) return [];
  const days = chartData.value.map((d) => d.day);
  const count = Math.min(days.length, 8);
  const step = Math.max(1, Math.floor(days.length / count));
  const ticks: number[] = [];
  for (let i = 0; i < days.length; i += step) {
    ticks.push(days[i]);
  }
  return ticks;
});

const yTicks = computed(() => {
  const max = yMax.value;
  const ticks: number[] = [];
  const step = max / 5;
  for (let i = 0; i <= 5; i++) {
    ticks.push(step * i);
  }
  return ticks;
});

function formatCredit(value: number): string {
  if (value >= 1e6) return `${(value / 1e6).toFixed(1)}M`;
  if (value >= 1e3) return `${(value / 1e3).toFixed(1)}K`;
  return value.toFixed(0);
}

function handleChartMouseMove(event: MouseEvent) {
  if (!chartData.value || chartData.value.length === 0) return;
  const svg = event.currentTarget as SVGSVGElement;
  const rect = svg.getBoundingClientRect();
  const mouseX = event.clientX - rect.left;

  // Find closest data point
  let closest: DailyStats | null = null;
  let closestDist = Infinity;
  for (const ds of chartData.value) {
    const sx = scaleX(ds.day);
    const dist = Math.abs(sx - mouseX);
    if (dist < closestDist) {
      closestDist = dist;
      closest = ds;
    }
  }

  if (closest && closestDist < 30) {
    // Find the best enabled series value
    let bestKey = "";
    let bestVal = 0;
    for (const s of seriesConfig) {
      if (enabledSeries.value.has(s.key)) {
        const val = closest[s.key];
        if (val > bestVal) {
          bestVal = val;
          bestKey = s.label;
        }
      }
    }
    hoveredPoint.value = {
      x: scaleX(closest.day),
      y: scaleY(bestVal),
      label: `${formatDateShort(closest.day)} - ${bestKey}: ${formatCredit(bestVal)}`,
      value: bestVal,
    };
  } else {
    hoveredPoint.value = null;
  }
}

function handleChartMouseLeave() {
  hoveredPoint.value = null;
}

onMounted(() => {
  store.fetchStatistics();
});

onUnmounted(() => {
  // nothing to clean up in this component
});
</script>

<template>
  <div class="statistics-view">
    <PageHeader title="Statistics" />

    <p v-if="store.error" class="error-text">{{ store.error }}</p>

    <div v-if="store.loading && store.projectStats.length === 0" class="loading-text">
      Loading statistics...
    </div>

    <EmptyState
      v-else-if="store.projectStats.length === 0 && !store.loading"
      icon="&#x1f4ca;"
      message="No statistics available yet. Attach a project and compute to see data."
    />

    <template v-else>
      <!-- Segmented control -->
      <div class="segmented-control">
        <button
          :class="['segment', { active: viewMode === 'single' }]"
          @click="viewMode = 'single'"
        >
          Single Project
        </button>
        <button
          :class="['segment', { active: viewMode === 'all' }]"
          @click="viewMode = 'all'"
        >
          All Together
        </button>
        <button
          :class="['segment', { active: viewMode === 'total' }]"
          @click="viewMode = 'total'"
        >
          Total
        </button>
      </div>

      <!-- Project picker -->
      <div v-if="viewMode === 'single'" class="project-picker">
        <label class="picker-label">Project</label>
        <select v-model="selectedProjectUrl" class="picker-select">
          <option v-for="url in projectUrls" :key="url" :value="url">
            {{ url }}
          </option>
        </select>
      </div>

      <!-- Series toggles -->
      <div class="series-toggles">
        <label
          v-for="s in seriesConfig"
          :key="s.key"
          class="series-toggle"
        >
          <input
            type="checkbox"
            :checked="enabledSeries.has(s.key)"
            @change="toggleSeries(s.key)"
          />
          <span class="series-swatch" :style="{ background: s.color }"></span>
          <span class="series-label">{{ s.label }}</span>
        </label>
      </div>

      <!-- SVG Chart -->
      <div class="chart-container">
        <svg
          :width="SVG_WIDTH"
          :height="SVG_HEIGHT"
          class="chart-svg"
          @mousemove="handleChartMouseMove"
          @mouseleave="handleChartMouseLeave"
        >
          <!-- Grid lines -->
          <line
            v-for="tick in yTicks"
            :key="'yg-' + tick"
            :x1="PADDING.left"
            :y1="scaleY(tick)"
            :x2="SVG_WIDTH - PADDING.right"
            :y2="scaleY(tick)"
            class="grid-line"
          />

          <!-- Y axis labels -->
          <text
            v-for="tick in yTicks"
            :key="'yl-' + tick"
            :x="PADDING.left - 8"
            :y="scaleY(tick) + 4"
            class="axis-label axis-label-y"
          >
            {{ formatCredit(tick) }}
          </text>

          <!-- X axis labels -->
          <text
            v-for="tick in xTicks"
            :key="'xl-' + tick"
            :x="scaleX(tick)"
            :y="SVG_HEIGHT - PADDING.bottom + 20"
            class="axis-label axis-label-x"
          >
            {{ formatDateShort(tick) }}
          </text>

          <!-- Axis lines -->
          <line
            :x1="PADDING.left"
            :y1="PADDING.top"
            :x2="PADDING.left"
            :y2="PADDING.top + chartH"
            class="axis-line"
          />
          <line
            :x1="PADDING.left"
            :y1="PADDING.top + chartH"
            :x2="SVG_WIDTH - PADDING.right"
            :y2="PADDING.top + chartH"
            class="axis-line"
          />

          <!-- Data series -->
          <template v-if="chartData && chartData.length > 1">
            <polyline
              v-for="s in seriesConfig.filter((sc) => enabledSeries.has(sc.key))"
              :key="s.key"
              :points="polylinePoints(s.key as keyof DailyStats)"
              fill="none"
              :stroke="s.color"
              stroke-width="2"
              stroke-linejoin="round"
              stroke-linecap="round"
            />
          </template>

          <!-- Single point indicator -->
          <template v-if="chartData && chartData.length === 1">
            <circle
              v-for="s in seriesConfig.filter((sc) => enabledSeries.has(sc.key))"
              :key="'dot-' + s.key"
              :cx="scaleX(chartData[0].day)"
              :cy="scaleY(chartData[0][s.key as keyof DailyStats] as number)"
              r="4"
              :fill="s.color"
            />
          </template>

          <!-- Hover indicator -->
          <template v-if="hoveredPoint">
            <line
              :x1="hoveredPoint.x"
              :y1="PADDING.top"
              :x2="hoveredPoint.x"
              :y2="PADDING.top + chartH"
              stroke="var(--color-text-tertiary)"
              stroke-width="1"
              stroke-dasharray="4 2"
            />
            <circle
              :cx="hoveredPoint.x"
              :cy="hoveredPoint.y"
              r="5"
              fill="var(--color-accent)"
              stroke="white"
              stroke-width="2"
            />
          </template>
        </svg>

        <!-- Tooltip -->
        <div
          v-if="hoveredPoint"
          class="chart-tooltip"
          :style="{
            left: hoveredPoint.x + 'px',
            top: (hoveredPoint.y - 10) + 'px',
          }"
        >
          {{ hoveredPoint.label }}
        </div>
      </div>
    </template>
  </div>
</template>

<style scoped>
.statistics-view {
  padding: var(--space-lg);
}

.error-text {
  color: var(--color-danger);
  font-size: var(--font-size-md);
}

.loading-text {
  color: var(--color-text-secondary);
  font-size: var(--font-size-md);
  padding: var(--space-xl) 0;
}

.segmented-control {
  display: inline-flex;
  background: var(--color-bg-tertiary);
  border-radius: var(--radius-md);
  padding: 2px;
  margin-bottom: var(--space-lg);
}

.segment {
  padding: 6px 16px;
  border: none;
  background: transparent;
  font-size: var(--font-size-md);
  color: var(--color-text-secondary);
  cursor: pointer;
  border-radius: var(--radius-sm);
  transition: all var(--transition-fast);
  font-weight: 500;
}

.segment.active {
  background: var(--color-bg);
  color: var(--color-text-primary);
  box-shadow: var(--shadow-sm);
}

.segment:hover:not(.active) {
  color: var(--color-text-primary);
}

.project-picker {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  margin-bottom: var(--space-lg);
}

.picker-label {
  font-size: var(--font-size-md);
  color: var(--color-text-secondary);
  font-weight: 500;
}

.picker-select {
  padding: 6px 12px;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  background: var(--color-bg);
  font-size: var(--font-size-md);
  color: var(--color-text-primary);
  min-width: 280px;
  outline: none;
  transition: border-color var(--transition-fast);
}

.picker-select:focus {
  border-color: var(--color-accent);
}

.series-toggles {
  display: flex;
  gap: var(--space-lg);
  margin-bottom: var(--space-md);
  flex-wrap: wrap;
}

.series-toggle {
  display: flex;
  align-items: center;
  gap: var(--space-xs);
  font-size: var(--font-size-md);
  color: var(--color-text-secondary);
  cursor: pointer;
  user-select: none;
}

.series-toggle input {
  display: none;
}

.series-swatch {
  width: 12px;
  height: 12px;
  border-radius: 3px;
}

.series-label {
  font-weight: 500;
}

.series-toggle:has(input:not(:checked)) .series-swatch {
  opacity: 0.3;
}

.series-toggle:has(input:not(:checked)) .series-label {
  opacity: 0.5;
  text-decoration: line-through;
}

.chart-container {
  position: relative;
  background: var(--color-bg);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  padding: var(--space-md);
  box-shadow: var(--shadow-sm);
  overflow: hidden;
  display: inline-block;
  max-width: 100%;
}

.chart-svg {
  display: block;
  max-width: 100%;
  height: auto;
}

.grid-line {
  stroke: var(--color-border-light);
  stroke-width: 1;
}

.axis-line {
  stroke: var(--color-border);
  stroke-width: 1;
}

.axis-label {
  font-size: 10px;
  fill: var(--color-text-tertiary);
}

.axis-label-y {
  text-anchor: end;
}

.axis-label-x {
  text-anchor: middle;
}

.chart-tooltip {
  position: absolute;
  transform: translate(-50%, -100%);
  background: var(--color-text-primary);
  color: var(--color-bg);
  padding: 4px 10px;
  border-radius: var(--radius-sm);
  font-size: var(--font-size-xs);
  white-space: nowrap;
  pointer-events: none;
  z-index: 10;
}
</style>
