let wasm_bindgen;(()=>{let _=`bigint`,Y=`Object`,L=null,a1=4,a0=8,K=`undefined`,Q=`utf-8`,P=0,O=1,V=`boolean`,W=`string`,T=`function`,U=`number`,N=Array,X=Array.isArray,$=BigInt,R=Error,Z=FinalizationRegistry,a2=Object,S=Uint8Array,M=undefined;function D(a,b){try{return a.apply(this,b)}catch(a){c.__wbindgen_exn_store(i(a))}}var C=((a,b)=>{if(a===P){return e(b)}else{return r(a,b)}});var y=(a=>{const b=typeof a;if(b==U||b==V||a==L){return `${a}`};if(b==W){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==L){return `Symbol`}else{return `Symbol(${b})`}};if(b==T){const b=a.name;if(typeof b==W&&b.length>P){return `Function(${b})`}else{return `Function`}};if(X(a)){const b=a.length;let c=`[`;if(b>P){c+=y(a[P])};for(let d=O;d<b;d++){c+=`, `+ y(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>O){d=c[O]}else{return toString.call(a)};if(d==Y){try{return `Object(`+ JSON.stringify(a)+ `)`}catch(a){return Y}};if(a instanceof R){return `${a.name}: ${a.message}\n${a.stack}`};return d});var r=((a,b)=>{a=a>>>P;return o.decode(q().subarray(a,a+ b))});var B=((a,b,d)=>{c.wasm_bindgen__convert__closures__invoke1__h9689a81c4d30f8e7(a,b,i(d))});var j=(a=>a===M||a===L);var I=(a=>{if(c!==M)return c;const b=F();G(b);if(!(a instanceof WebAssembly.Module)){a=new WebAssembly.Module(a)};const d=new WebAssembly.Instance(a,b);return H(d,a)});var h=(a=>{const b=e(a);g(a);return b});var x=(()=>{if(w===L||w.byteLength===P){w=new BigInt64Array(c.memory.buffer)};return w});var A=((a,b,d,e)=>{const f={a:a,b:b,cnt:O,dtor:d};const g=(...a)=>{f.cnt++;try{return e(f.a,f.b,...a)}finally{if(--f.cnt===P){c.__wbindgen_export_2.get(f.dtor)(f.a,f.b);f.a=P;z.unregister(f)}}};g.original=f;z.register(g,f,f);return g});var F=(()=>{const a={};a.wbg={};a.wbg.__wbindgen_object_drop_ref=(a=>{h(a)});a.wbg.__wbindgen_is_object=(a=>{const b=e(a);const c=typeof b===`object`&&b!==L;return c});a.wbg.__wbindgen_is_undefined=(a=>{const b=e(a)===M;return b});a.wbg.__wbindgen_in=((a,b)=>{const c=e(a) in e(b);return c});a.wbg.__wbindgen_is_bigint=(a=>{const b=typeof e(a)===_;return b});a.wbg.__wbindgen_bigint_from_u64=(a=>{const b=$.asUintN(64,a);return i(b)});a.wbg.__wbindgen_jsval_eq=((a,b)=>{const c=e(a)===e(b);return c});a.wbg.__wbindgen_number_get=((a,b)=>{const c=e(b);const d=typeof c===U?c:M;l()[a/a0+ O]=j(d)?P:d;n()[a/a1+ P]=!j(d)});a.wbg.__wbindgen_object_clone_ref=(a=>{const b=e(a);return i(b)});a.wbg.__wbindgen_error_new=((a,b)=>{const c=new R(r(a,b));return i(c)});a.wbg.__wbindgen_jsval_loose_eq=((a,b)=>{const c=e(a)==e(b);return c});a.wbg.__wbindgen_boolean_get=(a=>{const b=e(a);const c=typeof b===V?(b?O:P):2;return c});a.wbg.__wbindgen_string_get=((a,b)=>{const d=e(b);const f=typeof d===W?d:M;var g=j(f)?P:v(f,c.__wbindgen_malloc,c.__wbindgen_realloc);var h=s;n()[a/a1+ O]=h;n()[a/a1+ P]=g});a.wbg.__wbindgen_as_number=(a=>{const b=+e(a);return b});a.wbg.__wbindgen_number_new=(a=>{const b=a;return i(b)});a.wbg.__wbindgen_string_new=((a,b)=>{const c=r(a,b);return i(c)});a.wbg.__wbg_getwithrefkey_edc2c8960f0f1191=((a,b)=>{const c=e(a)[e(b)];return i(c)});a.wbg.__wbg_set_f975102236d3c502=((a,b,c)=>{e(a)[h(b)]=h(c)});a.wbg.__wbg_new_abda76e883ba8a5f=(()=>{const a=new R();return i(a)});a.wbg.__wbg_stack_658279fe44541cf6=((a,b)=>{const d=e(b).stack;const f=v(d,c.__wbindgen_malloc,c.__wbindgen_realloc);const g=s;n()[a/a1+ O]=g;n()[a/a1+ P]=f});a.wbg.__wbg_error_f851667af71bcfc6=((a,b)=>{var d=C(a,b);if(a!==P){c.__wbindgen_free(a,b,O)};console.error(d)});a.wbg.__wbg_crypto_566d7465cdbb6b7a=(a=>{const b=e(a).crypto;return i(b)});a.wbg.__wbg_process_dc09a8c7d59982f6=(a=>{const b=e(a).process;return i(b)});a.wbg.__wbg_versions_d98c6400c6ca2bd8=(a=>{const b=e(a).versions;return i(b)});a.wbg.__wbg_node_caaf83d002149bd5=(a=>{const b=e(a).node;return i(b)});a.wbg.__wbindgen_is_string=(a=>{const b=typeof e(a)===W;return b});a.wbg.__wbg_msCrypto_0b84745e9245cdf6=(a=>{const b=e(a).msCrypto;return i(b)});a.wbg.__wbg_require_94a9da52636aacbf=function(){return D((()=>{const a=module.require;return i(a)}),arguments)};a.wbg.__wbindgen_is_function=(a=>{const b=typeof e(a)===T;return b});a.wbg.__wbg_randomFillSync_290977693942bf03=function(){return D(((a,b)=>{e(a).randomFillSync(h(b))}),arguments)};a.wbg.__wbg_getRandomValues_260cc23a41afad9a=function(){return D(((a,b)=>{e(a).getRandomValues(e(b))}),arguments)};a.wbg.__wbg_setonmessage_bf8a4436ccd4af19=((a,b)=>{e(a).onmessage=e(b)});a.wbg.__wbg_postMessage_34005f67a84600d1=function(){return D(((a,b)=>{e(a).postMessage(e(b))}),arguments)};a.wbg.__wbg_data_3ce7c145ca4fbcdc=(a=>{const b=e(a).data;return i(b)});a.wbg.__wbg_log_5bb5f88f245d7762=(a=>{console.log(e(a))});a.wbg.__wbg_get_bd8e338fbd5f5cc8=((a,b)=>{const c=e(a)[b>>>P];return i(c)});a.wbg.__wbg_length_cd7af8117672b8b8=(a=>{const b=e(a).length;return b});a.wbg.__wbg_new_16b304a2cfa7ff4a=(()=>{const a=new N();return i(a)});a.wbg.__wbg_newnoargs_e258087cd0daa0ea=((a,b)=>{var c=C(a,b);const d=new Function(c);return i(d)});a.wbg.__wbg_next_40fc327bfc8770e6=(a=>{const b=e(a).next;return i(b)});a.wbg.__wbg_next_196c84450b364254=function(){return D((a=>{const b=e(a).next();return i(b)}),arguments)};a.wbg.__wbg_done_298b57d23c0fc80c=(a=>{const b=e(a).done;return b});a.wbg.__wbg_value_d93c65011f51a456=(a=>{const b=e(a).value;return i(b)});a.wbg.__wbg_iterator_2cee6dadfd956dfa=(()=>{const a=Symbol.iterator;return i(a)});a.wbg.__wbg_get_e3c254076557e348=function(){return D(((a,b)=>{const c=Reflect.get(e(a),e(b));return i(c)}),arguments)};a.wbg.__wbg_call_27c0f87801dedf93=function(){return D(((a,b)=>{const c=e(a).call(e(b));return i(c)}),arguments)};a.wbg.__wbg_new_72fb9a18b5ae2624=(()=>{const a=new a2();return i(a)});a.wbg.__wbg_self_ce0dbfc45cf2f5be=function(){return D((()=>{const a=self.self;return i(a)}),arguments)};a.wbg.__wbg_window_c6fb939a7f436783=function(){return D((()=>{const a=window.window;return i(a)}),arguments)};a.wbg.__wbg_globalThis_d1e6af4856ba331b=function(){return D((()=>{const a=globalThis.globalThis;return i(a)}),arguments)};a.wbg.__wbg_global_207b558942527489=function(){return D((()=>{const a=global.global;return i(a)}),arguments)};a.wbg.__wbg_set_d4638f722068f043=((a,b,c)=>{e(a)[b>>>P]=h(c)});a.wbg.__wbg_isArray_2ab64d95e09ea0ae=(a=>{const b=X(e(a));return b});a.wbg.__wbg_instanceof_ArrayBuffer_836825be07d4c9d2=(a=>{let b;try{b=e(a) instanceof ArrayBuffer}catch(a){b=!1}const c=b;return c});a.wbg.__wbg_call_b3ca7c6051f9bec1=function(){return D(((a,b,c)=>{const d=e(a).call(e(b),e(c));return i(d)}),arguments)};a.wbg.__wbg_isSafeInteger_f7b04ef02296c4d2=(a=>{const b=Number.isSafeInteger(e(a));return b});a.wbg.__wbg_buffer_12d079cc21e14bdb=(a=>{const b=e(a).buffer;return i(b)});a.wbg.__wbg_newwithbyteoffsetandlength_aa4a17c33a06e5cb=((a,b,c)=>{const d=new S(e(a),b>>>P,c>>>P);return i(d)});a.wbg.__wbg_new_63b92bc8671ed464=(a=>{const b=new S(e(a));return i(b)});a.wbg.__wbg_set_a47bac70306a19a7=((a,b,c)=>{e(a).set(e(b),c>>>P)});a.wbg.__wbg_length_c20a40f15020d68a=(a=>{const b=e(a).length;return b});a.wbg.__wbg_instanceof_Uint8Array_2b3bbecd033d19f6=(a=>{let b;try{b=e(a) instanceof S}catch(a){b=!1}const c=b;return c});a.wbg.__wbg_newwithlength_e9b4878cebadb3d3=(a=>{const b=new S(a>>>P);return i(b)});a.wbg.__wbg_subarray_a1f73cd4b5b42fe1=((a,b,c)=>{const d=e(a).subarray(b>>>P,c>>>P);return i(d)});a.wbg.__wbindgen_bigint_get_as_i64=((a,b)=>{const c=e(b);const d=typeof c===_?c:M;x()[a/a0+ O]=j(d)?$(P):d;n()[a/a1+ P]=!j(d)});a.wbg.__wbindgen_debug_string=((a,b)=>{const d=y(e(b));const f=v(d,c.__wbindgen_malloc,c.__wbindgen_realloc);const g=s;n()[a/a1+ O]=g;n()[a/a1+ P]=f});a.wbg.__wbindgen_throw=((a,b)=>{throw new R(r(a,b))});a.wbg.__wbindgen_memory=(()=>{const a=c.memory;return i(a)});a.wbg.__wbindgen_closure_wrapper103=((a,b,c)=>{const d=A(a,b,19,B);return i(d)});return a});var E=(async(a,b)=>{if(typeof Response===T&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===T){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var i=(a=>{if(f===d.length)d.push(d.length+ O);const b=f;f=d[b];d[b]=a;return b});var J=(async(a)=>{if(c!==M)return c;if(typeof a===K&&typeof b!==K){a=b.replace(/\.js$/,`_bg.wasm`)};const d=F();if(typeof a===W||typeof Request===T&&a instanceof Request||typeof URL===T&&a instanceof URL){a=fetch(a)};G(d);const {instance:e,module:f}=await E(await a,d);return H(e,f)});var v=((a,b,c)=>{if(c===M){const c=t.encode(a);const d=b(c.length,O)>>>P;q().subarray(d,d+ c.length).set(c);s=c.length;return d};let d=a.length;let e=b(d,O)>>>P;const f=q();let g=P;for(;g<d;g++){const b=a.charCodeAt(g);if(b>127)break;f[e+ g]=b};if(g!==d){if(g!==P){a=a.slice(g)};e=c(e,d,d=g+ a.length*3,O)>>>P;const b=q().subarray(e+ g,e+ d);const f=u(a,b);g+=f.written;e=c(e,d,g,O)>>>P};s=g;return e});var l=(()=>{if(k===L||k.byteLength===P){k=new Float64Array(c.memory.buffer)};return k});var g=(a=>{if(a<132)return;d[a]=f;f=a});var G=((a,b)=>{});var q=(()=>{if(p===L||p.byteLength===P){p=new S(c.memory.buffer)};return p});var e=(a=>d[a]);var n=(()=>{if(m===L||m.byteLength===P){m=new Int32Array(c.memory.buffer)};return m});var H=((a,b)=>{c=a.exports;J.__wbindgen_wasm_module=b;w=L;k=L;m=L;p=L;c.__wbindgen_start();return c});const a={};let b;if(typeof document!==K&&document.currentScript!==L){b=new URL(document.currentScript.src,location.href).toString()};let c=M;const d=new N(128).fill(M);d.push(M,L,!0,!1);let f=d.length;let k=L;let m=L;const o=typeof TextDecoder!==K?new TextDecoder(Q,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw R(`TextDecoder not available`)}};if(typeof TextDecoder!==K){o.decode()};let p=L;let s=P;const t=typeof TextEncoder!==K?new TextEncoder(Q):{encode:()=>{throw R(`TextEncoder not available`)}};const u=typeof t.encodeInto===T?((a,b)=>t.encodeInto(a,b)):((a,b)=>{const c=t.encode(a);b.set(c);return {read:a.length,written:c.length}});let w=L;const z=typeof Z===K?{register:()=>{},unregister:()=>{}}:new Z(a=>{c.__wbindgen_export_2.get(a.dtor)(a.a,a.b)});wasm_bindgen=a2.assign(J,{initSync:I},a)})()