import { createRouter, createWebHistory } from "vue-router";
import ConnectView from "../views/ConnectView.vue";
import TasksView from "../views/TasksView.vue";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: "/", component: ConnectView },
    { path: "/tasks", component: TasksView },
  ],
});

export default router;
