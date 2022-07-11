import { defineConfig }	from 'vite'
import rust            	from '@wasm-tool/rollup-plugin-rust';
import { resolve }     	from 'path';
import fs              	from 'fs';
import WindiCSS        	from 'vite-plugin-windicss'
import windiConfig     	from './windi.config'

const r   	= (...args) => resolve(__dirname, ...args);
const rr  	= (...args) => require.resolve(...args);
const port	= 5176;

export const sharedConfig = {
  root        	: r('src'),
  publicDir   	: r('static'),
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
  const pf = { dist 	: 'webextension-polyfill/dist/',
               fMin 	: 'browser-polyfill.min.js',
               fMMap	: 'browser-polyfill.min.js.map' }
  return {
  ...sharedConfig,
  base  	: command === 'serve' ? `http://localhost:${port}/` : '/dist/',
  server	: {port, hmr:{host:'localhost'}, open:true },
  build:{
    watch             	: isDev ? {buildDelay:500, include:'src/**'} : undefined,
    outDir            	: r('dist'),
    emptyOutDir       	: false,
    minify            	: isDev ?  false   : true,
    sourcemap         	: isDev ? 'inline' : false,
    rollupOptions     	: {
      input           	: {
        background    	: r('src/background/Cargo.toml'),
        popup         	: r('src/popup/Cargo.toml'),
        options       	: r('src/options/Cargo.toml'),
        popup_windi   	: r('src/popup/popup_windi.html'),
        options_windi 	: r('src/options/options_windi.html'),
      }               	,
      output          	: {
        dir           	: r('dist'),
        format        	: 'es',
        entryFileNames	: `js/[name].js`,
        assetFileNames	:  ((AssetInfo) => assetSort(AssetInfo.name)) ,
      }               	,
    }                 	,
    target            	: 'esnext',
  },
  plugins: [
    ...sharedConfig.plugins,
    WindiCSS({config:windiConfig}),
    copyAndWatch(r('src/options/index.html')	, 'options/index.html'),
    copyAndWatch(r('src/popup/index.html')  	, 'popup/index.html'),
    copyAndWatch(rr(pf['dist']+pf['fMin'])  	, 'js/'+pf['fMin']),
    copyAndWatch(rr(pf['dist']+pf['fMMap']) 	, 'js/'+pf['fMMap']),
  ],
}})

function assetSort(name) {
  let i   = name.lastIndexOf('.');
  let ext = name.substring(i+1);
  switch (ext) {
    case 'wasm'	: return  `wasm/[name][extname]`; break;
    case 'css' 	: return   `css/[name][extname]`; break;
    default    	: return `asset/[name][extname]`;
  }
}

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
