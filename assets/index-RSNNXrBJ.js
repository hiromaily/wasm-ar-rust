var xo=Object.defineProperty;var Bo=(m,w,k)=>w in m?xo(m,w,{enumerable:!0,configurable:!0,writable:!0,value:k}):m[w]=k;var So=(m,w)=>()=>(w||m((w={exports:{}}).exports,w),w.exports);var be=(m,w,k)=>Bo(m,typeof w!="symbol"?w+"":w,k);var vo=So((ko,ge)=>{(async()=>{(function(){const e=document.createElement("link").relList;if(e&&e.supports&&e.supports("modulepreload"))return;for(const _ of document.querySelectorAll('link[rel="modulepreload"]'))t(_);new MutationObserver(_=>{for(const o of _)if(o.type==="childList")for(const a of o.addedNodes)a.tagName==="LINK"&&a.rel==="modulepreload"&&t(a)}).observe(document,{childList:!0,subtree:!0});function n(_){const o={};return _.integrity&&(o.integrity=_.integrity),_.referrerPolicy&&(o.referrerPolicy=_.referrerPolicy),_.crossOrigin==="use-credentials"?o.credentials="include":_.crossOrigin==="anonymous"?o.credentials="omit":o.credentials="same-origin",o}function t(_){if(_.ep)return;_.ep=!0;const o=n(_);fetch(_.href,o)}})();function m(){}function w(e){return e()}function k(){return Object.create(null)}function L(e){e.forEach(w)}function le(e){return typeof e=="function"}function ee(e,n){return e!=e?n==n:e!==n||e&&typeof e=="object"||typeof e=="function"}function Pe(e){return Object.keys(e).length===0}function me(e,n){e.appendChild(n)}function I(e,n,t){e.insertBefore(n,t||null)}function v(e){e.parentNode&&e.parentNode.removeChild(e)}function q(e){return document.createElement(e)}function V(e){return document.createTextNode(e)}function we(){return V(" ")}function Ce(){return V("")}function ne(e,n,t){t==null?e.removeAttribute(n):e.getAttribute(n)!==t&&e.setAttribute(n,t)}function Te(e){return Array.from(e.childNodes)}function Ae(e,n){n=""+n,e.data!==n&&(e.data=n)}function T(e,n,t,_){t==null?e.style.removeProperty(n):e.style.setProperty(n,t,"")}let M;function R(e){M=e}function Ee(){if(!M)throw new Error("Function called outside component initialization");return M}function We(e){Ee().$$.on_mount.push(e)}const A=[],te=[];let E=[];const pe=[],Oe=Promise.resolve();let re=!1;function Ge(){re||(re=!0,Oe.then(he))}function _e(e){E.push(e)}const oe=new Set;let W=0;function he(){if(W!==0)return;const e=M;do{try{for(;W<A.length;){const n=A[W];W++,R(n),De(n.$$)}}catch(n){throw A.length=0,W=0,n}for(R(null),A.length=0,W=0;te.length;)te.pop()();for(let n=0;n<E.length;n+=1){const t=E[n];oe.has(t)||(oe.add(t),t())}E.length=0}while(A.length);for(;pe.length;)pe.pop()();re=!1,oe.clear(),R(e)}function De(e){if(e.fragment!==null){e.update(),L(e.before_update);const n=e.dirty;e.dirty=[-1],e.fragment&&e.fragment.p(e.ctx,n),e.after_update.forEach(_e)}}function ze(e){const n=[],t=[];E.forEach(_=>e.indexOf(_)===-1?n.push(_):t.push(_)),t.forEach(_=>_()),E=n}const N=new Set;let P;function Le(){P={r:0,c:[],p:P}}function Me(){P.r||L(P.c),P=P.p}function O(e,n){e&&e.i&&(N.delete(e),e.i(n))}function H(e,n,t,_){if(e&&e.o){if(N.has(e))return;N.add(e),P.c.push(()=>{N.delete(e),_&&(t&&e.d(1),_())}),e.o(n)}else _&&_()}function ye(e){e&&e.c()}function ce(e,n,t){const{fragment:_,after_update:o}=e.$$;_&&_.m(n,t),_e(()=>{const a=e.$$.on_mount.map(w).filter(le);e.$$.on_destroy?e.$$.on_destroy.push(...a):L(a),e.$$.on_mount=[]}),o.forEach(_e)}function ae(e,n){const t=e.$$;t.fragment!==null&&(ze(t.after_update),L(t.on_destroy),t.fragment&&t.fragment.d(n),t.on_destroy=t.fragment=null,t.ctx=[])}function Re(e,n){e.$$.dirty[0]===-1&&(A.push(e),Ge(),e.$$.dirty.fill(0)),e.$$.dirty[n/31|0]|=1<<n%31}function ue(e,n,t,_,o,a,u=null,s=[-1]){const g=M;R(e);const i=e.$$={fragment:null,ctx:[],props:a,update:m,not_equal:o,bound:k(),on_mount:[],on_destroy:[],on_disconnect:[],before_update:[],after_update:[],context:new Map(n.context||(g?g.$$.context:[])),callbacks:k(),dirty:s,skip_bound:!1,root:n.target||g.$$.root};u&&u(i.root);let b=!1;if(i.ctx=t?t(e,n.props||{},(d,Z,...J)=>{const K=J.length?J[0]:Z;return i.ctx&&o(i.ctx[d],i.ctx[d]=K)&&(!i.skip_bound&&i.bound[d]&&i.bound[d](K),b&&Re(e,d)),Z}):[],i.update(),b=!0,L(i.before_update),i.fragment=_?_(i.ctx):!1,n.target){if(n.hydrate){const d=Te(n.target);i.fragment&&i.fragment.l(d),d.forEach(v)}else i.fragment&&i.fragment.c();n.intro&&O(e.$$.fragment),ce(e,n.target,n.anchor),he()}R(g)}class ie{constructor(){be(this,"$$");be(this,"$$set")}$destroy(){ae(this,1),this.$destroy=m}$on(n,t){if(!le(t))return m;const _=this.$$.callbacks[n]||(this.$$.callbacks[n]=[]);return _.push(t),()=>{const o=_.indexOf(t);o!==-1&&_.splice(o,1)}}$set(n){this.$$set&&!Pe(n)&&(this.$$.skip_bound=!0,this.$$set(n),this.$$.skip_bound=!1)}}const Fe="4";typeof window<"u"&&(window.__svelte||(window.__svelte={v:new Set})).v.add(Fe);const Ue="/wasm-ar-rust/assets/image_detect_mozaic_wasm_bg-WBHxUbMu.wasm",je=async(e={},n)=>{let t;if(n.startsWith("data:")){const _=n.replace(/^data:.*?base64,/,"");let o;if(typeof Buffer=="function"&&typeof Buffer.from=="function")o=Buffer.from(_,"base64");else if(typeof atob=="function"){const a=atob(_);o=new Uint8Array(a.length);for(let u=0;u<a.length;u++)o[u]=a.charCodeAt(u)}else throw new Error("Cannot decode base64-encoded data URL");t=await WebAssembly.instantiate(o,e)}else{const _=await fetch(n),o=_.headers.get("Content-Type")||"";if("instantiateStreaming"in WebAssembly&&o.startsWith("application/wasm"))t=await WebAssembly.instantiateStreaming(_,e);else{const a=await _.arrayBuffer();t=await WebAssembly.instantiate(a,e)}}return t.instance.exports};let f;function qe(e){f=e}const S=new Array(128).fill(void 0);S.push(void 0,null,!0,!1);function r(e){return S[e]}let F=S.length;function Ve(e){e<132||(S[e]=F,F=e)}function G(e){const n=r(e);return Ve(e),n}const Ne=typeof TextDecoder>"u"?(0,ge.require)("util").TextDecoder:TextDecoder;let xe=new Ne("utf-8",{ignoreBOM:!0,fatal:!0});xe.decode();let Q=null;function U(){return(Q===null||Q.byteLength===0)&&(Q=new Uint8Array(f.memory.buffer)),Q}function $(e,n){return e=e>>>0,xe.decode(U().subarray(e,e+n))}function c(e){F===S.length&&S.push(S.length+1);const n=F;return F=S[n],S[n]=e,n}let x=0;const He=typeof TextEncoder>"u"?(0,ge.require)("util").TextEncoder:TextEncoder;let X=new He("utf-8");const Qe=typeof X.encodeInto=="function"?function(e,n){return X.encodeInto(e,n)}:function(e,n){const t=X.encode(e);return n.set(t),{read:e.length,written:t.length}};function C(e,n,t){if(t===void 0){const s=X.encode(e),g=n(s.length,1)>>>0;return U().subarray(g,g+s.length).set(s),x=s.length,g}let _=e.length,o=n(_,1)>>>0;const a=U();let u=0;for(;u<_;u++){const s=e.charCodeAt(u);if(s>127)break;a[o+u]=s}if(u!==_){u!==0&&(e=e.slice(u)),o=t(o,_,_=u+e.length*3,1)>>>0;const s=U().subarray(o+u,o+_),g=Qe(e,s);u+=g.written,o=t(o,_,u,1)>>>0}return x=u,o}function j(e){return e==null}let D=null;function p(){return(D===null||D.buffer.detached===!0||D.buffer.detached===void 0&&D.buffer!==f.memory.buffer)&&(D=new DataView(f.memory.buffer)),D}function fe(e){const n=typeof e;if(n=="number"||n=="boolean"||e==null)return`${e}`;if(n=="string")return`"${e}"`;if(n=="symbol"){const o=e.description;return o==null?"Symbol":`Symbol(${o})`}if(n=="function"){const o=e.name;return typeof o=="string"&&o.length>0?`Function(${o})`:"Function"}if(Array.isArray(e)){const o=e.length;let a="[";o>0&&(a+=fe(e[0]));for(let u=1;u<o;u++)a+=", "+fe(e[u]);return a+="]",a}const t=/\[object ([^\]]+)\]/.exec(toString.call(e));let _;if(t.length>1)_=t[1];else return toString.call(e);if(_=="Object")try{return"Object("+JSON.stringify(e)+")"}catch{return"Object"}return e instanceof Error?`${e.name}: ${e.message}
${e.stack}`:_}const Be=typeof FinalizationRegistry>"u"?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry(e=>{f.__wbindgen_export_2.get(e.dtor)(e.a,e.b)});function se(e,n,t,_){const o={a:e,b:n,cnt:1,dtor:t},a=(...u)=>{o.cnt++;const s=o.a;o.a=0;try{return _(s,o.b,...u)}finally{--o.cnt===0?(f.__wbindgen_export_2.get(o.dtor)(s,o.b),Be.unregister(o)):o.a=s}};return a.original=o,Be.register(a,o,o),a}function Se(e,n,t){f._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h187715d88a0ecad1(e,n,c(t))}function Xe(e,n,t){f._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h0960554546eefafc(e,n,c(t))}function Ye(e,n){const t=n(e.length*1,1)>>>0;return U().set(e,t/1),x=e.length,t}let Y=null;function Ze(){return(Y===null||Y.byteLength===0)&&(Y=new Uint32Array(f.memory.buffer)),Y}function de(e,n){return e=e>>>0,Ze().subarray(e/4,e/4+n)}function B(e,n){try{return e.apply(this,n)}catch(t){f.__wbindgen_exn_store(c(t))}}function Je(e,n,t,_){f.wasm_bindgen__convert__closures__invoke2_mut__h8b05459977c113ac(e,n,c(t),c(_))}const ve=typeof FinalizationRegistry>"u"?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry(e=>f.__wbg_imagedetector_free(e>>>0,1));class Ke{__destroy_into_raw(){const n=this.__wbg_ptr;return this.__wbg_ptr=0,ve.unregister(this),n}free(){const n=this.__destroy_into_raw();f.__wbg_imagedetector_free(n,0)}constructor(n,t,_,o){const a=f.imagedetector_new(n,t,_,o);return this.__wbg_ptr=a>>>0,ve.register(this,this.__wbg_ptr,this),this}increment(){return f.imagedetector_increment(this.__wbg_ptr)>>>0}decrement(){return f.imagedetector_decrement(this.__wbg_ptr)>>>0}detect_image_and_mozaic(n,t,_){const o=Ye(n,f.__wbindgen_malloc),a=x,u=f.imagedetector_detect_image_and_mozaic(this.__wbg_ptr,o,a,t,_);return G(u)}}function en(e){G(e)}function nn(e,n){const t=$(e,n);return c(t)}function tn(){const e=new Error;return c(e)}function rn(e,n){const t=r(n).stack,_=C(t,f.__wbindgen_malloc,f.__wbindgen_realloc),o=x;p().setInt32(e+4*1,o,!0),p().setInt32(e+4*0,_,!0)}function _n(e,n){let t,_;try{t=e,_=n,console.error($(e,n))}finally{f.__wbindgen_free(t,_,1)}}function on(e){let n;try{n=r(e)instanceof GPUValidationError}catch{n=!1}return n}function cn(e,n){const t=r(n).message,_=C(t,f.__wbindgen_malloc,f.__wbindgen_realloc),o=x;p().setInt32(e+4*1,o,!0),p().setInt32(e+4*0,_,!0)}function an(e){let n;try{n=r(e)instanceof GPUOutOfMemoryError}catch{n=!1}return n}function un(e){const n=r(e).type;return{error:0,warning:1,info:2}[n]??3}function fn(e){return r(e).offset}function sn(e){return r(e).length}function dn(e){const n=r(e);return c(n)}function bn(e){const n=r(e).error;return c(n)}function gn(e){const n=G(e).original;return n.cnt--==1?(n.a=0,!0):!1}function ln(e){let n;try{n=r(e)instanceof GPUDeviceLostInfo}catch{n=!1}return n}function mn(e){const n=r(e).reason;return{unknown:0,destroyed:1}[n]??2}function wn(e,n){const t=r(n).message,_=C(t,f.__wbindgen_malloc,f.__wbindgen_realloc),o=x;p().setInt32(e+4*1,o,!0),p().setInt32(e+4*0,_,!0)}function pn(e,n){const t=r(n),_=typeof t=="string"?t:void 0;var o=j(_)?0:C(_,f.__wbindgen_malloc,f.__wbindgen_realloc),a=x;p().setInt32(e+4*1,a,!0),p().setInt32(e+4*0,o,!0)}function hn(e,n,t){return r(e).has($(n,t))}function yn(e){return r(e).maxTextureDimension1D}function xn(e){return r(e).maxTextureDimension2D}function Bn(e){return r(e).maxTextureDimension3D}function Sn(e){return r(e).maxTextureArrayLayers}function vn(e){return r(e).maxBindGroups}function $n(e){return r(e).maxBindingsPerBindGroup}function kn(e){return r(e).maxDynamicUniformBuffersPerPipelineLayout}function In(e){return r(e).maxDynamicStorageBuffersPerPipelineLayout}function Pn(e){return r(e).maxSampledTexturesPerShaderStage}function Cn(e){return r(e).maxSamplersPerShaderStage}function Tn(e){return r(e).maxStorageBuffersPerShaderStage}function An(e){return r(e).maxStorageTexturesPerShaderStage}function En(e){return r(e).maxUniformBuffersPerShaderStage}function Wn(e){return r(e).maxUniformBufferBindingSize}function On(e){return r(e).maxStorageBufferBindingSize}function Gn(e){return r(e).maxVertexBuffers}function Dn(e){return r(e).maxBufferSize}function zn(e){return r(e).maxVertexAttributes}function Ln(e){return r(e).maxVertexBufferArrayStride}function Mn(e){return r(e).minUniformBufferOffsetAlignment}function Rn(e){return r(e).minStorageBufferOffsetAlignment}function Fn(e){return r(e).maxInterStageShaderComponents}function Un(e){return r(e).maxColorAttachments}function jn(e){return r(e).maxColorAttachmentBytesPerSample}function qn(e){return r(e).maxComputeWorkgroupStorageSize}function Vn(e){return r(e).maxComputeInvocationsPerWorkgroup}function Nn(e){return r(e).maxComputeWorkgroupSizeX}function Hn(e){return r(e).maxComputeWorkgroupSizeY}function Qn(e){return r(e).maxComputeWorkgroupSizeZ}function Xn(e){return r(e).maxComputeWorkgroupsPerDimension}function Yn(e){let n;try{n=r(e)instanceof GPUAdapter}catch{n=!1}return n}function Zn(e){const n=r(e).queue;return c(n)}function Jn(e){const n=r(e);return typeof n=="object"&&n!==null}function Kn(e){let n;try{n=r(e)instanceof GPUCanvasContext}catch{n=!1}return n}function et(e,n,t){const _=r(e).getMappedRange(n,t);return c(_)}function nt(e){const n=r(e).Window;return c(n)}function tt(e){return r(e)===void 0}function rt(e){const n=r(e).WorkerGlobalScope;return c(n)}function _t(e){const n=r(e).gpu;return c(n)}function ot(e,n){const t=r(e).requestAdapter(r(n));return c(t)}function ct(e){return c(e)}function at(e,n){const t=r(e).requestDevice(r(n));return c(t)}function ut(e){const n=r(e).features;return c(n)}function it(e){const n=r(e).limits;return c(n)}function ft(e){const n=r(e).getPreferredCanvasFormat();return{r8unorm:0,r8snorm:1,r8uint:2,r8sint:3,r16uint:4,r16sint:5,r16float:6,rg8unorm:7,rg8snorm:8,rg8uint:9,rg8sint:10,r32uint:11,r32sint:12,r32float:13,rg16uint:14,rg16sint:15,rg16float:16,rgba8unorm:17,"rgba8unorm-srgb":18,rgba8snorm:19,rgba8uint:20,rgba8sint:21,bgra8unorm:22,"bgra8unorm-srgb":23,rgb9e5ufloat:24,rgb10a2uint:25,rgb10a2unorm:26,rg11b10ufloat:27,rg32uint:28,rg32sint:29,rg32float:30,rgba16uint:31,rgba16sint:32,rgba16float:33,rgba32uint:34,rgba32sint:35,rgba32float:36,stencil8:37,depth16unorm:38,depth24plus:39,"depth24plus-stencil8":40,depth32float:41,"depth32float-stencil8":42,"bc1-rgba-unorm":43,"bc1-rgba-unorm-srgb":44,"bc2-rgba-unorm":45,"bc2-rgba-unorm-srgb":46,"bc3-rgba-unorm":47,"bc3-rgba-unorm-srgb":48,"bc4-r-unorm":49,"bc4-r-snorm":50,"bc5-rg-unorm":51,"bc5-rg-snorm":52,"bc6h-rgb-ufloat":53,"bc6h-rgb-float":54,"bc7-rgba-unorm":55,"bc7-rgba-unorm-srgb":56,"etc2-rgb8unorm":57,"etc2-rgb8unorm-srgb":58,"etc2-rgb8a1unorm":59,"etc2-rgb8a1unorm-srgb":60,"etc2-rgba8unorm":61,"etc2-rgba8unorm-srgb":62,"eac-r11unorm":63,"eac-r11snorm":64,"eac-rg11unorm":65,"eac-rg11snorm":66,"astc-4x4-unorm":67,"astc-4x4-unorm-srgb":68,"astc-5x4-unorm":69,"astc-5x4-unorm-srgb":70,"astc-5x5-unorm":71,"astc-5x5-unorm-srgb":72,"astc-6x5-unorm":73,"astc-6x5-unorm-srgb":74,"astc-6x6-unorm":75,"astc-6x6-unorm-srgb":76,"astc-8x5-unorm":77,"astc-8x5-unorm-srgb":78,"astc-8x6-unorm":79,"astc-8x6-unorm-srgb":80,"astc-8x8-unorm":81,"astc-8x8-unorm-srgb":82,"astc-10x5-unorm":83,"astc-10x5-unorm-srgb":84,"astc-10x6-unorm":85,"astc-10x6-unorm-srgb":86,"astc-10x8-unorm":87,"astc-10x8-unorm-srgb":88,"astc-10x10-unorm":89,"astc-10x10-unorm-srgb":90,"astc-12x10-unorm":91,"astc-12x10-unorm-srgb":92,"astc-12x12-unorm":93,"astc-12x12-unorm-srgb":94}[n]??95}function st(e,n){r(e).configure(r(n))}function dt(e){const n=r(e).getCurrentTexture();return c(n)}function bt(e){const n=r(e).features;return c(n)}function gt(e){const n=r(e).limits;return c(n)}function lt(e,n){const t=r(e).createShaderModule(r(n));return c(t)}function mt(e,n){const t=r(e).createBindGroupLayout(r(n));return c(t)}function wt(e,n){const t=r(e).createBindGroup(r(n));return c(t)}function pt(e,n){const t=r(e).createPipelineLayout(r(n));return c(t)}function ht(e,n){const t=r(e).createRenderPipeline(r(n));return c(t)}function yt(e,n){const t=r(e).createComputePipeline(r(n));return c(t)}function xt(e,n){const t=r(e).createBuffer(r(n));return c(t)}function Bt(e,n){const t=r(e).createTexture(r(n));return c(t)}function St(e,n){const t=r(e).createSampler(r(n));return c(t)}function vt(e,n){const t=r(e).createQuerySet(r(n));return c(t)}function $t(e,n){const t=r(e).createCommandEncoder(r(n));return c(t)}function kt(e,n){const t=r(e).createRenderBundleEncoder(r(n));return c(t)}function It(e){r(e).destroy()}function Pt(e){const n=r(e).lost;return c(n)}function Ct(e,n){r(e).onuncapturederror=r(n)}function Tt(e,n){r(e).pushErrorScope(["validation","out-of-memory","internal"][n])}function At(e){const n=r(e).popErrorScope();return c(n)}function Et(e,n,t,_){const o=r(e).mapAsync(n>>>0,t,_);return c(o)}function Wt(e){r(e).unmap()}function Ot(e){const n=r(e).getCompilationInfo();return c(n)}function Gt(e){const n=r(e).messages;return c(n)}function Dt(e,n){const t=r(e).createView(r(n));return c(t)}function zt(e){r(e).destroy()}function Lt(e){r(e).destroy()}function Mt(e,n){const t=r(e).getBindGroupLayout(n>>>0);return c(t)}function Rt(e,n){const t=r(e).getBindGroupLayout(n>>>0);return c(t)}function Ft(e,n,t,_,o,a){r(e).copyBufferToBuffer(r(n),t,r(_),o,a)}function Ut(e,n,t,_){r(e).copyBufferToTexture(r(n),r(t),r(_))}function jt(e,n,t,_){r(e).copyTextureToBuffer(r(n),r(t),r(_))}function qt(e,n,t,_){r(e).copyTextureToTexture(r(n),r(t),r(_))}function Vt(e,n){const t=r(e).beginComputePass(r(n));return c(t)}function Nt(e,n){const t=r(e).beginRenderPass(r(n));return c(t)}function Ht(e,n){const t=r(n).label,_=C(t,f.__wbindgen_malloc,f.__wbindgen_realloc),o=x;p().setInt32(e+4*1,o,!0),p().setInt32(e+4*0,_,!0)}function Qt(e,n){const t=r(e).finish(r(n));return c(t)}function Xt(e){const n=r(e).finish();return c(n)}function Yt(e,n,t){r(e).clearBuffer(r(n),t)}function Zt(e,n,t,_){r(e).clearBuffer(r(n),t,_)}function Jt(e,n,t,_,o,a){r(e).resolveQuerySet(r(n),t>>>0,_>>>0,r(o),a>>>0)}function Kt(e){const n=r(e).finish();return c(n)}function er(e,n){const t=r(e).finish(r(n));return c(t)}function nr(e,n,t,_,o,a){r(e).writeBuffer(r(n),t,r(_),o,a)}function tr(e){return r(e).usage}function rr(e){return r(e).size}function _r(e,n,t,_,o){r(e).writeTexture(r(n),r(t),r(_),r(o))}function or(e,n,t,_){r(e).copyExternalImageToTexture(r(n),r(t),r(_))}function cr(e,n){r(e).setPipeline(r(n))}function ar(e,n,t){r(e).setBindGroup(n>>>0,r(t))}function ur(e,n,t,_,o,a,u){r(e).setBindGroup(n>>>0,r(t),de(_,o),a,u>>>0)}function ir(e,n,t,_){r(e).dispatchWorkgroups(n>>>0,t>>>0,_>>>0)}function fr(e,n,t){r(e).dispatchWorkgroupsIndirect(r(n),t)}function sr(e){r(e).end()}function dr(e,n){r(e).setPipeline(r(n))}function br(e,n,t){r(e).setBindGroup(n>>>0,r(t))}function gr(e,n,t,_,o,a,u){r(e).setBindGroup(n>>>0,r(t),de(_,o),a,u>>>0)}function lr(e,n,t,_){r(e).setIndexBuffer(r(n),["uint16","uint32"][t],_)}function mr(e,n,t,_,o){r(e).setIndexBuffer(r(n),["uint16","uint32"][t],_,o)}function wr(e,n,t,_){r(e).setVertexBuffer(n>>>0,r(t),_)}function pr(e,n,t,_,o){r(e).setVertexBuffer(n>>>0,r(t),_,o)}function hr(e,n,t,_,o){r(e).draw(n>>>0,t>>>0,_>>>0,o>>>0)}function yr(e,n,t,_,o,a){r(e).drawIndexed(n>>>0,t>>>0,_>>>0,o,a>>>0)}function xr(e,n,t){r(e).drawIndirect(r(n),t)}function Br(e,n,t){r(e).drawIndexedIndirect(r(n),t)}function Sr(e,n){r(e).setPipeline(r(n))}function vr(e,n,t){r(e).setBindGroup(n>>>0,r(t))}function $r(e,n,t,_,o,a,u){r(e).setBindGroup(n>>>0,r(t),de(_,o),a,u>>>0)}function kr(e,n,t,_){r(e).setIndexBuffer(r(n),["uint16","uint32"][t],_)}function Ir(e,n,t,_,o){r(e).setIndexBuffer(r(n),["uint16","uint32"][t],_,o)}function Pr(e,n,t,_){r(e).setVertexBuffer(n>>>0,r(t),_)}function Cr(e,n,t,_,o){r(e).setVertexBuffer(n>>>0,r(t),_,o)}function Tr(e,n,t,_,o){r(e).draw(n>>>0,t>>>0,_>>>0,o>>>0)}function Ar(e,n,t,_,o,a){r(e).drawIndexed(n>>>0,t>>>0,_>>>0,o,a>>>0)}function Er(e,n,t){r(e).drawIndirect(r(n),t)}function Wr(e,n,t){r(e).drawIndexedIndirect(r(n),t)}function Or(e,n){r(e).setBlendConstant(r(n))}function Gr(e,n,t,_,o){r(e).setScissorRect(n>>>0,t>>>0,_>>>0,o>>>0)}function Dr(e,n,t,_,o,a,u){r(e).setViewport(n,t,_,o,a,u)}function zr(e,n){r(e).setStencilReference(n>>>0)}function Lr(e,n){r(e).executeBundles(r(n))}function Mr(e){r(e).end()}function Rr(e,n){r(e).submit(r(n))}function Fr(e){return r(e).lineNum}function Ur(e,n){const t=r(n).message,_=C(t,f.__wbindgen_malloc,f.__wbindgen_realloc),o=x;p().setInt32(e+4*1,o,!0),p().setInt32(e+4*0,_,!0)}function jr(e){const n=r(e).queueMicrotask;return c(n)}function qr(e){return typeof r(e)=="function"}function Vr(e){queueMicrotask(r(e))}function Nr(e){let n;try{n=r(e)instanceof Window}catch{n=!1}return n}function Hr(e){const n=r(e).document;return j(n)?0:c(n)}function Qr(e){const n=r(e).navigator;return c(n)}function Xr(){return B(function(e,n,t){const _=r(e).querySelectorAll($(n,t));return c(_)},arguments)}function Yr(e){const n=r(e).navigator;return c(n)}function Zr(e,n){const t=r(e)[n>>>0];return j(t)?0:c(t)}function Jr(e,n){r(e).width=n>>>0}function Kr(e,n){r(e).height=n>>>0}function e_(){return B(function(e,n,t){const _=r(e).getContext($(n,t));return j(_)?0:c(_)},arguments)}function n_(e){console.log(r(e))}function t_(e,n){r(e).width=n>>>0}function r_(e,n){r(e).height=n>>>0}function __(){return B(function(e,n,t){const _=r(e).getContext($(n,t));return j(_)?0:c(_)},arguments)}function o_(e,n,t){r(e)[G(n)]=G(t)}function c_(e,n){const t=r(e)[n>>>0];return c(t)}function a_(e){return r(e).length}function u_(){const e=new Array;return c(e)}function i_(e,n){const t=new Function($(e,n));return c(t)}function f_(){return B(function(e,n){const t=r(e).call(r(n));return c(t)},arguments)}function s_(){const e=new Object;return c(e)}function d_(){return B(function(){const e=self.self;return c(e)},arguments)}function b_(){return B(function(){const e=window.window;return c(e)},arguments)}function g_(){return B(function(){const e=globalThis.globalThis;return c(e)},arguments)}function l_(){return B(function(){const e=global.global;return c(e)},arguments)}function m_(e,n,t){r(e)[n>>>0]=G(t)}function w_(e,n){return r(e).push(r(n))}function p_(){return B(function(e,n,t){const _=r(e).call(r(n),r(t));return c(_)},arguments)}function h_(e){let n;try{n=r(e)instanceof Object}catch{n=!1}return n}function y_(e){const n=r(e).valueOf();return c(n)}function x_(e,n){try{var t={a:e,b:n},_=(a,u)=>{const s=t.a;t.a=0;try{return Je(s,t.b,a,u)}finally{t.a=s}};const o=new Promise(_);return c(o)}finally{t.a=t.b=0}}function B_(e){const n=Promise.resolve(r(e));return c(n)}function S_(e,n){const t=r(e).then(r(n));return c(t)}function v_(e,n,t){const _=r(e).then(r(n),r(t));return c(_)}function $_(e){const n=r(e).buffer;return c(n)}function k_(e,n,t){const _=new Uint8Array(r(e),n>>>0,t>>>0);return c(_)}function I_(e){const n=new Uint8Array(r(e));return c(n)}function P_(e,n,t){r(e).set(r(n),t>>>0)}function C_(e){return r(e).length}function T_(e){const n=r(e).buffer;return c(n)}function A_(){return B(function(e,n,t){return Reflect.set(r(e),r(n),r(t))},arguments)}function E_(e,n){const t=fe(r(n)),_=C(t,f.__wbindgen_malloc,f.__wbindgen_realloc),o=x;p().setInt32(e+4*1,o,!0),p().setInt32(e+4*0,_,!0)}function W_(e,n){throw new Error($(e,n))}function O_(){const e=f.memory;return c(e)}function G_(e,n,t){const _=se(e,n,571,Se);return c(_)}function D_(e,n,t){const _=se(e,n,571,Se);return c(_)}function z_(e,n,t){const _=se(e,n,578,Xe);return c(_)}URL=globalThis.URL;const h=await je({"./image_detect_mozaic_wasm_bg.js":{__wbindgen_object_drop_ref:en,__wbindgen_string_new:nn,__wbg_new_abda76e883ba8a5f:tn,__wbg_stack_658279fe44541cf6:rn,__wbg_error_f851667af71bcfc6:_n,__wbg_instanceof_GpuValidationError_05482398d349fd2d:on,__wbg_message_4bd9ef09b3092122:cn,__wbg_instanceof_GpuOutOfMemoryError_658135cd3b3f08e2:an,__wbg_type_c3e79de7c41f03c2:un,__wbg_offset_47f9a19926637c8e:fn,__wbg_length_ff62902e8840f82f:sn,__wbindgen_object_clone_ref:dn,__wbg_error_520ca6f621497012:bn,__wbindgen_cb_drop:gn,__wbg_instanceof_GpuDeviceLostInfo_c7232ceb822b15d6:ln,__wbg_reason_436ee862de561851:mn,__wbg_message_54cb97c0fd1579bf:wn,__wbindgen_string_get:pn,__wbg_has_14b751afdcf0a341:hn,__wbg_maxTextureDimension1D_71c238385d79f287:yn,__wbg_maxTextureDimension2D_ce910a0ea6c7213b:xn,__wbg_maxTextureDimension3D_76032d2a97af63ac:Bn,__wbg_maxTextureArrayLayers_b561668f7e1ebacc:Sn,__wbg_maxBindGroups_d2b688642140a1bb:vn,__wbg_maxBindingsPerBindGroup_a3e9e404dd893c83:$n,__wbg_maxDynamicUniformBuffersPerPipelineLayout_98a8fbca367148bf:kn,__wbg_maxDynamicStorageBuffersPerPipelineLayout_0dec6aea74b472ad:In,__wbg_maxSampledTexturesPerShaderStage_7a0712465c0a12b4:Pn,__wbg_maxSamplersPerShaderStage_6716e9792fc2a833:Cn,__wbg_maxStorageBuffersPerShaderStage_640d34738978a4ff:Tn,__wbg_maxStorageTexturesPerShaderStage_6614a1e40f7e2827:An,__wbg_maxUniformBuffersPerShaderStage_1ff2f3c6468374ae:En,__wbg_maxUniformBufferBindingSize_8830a8df4f730637:Wn,__wbg_maxStorageBufferBindingSize_10b6eb49372335bc:On,__wbg_maxVertexBuffers_9f97f2a89863a431:Gn,__wbg_maxBufferSize_1c8b836056558ebf:Dn,__wbg_maxVertexAttributes_cff466bbace9aa7c:zn,__wbg_maxVertexBufferArrayStride_fb650714a5bd0e68:Ln,__wbg_minUniformBufferOffsetAlignment_0168a0d08b19afbe:Mn,__wbg_minStorageBufferOffsetAlignment_3b63a59f37f275f8:Rn,__wbg_maxInterStageShaderComponents_db659eaa3b248a74:Fn,__wbg_maxColorAttachments_e821b856b5cba24e:Un,__wbg_maxColorAttachmentBytesPerSample_ab770042dd82a5bf:jn,__wbg_maxComputeWorkgroupStorageSize_e6773eb1cbfa7a83:qn,__wbg_maxComputeInvocationsPerWorkgroup_4ed447998b195973:Vn,__wbg_maxComputeWorkgroupSizeX_de94f4925b26c73c:Nn,__wbg_maxComputeWorkgroupSizeY_cb75de6b450c8915:Hn,__wbg_maxComputeWorkgroupSizeZ_6277d18773d70891:Qn,__wbg_maxComputeWorkgroupsPerDimension_baef21641827881d:Xn,__wbg_instanceof_GpuAdapter_ba82c448cfa55608:Yn,__wbg_queue_e124eaca54d285d4:Zn,__wbindgen_is_object:Jn,__wbg_instanceof_GpuCanvasContext_1eacd2a8c6b36ada:Kn,__wbg_getMappedRange_08e71df297c66a50:et,__wbg_Window_4d1f8d969d639a22:nt,__wbindgen_is_undefined:tt,__wbg_WorkerGlobalScope_c4f12290f7d2efed:rt,__wbg_gpu_7d756a02ad45027d:_t,__wbg_requestAdapter_8413757c51a35b1d:ot,__wbindgen_number_new:ct,__wbg_requestDevice_1c8e4f0fe8729328:at,__wbg_features_e7f12cb6c5258238:ut,__wbg_limits_622a6ae19a037dbf:it,__wbg_getPreferredCanvasFormat_d55bc32b5a6b948a:ft,__wbg_configure_48cfbf148a9998c2:st,__wbg_getCurrentTexture_1c8e29bec577927d:dt,__wbg_features_b1971639ec1a77f7:bt,__wbg_limits_e806d307d42a9dde:gt,__wbg_createShaderModule_cda89eb5c1073627:lt,__wbg_createBindGroupLayout_4243a95be946d48a:mt,__wbg_createBindGroup_f93afa3a0a06b10e:wt,__wbg_createPipelineLayout_bcb406883550f9cc:pt,__wbg_createRenderPipeline_7ca396c186d8d06a:ht,__wbg_createComputePipeline_fb60500f9a96e290:yt,__wbg_createBuffer_44406243485760b1:xt,__wbg_createTexture_06106f81b60e5462:Bt,__wbg_createSampler_ed81ff565caa903a:St,__wbg_createQuerySet_4040f9ea5a2ac03c:vt,__wbg_createCommandEncoder_c7eddb5143f91992:$t,__wbg_createRenderBundleEncoder_d9644450ab4cad8f:kt,__wbg_destroy_2a8c41712abac4cb:It,__wbg_lost_02e8ddfb37103cc2:Pt,__wbg_setonuncapturederror_c702acc9eeeb9613:Ct,__wbg_pushErrorScope_3dc565fa86fee870:Tt,__wbg_popErrorScope_6d6b4abc95412374:At,__wbg_mapAsync_98ce4986e2f6d4af:Et,__wbg_unmap_efca7885e5daff78:Wt,__wbg_getCompilationInfo_adcb4d74ed54d1f9:Ot,__wbg_messages_6833dfd0ae6a0a7c:Gt,__wbg_createView_87e589e1574ba76c:Dt,__wbg_destroy_387cb19081689594:zt,__wbg_destroy_b040948312c539a9:Lt,__wbg_getBindGroupLayout_0194b7a790ac805d:Mt,__wbg_getBindGroupLayout_1490d5a61f4fd56b:Rt,__wbg_copyBufferToBuffer_f0736fef84f76be5:Ft,__wbg_copyBufferToTexture_aedde01ad3786b4f:Ut,__wbg_copyTextureToBuffer_268207d3e09dfa81:jt,__wbg_copyTextureToTexture_7ea3d6de0a82ce7f:qt,__wbg_beginComputePass_df50d9ddd5f32a63:Vt,__wbg_beginRenderPass_14284a54cee2063b:Nt,__wbg_label_81cb6c4ebcba5f4d:Ht,__wbg_finish_78696a2f194fbb7a:Qt,__wbg_finish_7ad9d3e23124bbc6:Xt,__wbg_clearBuffer_a5ccb106665ad51e:Yt,__wbg_clearBuffer_f06a69a0aa134d24:Zt,__wbg_resolveQuerySet_7354946ea63dacbb:Jt,__wbg_finish_5be91110098e071c:Kt,__wbg_finish_667443ed0047f53a:er,__wbg_writeBuffer_6ce87bc6ff22a2b5:nr,__wbg_usage_5043ac06189fbe53:tr,__wbg_size_61d4fa05868b79cd:rr,__wbg_writeTexture_3708ced0dd386721:_r,__wbg_copyExternalImageToTexture_e192d56d70996ad4:or,__wbg_setPipeline_4d0e04e7370f0e2e:cr,__wbg_setBindGroup_48300d51a3d74853:ar,__wbg_setBindGroup_d79f4f1d5e43c06f:ur,__wbg_dispatchWorkgroups_f0fd90dcd4a506fa:ir,__wbg_dispatchWorkgroupsIndirect_567a84763f6a0b87:fr,__wbg_end_bbe499813ce72830:sr,__wbg_setPipeline_6174c2e8900fe24a:dr,__wbg_setBindGroup_de4812744c6ebb6c:br,__wbg_setBindGroup_92581920e209bf52:gr,__wbg_setIndexBuffer_91b6f5eb1a43df9b:lr,__wbg_setIndexBuffer_5bce79843be8653d:mr,__wbg_setVertexBuffer_d9b48c3489dcfa22:wr,__wbg_setVertexBuffer_330ab505b9dfc64b:pr,__wbg_draw_29abcb466fee48b4:hr,__wbg_drawIndexed_34b06707991ddaf7:yr,__wbg_drawIndirect_0054fe754e8e46e9:xr,__wbg_drawIndexedIndirect_4b7b51fa979657ca:Br,__wbg_setPipeline_8f2f5c316ddb7f68:Sr,__wbg_setBindGroup_da48569994113ec3:vr,__wbg_setBindGroup_1c3dd07b998fa943:$r,__wbg_setIndexBuffer_1dc175abfd5d9be9:kr,__wbg_setIndexBuffer_a0fcb26f210351b7:Ir,__wbg_setVertexBuffer_c347f9618d3f056a:Pr,__wbg_setVertexBuffer_40da6368898587db:Cr,__wbg_draw_a3e2be7a25d4af68:Tr,__wbg_drawIndexed_f219cccc74b869c5:Ar,__wbg_drawIndirect_23fc0a72c5f1b993:Er,__wbg_drawIndexedIndirect_6839c0505e2eed2e:Wr,__wbg_setBlendConstant_fd172910ef2cc0c8:Or,__wbg_setScissorRect_915b4534e3936f28:Gr,__wbg_setViewport_aff318ede051c64e:Dr,__wbg_setStencilReference_e2bb05496423e92e:zr,__wbg_executeBundles_0f6b9b3accb5b6a7:Lr,__wbg_end_c97b7dbccda72e72:Mr,__wbg_submit_4283b63806c5d15e:Rr,__wbg_lineNum_06a4c70c1027df81:Fr,__wbg_message_0ff806941d54e1d2:Ur,__wbg_queueMicrotask_48421b3cc9052b68:jr,__wbindgen_is_function:qr,__wbg_queueMicrotask_12a30234db4045d3:Vr,__wbg_instanceof_Window_5012736c80a01584:Nr,__wbg_document_8554450897a855b9:Hr,__wbg_navigator_6210380287bf8581:Qr,__wbg_querySelectorAll_52447cbab6df8bae:Xr,__wbg_navigator_db73b5b11a0c5c93:Yr,__wbg_get_fe289e3950b3978a:Zr,__wbg_setwidth_c20f1f8fcd5d93b4:Jr,__wbg_setheight_a5e39c9d97429299:Kr,__wbg_getContext_bd2ece8a59fd4732:e_,__wbg_log_b103404cc5920657:n_,__wbg_setwidth_e371a8d6b16ebe84:t_,__wbg_setheight_ba99ad2df4295e89:r_,__wbg_getContext_69ec873410cbba3c:__,__wbg_set_f975102236d3c502:o_,__wbg_get_3baa728f9d58d3f6:c_,__wbg_length_ae22078168b726f5:a_,__wbg_new_a220cf903aa02ca2:u_,__wbg_newnoargs_76313bd6ff35d0f2:i_,__wbg_call_1084a111329e68ce:f_,__wbg_new_525245e2b9901204:s_,__wbg_self_3093d5d1f7bcb682:d_,__wbg_window_3bcfc4d31bc012f8:b_,__wbg_globalThis_86b222e13bdf32ed:g_,__wbg_global_e5a3fe56f8be9485:l_,__wbg_set_673dda6c73d19609:m_,__wbg_push_37c89022f34c01ca:w_,__wbg_call_89af060b4e1523f2:p_,__wbg_instanceof_Object_b80213ae6cc9aafb:h_,__wbg_valueOf_d5ba0a54a2aa5615:y_,__wbg_new_b85e72ed1bfd57f9:x_,__wbg_resolve_570458cb99d56a43:B_,__wbg_then_95e6edc0f89b73b1:S_,__wbg_then_876bb3c633745cc6:v_,__wbg_buffer_b7b08af79b0b0974:$_,__wbg_newwithbyteoffsetandlength_8a2cb9ca96b27ec9:k_,__wbg_new_ea1883e1e5e86686:I_,__wbg_set_d1e79e2388520f18:P_,__wbg_length_8339fcf5d8ecd12e:C_,__wbg_buffer_0710d1b9dbe2eea6:T_,__wbg_set_eacc7d73fefaafdf:A_,__wbindgen_debug_string:E_,__wbindgen_throw:W_,__wbindgen_memory:O_,__wbindgen_closure_wrapper1326:G_,__wbindgen_closure_wrapper1328:D_,__wbindgen_closure_wrapper1803:z_}},Ue),L_=h.memory,M_=h.__wbg_imagedetector_free,R_=h.imagedetector_new,F_=h.imagedetector_increment,U_=h.imagedetector_decrement,j_=h.imagedetector_detect_image_and_mozaic,q_=h.__wbindgen_malloc,V_=h.__wbindgen_realloc,N_=h.__wbindgen_export_2,H_=h._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h187715d88a0ecad1,Q_=h._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h0960554546eefafc,X_=h.__wbindgen_free,Y_=h.__wbindgen_exn_store,Z_=h.wasm_bindgen__convert__closures__invoke2_mut__h8b05459977c113ac,J_=Object.freeze(Object.defineProperty({__proto__:null,__wbg_imagedetector_free:M_,__wbindgen_exn_store:Y_,__wbindgen_export_2:N_,__wbindgen_free:X_,__wbindgen_malloc:q_,__wbindgen_realloc:V_,_dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h0960554546eefafc:Q_,_dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h187715d88a0ecad1:H_,imagedetector_decrement:U_,imagedetector_detect_image_and_mozaic:j_,imagedetector_increment:F_,imagedetector_new:R_,memory:L_,wasm_bindgen__convert__closures__invoke2_mut__h8b05459977c113ac:Z_},Symbol.toStringTag,{value:"Module"}));qe(J_);const $e=(e,n)=>{const t=e.toDataURL("image/png"),_=document.createElement("a");_.href=t,_.download=n,_.click()},K_=(e,n)=>{var _;if(!e||!n)return;const t=document.createElement("canvas");t.width=n.videoWidth,t.height=n.videoHeight,(_=t.getContext("2d"))==null||_.drawImage(n,0,0,n.videoWidth,n.videoHeight),$e(t,"original_image.png")},eo=(e,n)=>{var _;if(!e||!n)return;const t=document.createElement("canvas");t.width=n.width,t.height=n.height,(_=t.getContext("2d"))==null||_.drawImage(n,0,0),$e(t,"edge_detected_image.png")};function no(e){let n;return{c(){n=q("div"),n.innerHTML="<h3>Help</h3> <p>Full screen by `Escape` key</p> <p>Full screen for effect window by `w` key</p> <p>Save effect image by `s` key</p> <p>Save web camera image by `o` key</p> <p>Show Help window by `h` key</p>",ne(n,"class","help-overlay svelte-j8kng8")},m(t,_){I(t,n,_)},p:m,i:m,o:m,d(t){t&&v(n)}}}class to extends ie{constructor(n){super(),ue(this,n,null,no,ee,{})}}function ke(e){let n,t;return n=new to({}),{c(){ye(n.$$.fragment)},m(_,o){ce(n,_,o),t=!0},i(_){t||(O(n.$$.fragment,_),t=!0)},o(_){H(n.$$.fragment,_),t=!1},d(_){ae(n,_)}}}function ro(e){let n,t,_,o,a,u,s,g,i=e[1]&&ke();return{c(){n=q("video"),t=we(),_=q("div"),o=V(e[2]),a=V(" FPS"),u=we(),i&&i.c(),s=Ce(),ne(n,"id","video-off"),n.autoplay=!0,n.playsInline=!0,ne(_,"id","fps"),T(_,"position","absolute"),T(_,"bottom","10px"),T(_,"left","10px"),T(_,"color","white"),T(_,"background-color","rgba(0, 0, 0, 0.7)"),T(_,"padding","5px")},m(b,d){I(b,n,d),e[3](n),I(b,t,d),I(b,_,d),me(_,o),me(_,a),I(b,u,d),i&&i.m(b,d),I(b,s,d),g=!0},p(b,[d]){(!g||d&4)&&Ae(o,b[2]),b[1]?i?d&2&&O(i,1):(i=ke(),i.c(),O(i,1),i.m(s.parentNode,s)):i&&(Le(),H(i,1,1,()=>{i=null}),Me())},i(b){g||(O(i),g=!0)},o(b){H(i),g=!1},d(b){b&&(v(n),v(t),v(_),v(u),v(s)),e[3](null),i&&i.d(b)}}}const _o=50,oo=4e3,co=!0;function ao(e,n,t){let _,o,a,u,s,g=!1,i=!1,b=0,d=0;const Z=Math.random()<.5?1:2,J=new Ke(Z,_o,oo,co),K=async()=>{try{const l=await navigator.mediaDevices.getUserMedia({video:!0});t(0,_.srcObject=l,_),await _.play(),g=!0}catch(l){console.error("Failed to setup video stream:",l)}},so=()=>{o=document.createElement("canvas"),o.width=_.videoWidth,o.height=_.videoHeight,a=o.getContext("2d")},bo=()=>{u=document.createElement("canvas"),document.getElementById("app").appendChild(u),u.width=_.videoWidth,u.height=_.videoHeight,u.style.display="block",u.style.width="100%",u.style.height="100%",u.style.objectFit="contain",u.style.objectPosition="center",s=u.getContext("2d")},go=(l,z)=>{document.addEventListener("keydown",y=>{y.key==="Escape"?lo():y.key==="s"||y.key==="S"?(console.log("S: saveOutputImage"),eo(l,z)):y.key==="o"||y.key==="O"?(console.log("O: saveOriginalImage"),K_(l,_)):y.key==="h"||y.key==="H"?t(1,i=!i):(y.key==="w"||y.key==="W")&&mo(z)})},Ie=async l=>{try{if(!g||!a)return;if(d){const yo=(l-d)/1e3;t(2,b=Math.round(1/yo))}d=l,a.drawImage(_,0,0,o.width,o.height);const z=a.getImageData(0,0,o.width,o.height);console.log("call imageDetector.detect_image_and_mozaic()");const y=await J.detect_image_and_mozaic(new Uint8Array(z.data.buffer),o.width,o.height);if(y instanceof Error)throw y;const po=y.raw_data,ho=new ImageData(new Uint8ClampedArray(po),u.width,u.height);s.putImageData(ho,0,0),requestAnimationFrame(Ie)}catch(z){console.error("Error processing frame:",z)}},lo=()=>{document.fullscreenElement?document.exitFullscreen&&document.exitFullscreen():document.documentElement.requestFullscreen()},mo=l=>{l.requestFullscreen&&l.requestFullscreen()};We(async()=>{try{await K(),so(),bo(),go(a,o);const l=Math.floor(new Date().getTime()/1e3);Ie(l),t(1,i=!0)}catch(l){console.error("Error during onMount:",l)}});function wo(l){te[l?"unshift":"push"](()=>{_=l,t(0,_)})}return[_,i,b,wo]}class uo extends ie{constructor(n){super(),ue(this,n,ao,ro,ee,{})}}function io(e){let n,t,_;return t=new uo({}),{c(){n=q("main"),ye(t.$$.fragment)},m(o,a){I(o,n,a),ce(t,n,null),_=!0},p:m,i(o){_||(O(t.$$.fragment,o),_=!0)},o(o){H(t.$$.fragment,o),_=!1},d(o){o&&v(n),ae(t)}}}class fo extends ie{constructor(n){super(),ue(this,n,null,io,ee,{})}}new fo({target:document.getElementById("app")})})()});export default vo();
