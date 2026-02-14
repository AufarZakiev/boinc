import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { setActivePinia, createPinia } from "pinia";
import { useTasksStore } from "./tasks";
import type { TaskResult } from "../types/boinc";

// Mock the Tauri invoke layer
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

import { invoke } from "@tauri-apps/api/core";

const mockInvoke = vi.mocked(invoke);

function makeTask(overrides: Partial<TaskResult> = {}): TaskResult {
  return {
    name: "task_001_0",
    wu_name: "task_001",
    project_url: "https://example.com/",
    report_deadline: 1700000000,
    received_time: 1699900000,
    elapsed_time: 600,
    estimated_cpu_time_remaining: 1200,
    fraction_done: 0.33,
    state: 2,
    scheduler_state: 2,
    active_task_state: 1,
    active_task: true,
    suspended_via_gui: false,
    project_suspended_via_gui: false,
    ready_to_report: false,
    got_server_ack: false,
    plan_class: "",
    resources: "1 CPU",
    ...overrides,
  };
}

describe("useTasksStore", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
    vi.restoreAllMocks();
  });

  it("starts with empty state", () => {
    const store = useTasksStore();
    expect(store.tasks).toEqual([]);
    expect(store.loading).toBe(false);
    expect(store.error).toBeNull();
  });

  it("fetches tasks via RPC", async () => {
    const tasks = [makeTask(), makeTask({ name: "task_002_0" })];
    mockInvoke.mockResolvedValueOnce(tasks);

    const store = useTasksStore();
    await store.fetchTasks();

    expect(mockInvoke).toHaveBeenCalledWith("get_results", {
      activeOnly: false,
    });
    expect(store.tasks).toEqual(tasks);
    expect(store.loading).toBe(false);
    expect(store.error).toBeNull();
  });

  it("sets error on RPC failure", async () => {
    mockInvoke.mockRejectedValueOnce("Connection lost");

    const store = useTasksStore();
    await store.fetchTasks();

    expect(store.error).toBe("Connection lost");
    expect(store.tasks).toEqual([]);
  });

  it("polls at the given interval and stops", async () => {
    mockInvoke.mockResolvedValue([]);

    const store = useTasksStore();
    store.startPolling(1000);

    // Let initial fetch + early ticks settle
    await vi.advanceTimersByTimeAsync(100);
    const initialCalls = mockInvoke.mock.calls.length;
    expect(initialCalls).toBeGreaterThanOrEqual(1);

    // Advance by 2 intervals — should get ~2 more calls
    await vi.advanceTimersByTimeAsync(2000);
    const afterPolling = mockInvoke.mock.calls.length;
    expect(afterPolling).toBeGreaterThan(initialCalls);

    store.stopPolling();
    const afterStop = mockInvoke.mock.calls.length;

    // No more calls after stop
    await vi.advanceTimersByTimeAsync(5000);
    expect(mockInvoke.mock.calls.length).toBe(afterStop);
  });

  it("stopPolling is idempotent", () => {
    const store = useTasksStore();
    store.stopPolling(); // should not throw
    store.stopPolling();
  });
});
