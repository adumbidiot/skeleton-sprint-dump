// rollup.config.js
import svelte from 'rollup-plugin-svelte';
import nodeResolve from 'rollup-plugin-node-resolve';

export default {
  input: 'src/main.js',
  output: {
    file: 'dist-scripts/skeleton-sprint-lvlbuilder.js',
    format: 'cjs',
	name: 'SKSLvlBuilder'
  },
  plugins: [
    svelte(),
	nodeResolve()
  ]
}