import { defineConfig } from 'umi';

export default defineConfig({
  nodeModulesTransform: {
    type: 'none',
  },
  routes: [
    { path: '/', component: '@/pages/index' },
  ],
  fastRefresh: {},
  base: '/my/',
  hash: true,
  dynamicImport: {
    loading: '@/Loading',
  },
});
