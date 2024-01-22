import { createRouter, createWebHistory } from "vue-router";
import WelcomeView from "@/views/WelcomeView.vue";
import AbortView from "@/views/AbortView.vue";
import CompleteView from "@/views/CompleteView.vue";
import ErrorView from "@/views/ErrorView.vue";
import HostnameView from "@/views/HostnameView.vue";
import UserView from "@/views/UserView.vue";
import ConfirmView from "@/views/ConfirmView.vue";
import SwapFileView from "@/views/SwapFileView.vue";
import LocaleView from "@/views/LocaleView.vue";
import MirrorView from "@/views/MirrorView.vue";
import MirrorSelectView from "@/views/MirrorSelectView.vue";
import RescueKitView from "@/views/RescueKitView.vue";
import PartitionView from "@/views/PartitionView.vue";
import VariantView from "@/views/VariantView.vue";
import InstallView from "@/views/InstallView.vue";
import DriverView from "@/views/DriverView.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "home",
      component: WelcomeView,
      meta: { steps: 0, next: "/variants" },
    },
    {
      path: "/rescue",
      name: "rescue",
      component: RescueKitView,
      meta: { steps: 0, next: "/users" },
    },
    {
      path: "/users",
      name: "users",
      component: UserView,
      meta: { steps: 1, next: "/hostname" },
    },
    {
      path: "/abort",
      name: "abort",
      component: AbortView,
    },
    {
      path: "/finish",
      name: "complete",
      component: CompleteView,
      meta: { steps: 4 },
    },
    {
      path: "/error",
      name: "error",
      props: true,
      component: ErrorView,
    },
    {
      path: "/swapfile",
      name: "swapfile",
      component: SwapFileView,
      meta: { steps: 1, next: "/confirm" },
    },
    {
      path: "/mirrors",
      name: "mirrors",
      component: MirrorView,
      meta: { steps: 0, next: "/partitions" },
    },
    {
      path: "/mirrors-sel",
      name: "mirrors-sel",
      component: MirrorSelectView,
      meta: { steps: 0, next: "/partitions" },
    },
    {
      path: "/locales",
      name: "locales",
      component: LocaleView,
      meta: { steps: 1, next: "/swapfile" },
    },
    {
      path: "/confirm",
      name: "confirm",
      component: ConfirmView,
      meta: { steps: 1, next: "/install" },
    },
    {
      path: "/hostname",
      name: "hostname",
      component: HostnameView,
      meta: { steps: 1, next: "/locales" },
    },
    {
      path: "/partitions",
      name: "partitions",
      component: PartitionView,
      meta: { steps: 0, next: "/rescue" },
    },
    {
      path: "/variants",
      name: "variants",
      component: VariantView,
      meta: { steps: 0, next: "/mirrors" },
    },
    {
      path: "/install",
      name: "install",
      component: InstallView,
      meta: { steps: 2 },
    },
    {
      path: "/driver",
      name: "driver",
      component: DriverView,
    },
  ],
});

export default router;
