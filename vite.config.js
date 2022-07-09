import { defineConfig }	from 'vite'
import rust            	from '@wasm-tool/rollup-plugin-rust';
import { resolve }     	from 'path';
import fs              	from 'fs';

const r   	= (...args) => resolve(__dirname, ...args);
const port	= 5176;

export const sharedConfig = {
  // root     	: r('src'),
  resolve     	: {
    conditions	: ['development', 'browser'],
  },
  plugins:[
    rust({
      importHook	: function (path)	{ // browser ext' files are in a separate URL namespace
        return  	  'browser.runtime.getURL(' + JSON.stringify(path) + ')';
      }         	,
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
        background    	: r('src/background/Cargo.toml'),
        popup         	: r('src/popup/Cargo.toml'),
        options       	: r('src/options/Cargo.toml'),
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

function copyAndWatch(fileIn, fileOut) { return {
  name: 'copy-and-watch',
  async buildStart() { this.addWatchFile(fileIn); },
  async generateBundle() {
    this.emitFile({
      type    	: 'asset',
      fileName	: fileOut,
      source  	: fs.readFileSync(fileIn)
    }); }
} }
