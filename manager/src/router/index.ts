import { createRouter, createWebHistory } from "vue-router";
import ConnectView from "../views/ConnectView.vue";
import TasksView from "../views/TasksView.vue";
import ProjectsView from "../views/ProjectsView.vue";
import TransfersView from "../views/TransfersView.vue";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: "/", component: ConnectView },
    { path: "/tasks", component: TasksView },
    { path: "/projects", component: ProjectsView },
    { path: "/transfers", component: TransfersView },
  ],
});

export default router;
