import { createRouter, createWebHistory } from 'vue-router';
import AbortView from '@/views/AbortView.vue';
import CompleteView from '@/views/CompleteView.vue';
import ErrorView from '@/views/ErrorView.vue';
import HostnameView from '@/views/HostnameView.vue';
import UserView from '@/views/UserView.vue';
import ConfirmView from '@/views/ConfirmView.vue';
import SwapFileView from '@/views/SwapFileView.vue';
import LocaleView from '@/views/LocaleView.vue';
// import MirrorView from "@/views/MirrorView.vue";
import MirrorSelectView from '@/views/MirrorSelectView.vue';
// import RescueKitView from "@/views/RescueKitView.vue";
import PartitionView from '@/views/PartitionView.vue';
import VariantView from '@/views/VariantView.vue';
import InstallView from '@/views/InstallView.vue';
import DriverView from '@/views/DriverView.vue';
import DeviceView from '@/views/DeviceView.vue';
import EspPartitionView from '@/views/EspPartitionView.vue';
// import DesktopOrInstall from '@/views/DesktopOrInstall.vue';
import WelcomeView from '@/views/WelcomeView.vue';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: WelcomeView,
      meta: { steps: 0, next: '/variants' },
    },
    {
      path: '/users',
      name: 'users',
      component: UserView,
      meta: { steps: 1, next: '/hostname' },
    },
    {
      path: '/abort',
      name: 'abort',
      component: AbortView,
    },
    {
      path: '/finish',
      name: 'complete',
      component: CompleteView,
      meta: { steps: 4 },
    },
    {
      path: '/error/:message',
      name: 'error',
      props: (route) => ({ ...route.query, ...route.params }),
      component: ErrorView,
    },
    {
      path: '/swapfile',
      name: 'swapfile',
      component: SwapFileView,
      meta: { steps: 1, next: '/confirm' },
    },
    {
      path: '/mirrors-sel',
      name: 'mirrors-sel',
      component: MirrorSelectView,
      meta: { steps: 0, next: '/device' },
    },
    {
      path: '/device',
      name: 'device',
      component: DeviceView,
      meta: { steps: 0, next: '/partitions' },
    },
    {
      path: '/locales',
      name: 'locales',
      component: LocaleView,
      meta: { steps: 1, next: '/swapfile' },
    },
    {
      path: '/confirm',
      name: 'confirm',
      component: ConfirmView,
      meta: { steps: 1, next: '/install' },
    },
    {
      path: '/hostname',
      name: 'hostname',
      component: HostnameView,
      meta: { steps: 1, next: '/locales' },
    },
    {
      path: '/partitions',
      name: 'partitions',
      component: PartitionView,
      meta: { steps: 0, next: '/users' },
    },
    {
      path: '/variants',
      name: 'variants',
      component: VariantView,
      meta: { steps: 0, next: '/mirrors-sel' },
    },
    {
      path: '/install',
      name: 'install',
      component: InstallView,
      meta: { steps: 2 },
    },
    {
      path: '/driver',
      name: 'driver',
      component: DriverView,
    },
    {
      path: '/esp/:esps',
      name: 'esp',
      component: EspPartitionView,
      props: true,
      meta: { steps: 0, next: '/partitions' },
    },
  ],
});

export default router;
