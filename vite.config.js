import { defineConfig }	from 'vite'
import rust            	from '@wasm-tool/rollup-plugin-rust';
import { resolve }     	from 'path';

const r   	= (...args) => resolve(__dirname, ...args);
const port	= 5176;

export const sharedConfig = {
  // root     	: r('src'),
  resolve     	: {
    conditions	: ['development', 'browser'],
  },
  plugins:[
    rust({
      serverPath   	: 'dist/js/',
      // importHook	: function (path)	{ // for browser extensions since their files are put into a separate URL namespace, so need chrome.runtime.getURL to get the correct URL
      //   return  	  'browser.runtime.getURL(' + JSON.stringify(path) + ')';
      // }         	,
    }),
  ],
  optimizeDeps	: {
    include   	: [
    ],
    exclude	: [],
  },
}

export default defineConfig(({ command, mode }) => {
  const isDev = mode==='development';
  return {
  ...sharedConfig,
  base  	: command === 'serve' ? `http://localhost:${port}/` : '/dist/',
  server	: {port, hmr:{host:'localhost'}, open:true },
  build:{
    watch             	: isDev ? {buildDelay:500, include:'src/**'} : undefined,
    outDir            	: r('dist'),
    emptyOutDir       	: false,
    assetsDir         	: 'wasm', // ['assets']
    minify            	: isDev ?  false   : true,
    sourcemap         	: isDev ? 'inline' : false,
    rollupOptions     	: {
      input           	: {
        index         	: 'Cargo.toml',
      }               	,
      output          	: {
        dir           	: r('dist/js'),
        format        	: 'es',
        entryFileNames	: `[name].js`,
      }               	,
    }                 	,
    target            	: 'esnext',
  },
  plugins: [
    ...sharedConfig.plugins,
  ],
}})
