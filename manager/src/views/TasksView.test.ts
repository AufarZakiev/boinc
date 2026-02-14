import { describe, it, expect, vi, beforeEach } from "vitest";
import { mount } from "@vue/test-utils";
import { setActivePinia, createPinia } from "pinia";
import TasksView from "./TasksView.vue";
import { useTasksStore } from "../stores/tasks";
import type { TaskResult } from "../types/boinc";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

function makeTask(overrides: Partial<TaskResult> = {}): TaskResult {
  return {
    name: "task_001_0",
    wu_name: "task_001",
    project_url: "https://example.com/",
    report_deadline: 1700000000,
    received_time: 1699900000,
    elapsed_time: 3661,
    estimated_cpu_time_remaining: 7200,
    fraction_done: 0.456,
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

describe("TasksView", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
  });

  it("shows empty message when no tasks", () => {
    const wrapper = mount(TasksView);
    expect(wrapper.text()).toContain("No tasks");
  });

  it("shows loading message", () => {
    const store = useTasksStore();
    store.loading = true;

    const wrapper = mount(TasksView);
    expect(wrapper.text()).toContain("Loading tasks");
  });

  it("renders task table with data", () => {
    const store = useTasksStore();
    store.tasks = [
      makeTask({ wu_name: "climate_sim_42", fraction_done: 0.75 }),
      makeTask({
        wu_name: "protein_fold_99",
        fraction_done: 0.25,
        active_task: false,
        state: 1,
      }),
    ];

    const wrapper = mount(TasksView);
    const rows = wrapper.findAll("tbody tr");
    expect(rows).toHaveLength(2);

    // Active task should be first (sorted by active_task, then fraction_done)
    expect(rows[0].text()).toContain("climate_sim_42");
    expect(rows[0].text()).toContain("75.00%");
    expect(rows[0].text()).toContain("Running");

    expect(rows[1].text()).toContain("protein_fold_99");
    expect(rows[1].text()).toContain("Downloading");
  });

  it("formats elapsed time correctly", () => {
    const store = useTasksStore();
    store.tasks = [makeTask({ elapsed_time: 3661 })]; // 1h 1m 1s

    const wrapper = mount(TasksView);
    expect(wrapper.text()).toContain("01:01:01");
  });

  it("shows error message", () => {
    const store = useTasksStore();
    store.error = "Connection lost";

    const wrapper = mount(TasksView);
    expect(wrapper.text()).toContain("Connection lost");
  });

  it("shows suspended status", () => {
    const store = useTasksStore();
    store.tasks = [makeTask({ suspended_via_gui: true })];

    const wrapper = mount(TasksView);
    expect(wrapper.text()).toContain("Suspended");
  });

  it("shows ready to report status", () => {
    const store = useTasksStore();
    store.tasks = [
      makeTask({ ready_to_report: true, active_task: false, state: 5 }),
    ];

    const wrapper = mount(TasksView);
    expect(wrapper.text()).toContain("Ready to report");
  });
});
