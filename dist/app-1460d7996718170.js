let a4=`bigint`,$=`Object`,P=null,a3=4,a2=8,T=`utf-8`,S=`undefined`,a8=77,Z=`string`,Q=0,W=1,Y=`boolean`,V=`function`,X=`number`,N=Array,_=Array.isArray,a5=BigInt,U=Error,a1=FinalizationRegistry,a0=JSON.stringify,a7=Object,a6=Reflect,R=Uint8Array,O=undefined;var A=((b,c,d,e)=>{const f={a:b,b:c,cnt:W,dtor:d};const g=(...b)=>{f.cnt++;const c=f.a;f.a=Q;try{return e(c,f.b,...b)}finally{if(--f.cnt===Q){a.__wbindgen_export_2.get(f.dtor)(c,f.b);x.unregister(f)}else{f.a=c}}};g.original=f;x.register(g,f,f);return g});function G(b,c){try{return b.apply(this,c)}catch(b){a.__wbindgen_exn_store(r(b))}}var w=(a=>{const b=typeof a;if(b==X||b==Y||a==P){return `${a}`};if(b==Z){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==P){return `Symbol`}else{return `Symbol(${b})`}};if(b==V){const b=a.name;if(typeof b==Z&&b.length>Q){return `Function(${b})`}else{return `Function`}};if(_(a)){const b=a.length;let c=`[`;if(b>Q){c+=w(a[Q])};for(let d=W;d<b;d++){c+=`, `+ w(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>W){d=c[W]}else{return toString.call(a)};if(d==$){try{return `Object(`+ a0(a)+ `)`}catch(a){return $}};if(a instanceof U){return `${a.name}: ${a.message}\n${a.stack}`};return d});var F=((a,b)=>{if(a===Q){return c(b)}else{return t(a,b)}});var t=((a,b)=>{a=a>>>Q;return s.decode(n().subarray(a,a+ b))});var d=(a=>a===O||a===P);var L=(b=>{if(a!==O)return a;const c=I();J(c);if(!(b instanceof WebAssembly.Module)){b=new WebAssembly.Module(b)};const d=new WebAssembly.Instance(b,c);return K(d,b)});var E=((b,c,d)=>{a._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h8712d875dd9def09(b,c,r(d))});var k=(a=>{const b=c(a);j(a);return b});var v=(()=>{if(u===P||u.byteLength===Q){u=new BigInt64Array(a.memory.buffer)};return u});var D=((b,c,d)=>{a.wasm_bindgen__convert__closures__invoke1_mut__hb13ebf7135e4b988(b,c,r(d))});var y=((b,c,d,e)=>{const f={a:b,b:c,cnt:W,dtor:d};const g=(...b)=>{f.cnt++;try{return e(f.a,f.b,...b)}finally{if(--f.cnt===Q){a.__wbindgen_export_2.get(f.dtor)(f.a,f.b);f.a=Q;x.unregister(f)}}};g.original=f;x.register(g,f,f);return g});var M=(async(b)=>{if(a!==O)return a;if(typeof b===S){b=new URL(`app-1460d7996718170_bg.wasm`,import.meta.url)};const c=I();if(typeof b===Z||typeof Request===V&&b instanceof Request||typeof URL===V&&b instanceof URL){b=fetch(b)};J(c);const {instance:d,module:e}=await H(await b,c);return K(d,e)});var I=(()=>{const b={};b.wbg={};b.wbg.__wbindgen_number_get=((a,b)=>{const e=c(b);const g=typeof e===X?e:O;f()[a/a2+ W]=d(g)?Q:g;h()[a/a3+ Q]=!d(g)});b.wbg.__wbindgen_object_drop_ref=(a=>{k(a)});b.wbg.__wbindgen_string_get=((b,e)=>{const f=c(e);const g=typeof f===Z?f:O;var i=d(g)?Q:q(g,a.__wbindgen_malloc,a.__wbindgen_realloc);var j=l;h()[b/a3+ W]=j;h()[b/a3+ Q]=i});b.wbg.__wbindgen_is_undefined=(a=>{const b=c(a)===O;return b});b.wbg.__wbindgen_in=((a,b)=>{const d=c(a) in c(b);return d});b.wbg.__wbindgen_is_bigint=(a=>{const b=typeof c(a)===a4;return b});b.wbg.__wbindgen_bigint_from_u64=(a=>{const b=a5.asUintN(64,a);return r(b)});b.wbg.__wbindgen_jsval_eq=((a,b)=>{const d=c(a)===c(b);return d});b.wbg.__wbindgen_error_new=((a,b)=>{const c=new U(t(a,b));return r(c)});b.wbg.__wbindgen_is_string=(a=>{const b=typeof c(a)===Z;return b});b.wbg.__wbindgen_is_object=(a=>{const b=c(a);const d=typeof b===`object`&&b!==P;return d});b.wbg.__wbindgen_object_clone_ref=(a=>{const b=c(a);return r(b)});b.wbg.__wbindgen_cb_drop=(a=>{const b=k(a).original;if(b.cnt--==W){b.a=Q;return !0};const c=!1;return c});b.wbg.__wbindgen_string_new=((a,b)=>{const c=t(a,b);return r(c)});b.wbg.__wbg_new_abda76e883ba8a5f=(()=>{const a=new U();return r(a)});b.wbg.__wbg_stack_658279fe44541cf6=((b,d)=>{const e=c(d).stack;const f=q(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;h()[b/a3+ W]=g;h()[b/a3+ Q]=f});b.wbg.__wbg_error_f851667af71bcfc6=((b,c)=>{var d=F(b,c);if(b!==Q){a.__wbindgen_free(b,c,W)};console.error(d)});b.wbg.__wbindgen_jsval_loose_eq=((a,b)=>{const d=c(a)==c(b);return d});b.wbg.__wbindgen_boolean_get=(a=>{const b=c(a);const d=typeof b===Y?(b?W:Q):2;return d});b.wbg.__wbindgen_as_number=(a=>{const b=+c(a);return b});b.wbg.__wbindgen_number_new=(a=>{const b=a;return r(b)});b.wbg.__wbg_getwithrefkey_edc2c8960f0f1191=((a,b)=>{const d=c(a)[c(b)];return r(d)});b.wbg.__wbg_set_f975102236d3c502=((a,b,d)=>{c(a)[k(b)]=k(d)});b.wbg.__wbg_fetch_1db5b0ae726d68b5=(a=>{const b=fetch(c(a));return r(b)});b.wbg.__wbg_crypto_566d7465cdbb6b7a=(a=>{const b=c(a).crypto;return r(b)});b.wbg.__wbg_process_dc09a8c7d59982f6=(a=>{const b=c(a).process;return r(b)});b.wbg.__wbg_versions_d98c6400c6ca2bd8=(a=>{const b=c(a).versions;return r(b)});b.wbg.__wbg_node_caaf83d002149bd5=(a=>{const b=c(a).node;return r(b)});b.wbg.__wbg_msCrypto_0b84745e9245cdf6=(a=>{const b=c(a).msCrypto;return r(b)});b.wbg.__wbg_require_94a9da52636aacbf=function(){return G((()=>{const a=module.require;return r(a)}),arguments)};b.wbg.__wbindgen_is_function=(a=>{const b=typeof c(a)===V;return b});b.wbg.__wbg_randomFillSync_290977693942bf03=function(){return G(((a,b)=>{c(a).randomFillSync(k(b))}),arguments)};b.wbg.__wbg_getRandomValues_260cc23a41afad9a=function(){return G(((a,b)=>{c(a).getRandomValues(c(b))}),arguments)};b.wbg.__wbindgen_is_null=(a=>{const b=c(a)===P;return b});b.wbg.__wbindgen_is_falsy=(a=>{const b=!c(a);return b});b.wbg.__wbg_instanceof_Window_f401953a2cf86220=(a=>{let b;try{b=c(a) instanceof Window}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_document_5100775d18896c16=(a=>{const b=c(a).document;return d(b)?Q:r(b)});b.wbg.__wbg_location_2951b5ee34f19221=(a=>{const b=c(a).location;return r(b)});b.wbg.__wbg_body_edb1908d3ceff3a1=(a=>{const b=c(a).body;return d(b)?Q:r(b)});b.wbg.__wbg_createComment_354ccab4fdc521ee=((a,b,d)=>{var e=F(b,d);const f=c(a).createComment(e);return r(f)});b.wbg.__wbg_createDocumentFragment_8c86903bbb0a3c3c=(a=>{const b=c(a).createDocumentFragment();return r(b)});b.wbg.__wbg_createElement_8bae7856a4bb7411=function(){return G(((a,b,d)=>{var e=F(b,d);const f=c(a).createElement(e);return r(f)}),arguments)};b.wbg.__wbg_createElementNS_556a62fb298be5a2=function(){return G(((a,b,d,e,f)=>{var g=F(b,d);var h=F(e,f);const i=c(a).createElementNS(g,h);return r(i)}),arguments)};b.wbg.__wbg_createTextNode_0c38fd80a5b2284d=((a,b,d)=>{var e=F(b,d);const f=c(a).createTextNode(e);return r(f)});b.wbg.__wbg_getElementById_c369ff43f0db99cf=((a,b,e)=>{var f=F(b,e);const g=c(a).getElementById(f);return d(g)?Q:r(g)});b.wbg.__wbg_setinnerHTML_26d69b59e1af99c7=((a,b,d)=>{var e=F(b,d);c(a).innerHTML=e});b.wbg.__wbg_removeAttribute_1b10a06ae98ebbd1=function(){return G(((a,b,d)=>{var e=F(b,d);c(a).removeAttribute(e)}),arguments)};b.wbg.__wbg_setAttribute_3c9f6c303b696daa=function(){return G(((a,b,d,e,f)=>{var g=F(b,d);var h=F(e,f);c(a).setAttribute(g,h)}),arguments)};b.wbg.__wbg_before_210596e44d88649f=function(){return G(((a,b)=>{c(a).before(c(b))}),arguments)};b.wbg.__wbg_remove_49b0a5925a04b955=(a=>{c(a).remove()});b.wbg.__wbg_append_7ba9d5c2eb183eea=function(){return G(((a,b)=>{c(a).append(c(b))}),arguments)};b.wbg.__wbg_target_2fc177e386c8b7b0=(a=>{const b=c(a).target;return d(b)?Q:r(b)});b.wbg.__wbg_cancelBubble_c0aa3172524eb03c=(a=>{const b=c(a).cancelBubble;return b});b.wbg.__wbg_composedPath_58473fd5ae55f2cd=(a=>{const b=c(a).composedPath();return r(b)});b.wbg.__wbg_preventDefault_b1a4aafc79409429=(a=>{c(a).preventDefault()});b.wbg.__wbg_instanceof_Response_849eb93e75734b6e=(a=>{let b;try{b=c(a) instanceof Response}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_url_5f6dc4009ac5f99d=((b,d)=>{const e=c(d).url;const f=q(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;h()[b/a3+ W]=g;h()[b/a3+ Q]=f});b.wbg.__wbg_status_61a01141acd3cf74=(a=>{const b=c(a).status;return b});b.wbg.__wbg_headers_9620bfada380764a=(a=>{const b=c(a).headers;return r(b)});b.wbg.__wbg_text_450a059667fd91fd=function(){return G((a=>{const b=c(a).text();return r(b)}),arguments)};b.wbg.__wbg_instanceof_ShadowRoot_9db040264422e84a=(a=>{let b;try{b=c(a) instanceof ShadowRoot}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_host_c667c7623404d6bf=(a=>{const b=c(a).host;return r(b)});b.wbg.__wbg_data_3ce7c145ca4fbcdc=(a=>{const b=c(a).data;return r(b)});b.wbg.__wbg_fetch_921fad6ef9e883dd=((a,b)=>{const d=c(a).fetch(c(b));return r(d)});b.wbg.__wbg_new_ab6fd82b10560829=function(){return G((()=>{const a=new Headers();return r(a)}),arguments)};b.wbg.__wbg_append_7bfcb4937d1d5e29=function(){return G(((a,b,d,e,f)=>{var g=F(b,d);var h=F(e,f);c(a).append(g,h)}),arguments)};b.wbg.__wbg_instanceof_HtmlInputElement_307512fe1252c849=(a=>{let b;try{b=c(a) instanceof HTMLInputElement}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_checked_749a34774f2df2e3=(a=>{const b=c(a).checked;return b});b.wbg.__wbg_valueAsNumber_b6456a4b05234115=(a=>{const b=c(a).valueAsNumber;return b});b.wbg.__wbg_addEventListener_53b787075bd5e003=function(){return G(((a,b,d,e)=>{var f=F(b,d);c(a).addEventListener(f,c(e))}),arguments)};b.wbg.__wbg_addEventListener_4283b15b4f039eb5=function(){return G(((a,b,d,e,f)=>{var g=F(b,d);c(a).addEventListener(g,c(e),c(f))}),arguments)};b.wbg.__wbg_setdata_8c2b43af041cc1b3=((a,b,d)=>{var e=F(b,d);c(a).data=e});b.wbg.__wbg_origin_ee93e29ace71f568=function(){return G(((b,d)=>{const e=c(d).origin;const f=q(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;h()[b/a3+ W]=g;h()[b/a3+ Q]=f}),arguments)};b.wbg.__wbg_createObjectURL_ad8244759309f204=function(){return G(((b,d)=>{const e=URL.createObjectURL(c(d));const f=q(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;h()[b/a3+ W]=g;h()[b/a3+ Q]=f}),arguments)};b.wbg.__wbg_log_5bb5f88f245d7762=(a=>{console.log(c(a))});b.wbg.__wbg_warn_63bbae1730aead09=(a=>{console.warn(c(a))});b.wbg.__wbg_newwithstrsequenceandoptions_ce1f1ca2d522b8aa=function(){return G(((a,b)=>{const d=new Blob(c(a),c(b));return r(d)}),arguments)};b.wbg.__wbg_newwithstrandinit_3fd6fba4083ff2d0=function(){return G(((a,b,d)=>{var e=F(a,b);const f=new Request(e,c(d));return r(f)}),arguments)};b.wbg.__wbg_parentNode_6be3abff20e1a5fb=(a=>{const b=c(a).parentNode;return d(b)?Q:r(b)});b.wbg.__wbg_childNodes_118168e8b23bcb9b=(a=>{const b=c(a).childNodes;return r(b)});b.wbg.__wbg_previousSibling_9708a091a3e6e03b=(a=>{const b=c(a).previousSibling;return d(b)?Q:r(b)});b.wbg.__wbg_nextSibling_709614fdb0fb7a66=(a=>{const b=c(a).nextSibling;return d(b)?Q:r(b)});b.wbg.__wbg_appendChild_580ccb11a660db68=function(){return G(((a,b)=>{const d=c(a).appendChild(c(b));return r(d)}),arguments)};b.wbg.__wbg_cloneNode_e19c313ea20d5d1d=function(){return G((a=>{const b=c(a).cloneNode();return r(b)}),arguments)};b.wbg.__wbg_length_d0a802565d17eec4=(a=>{const b=c(a).length;return b});b.wbg.__wbg_signal_a61f78a3478fd9bc=(a=>{const b=c(a).signal;return r(b)});b.wbg.__wbg_new_0d76b0581eca6298=function(){return G((()=>{const a=new AbortController();return r(a)}),arguments)};b.wbg.__wbg_abort_2aa7521d5690750e=(a=>{c(a).abort()});b.wbg.__wbg_setonmessage_503809e5bb51bd33=((a,b)=>{c(a).onmessage=c(b)});b.wbg.__wbg_new_d1187ae36d662ef9=function(){return G(((a,b)=>{var c=F(a,b);const d=new Worker(c);return r(d)}),arguments)};b.wbg.__wbg_postMessage_7380d10e8b8269df=function(){return G(((a,b)=>{c(a).postMessage(c(b))}),arguments)};b.wbg.__wbg_instanceof_HtmlButtonElement_534f7aa847dae46f=(a=>{let b;try{b=c(a) instanceof HTMLButtonElement}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_setdisabled_510a6aa50551bcf5=((a,b)=>{c(a).disabled=b!==Q});b.wbg.__wbg_queueMicrotask_481971b0d87f3dd4=(a=>{queueMicrotask(c(a))});b.wbg.__wbg_queueMicrotask_3cbae2ec6b6cd3d6=(a=>{const b=c(a).queueMicrotask;return r(b)});b.wbg.__wbg_get_bd8e338fbd5f5cc8=((a,b)=>{const d=c(a)[b>>>Q];return r(d)});b.wbg.__wbg_length_cd7af8117672b8b8=(a=>{const b=c(a).length;return b});b.wbg.__wbg_new_16b304a2cfa7ff4a=(()=>{const a=new N();return r(a)});b.wbg.__wbg_newnoargs_e258087cd0daa0ea=((a,b)=>{var c=F(a,b);const d=new Function(c);return r(d)});b.wbg.__wbg_next_40fc327bfc8770e6=(a=>{const b=c(a).next;return r(b)});b.wbg.__wbg_next_196c84450b364254=function(){return G((a=>{const b=c(a).next();return r(b)}),arguments)};b.wbg.__wbg_done_298b57d23c0fc80c=(a=>{const b=c(a).done;return b});b.wbg.__wbg_value_d93c65011f51a456=(a=>{const b=c(a).value;return r(b)});b.wbg.__wbg_iterator_2cee6dadfd956dfa=(()=>{const a=Symbol.iterator;return r(a)});b.wbg.__wbg_get_e3c254076557e348=function(){return G(((a,b)=>{const d=a6.get(c(a),c(b));return r(d)}),arguments)};b.wbg.__wbg_call_27c0f87801dedf93=function(){return G(((a,b)=>{const d=c(a).call(c(b));return r(d)}),arguments)};b.wbg.__wbg_new_72fb9a18b5ae2624=(()=>{const a=new a7();return r(a)});b.wbg.__wbg_self_ce0dbfc45cf2f5be=function(){return G((()=>{const a=self.self;return r(a)}),arguments)};b.wbg.__wbg_window_c6fb939a7f436783=function(){return G((()=>{const a=window.window;return r(a)}),arguments)};b.wbg.__wbg_globalThis_d1e6af4856ba331b=function(){return G((()=>{const a=globalThis.globalThis;return r(a)}),arguments)};b.wbg.__wbg_global_207b558942527489=function(){return G((()=>{const a=global.global;return r(a)}),arguments)};b.wbg.__wbg_set_d4638f722068f043=((a,b,d)=>{c(a)[b>>>Q]=k(d)});b.wbg.__wbg_isArray_2ab64d95e09ea0ae=(a=>{const b=_(c(a));return b});b.wbg.__wbg_push_a5b05aedc7234f9f=((a,b)=>{const d=c(a).push(c(b));return d});b.wbg.__wbg_instanceof_ArrayBuffer_836825be07d4c9d2=(a=>{let b;try{b=c(a) instanceof ArrayBuffer}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_call_b3ca7c6051f9bec1=function(){return G(((a,b,d)=>{const e=c(a).call(c(b),c(d));return r(e)}),arguments)};b.wbg.__wbg_isSafeInteger_f7b04ef02296c4d2=(a=>{const b=Number.isSafeInteger(c(a));return b});b.wbg.__wbg_entries_95cc2c823b285a09=(a=>{const b=a7.entries(c(a));return r(b)});b.wbg.__wbg_is_010fdc0f4ab96916=((a,b)=>{const d=a7.is(c(a),c(b));return d});b.wbg.__wbg_resolve_b0083a7967828ec8=(a=>{const b=Promise.resolve(c(a));return r(b)});b.wbg.__wbg_then_0c86a60e8fcfe9f6=((a,b)=>{const d=c(a).then(c(b));return r(d)});b.wbg.__wbg_then_a73caa9a87991566=((a,b,d)=>{const e=c(a).then(c(b),c(d));return r(e)});b.wbg.__wbg_buffer_12d079cc21e14bdb=(a=>{const b=c(a).buffer;return r(b)});b.wbg.__wbg_newwithbyteoffsetandlength_aa4a17c33a06e5cb=((a,b,d)=>{const e=new R(c(a),b>>>Q,d>>>Q);return r(e)});b.wbg.__wbg_new_63b92bc8671ed464=(a=>{const b=new R(c(a));return r(b)});b.wbg.__wbg_set_a47bac70306a19a7=((a,b,d)=>{c(a).set(c(b),d>>>Q)});b.wbg.__wbg_length_c20a40f15020d68a=(a=>{const b=c(a).length;return b});b.wbg.__wbg_instanceof_Uint8Array_2b3bbecd033d19f6=(a=>{let b;try{b=c(a) instanceof R}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_newwithlength_e9b4878cebadb3d3=(a=>{const b=new R(a>>>Q);return r(b)});b.wbg.__wbg_subarray_a1f73cd4b5b42fe1=((a,b,d)=>{const e=c(a).subarray(b>>>Q,d>>>Q);return r(e)});b.wbg.__wbg_stringify_8887fe74e1c50d81=function(){return G((a=>{const b=a0(c(a));return r(b)}),arguments)};b.wbg.__wbg_has_0af94d20077affa2=function(){return G(((a,b)=>{const d=a6.has(c(a),c(b));return d}),arguments)};b.wbg.__wbg_set_1f9b04f170055d33=function(){return G(((a,b,d)=>{const e=a6.set(c(a),c(b),c(d));return e}),arguments)};b.wbg.__wbindgen_bigint_get_as_i64=((a,b)=>{const e=c(b);const f=typeof e===a4?e:O;v()[a/a2+ W]=d(f)?a5(Q):f;h()[a/a3+ Q]=!d(f)});b.wbg.__wbindgen_debug_string=((b,d)=>{const e=w(c(d));const f=q(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;h()[b/a3+ W]=g;h()[b/a3+ Q]=f});b.wbg.__wbindgen_throw=((a,b)=>{throw new U(t(a,b))});b.wbg.__wbindgen_memory=(()=>{const b=a.memory;return r(b)});b.wbg.__wbindgen_closure_wrapper271=((a,b,c)=>{const d=y(a,b,a8,z);return r(d)});b.wbg.__wbindgen_closure_wrapper273=((a,b,c)=>{const d=A(a,b,a8,B);return r(d)});b.wbg.__wbindgen_closure_wrapper275=((a,b,c)=>{const d=A(a,b,a8,C);return r(d)});b.wbg.__wbindgen_closure_wrapper277=((a,b,c)=>{const d=A(a,b,a8,B);return r(d)});b.wbg.__wbindgen_closure_wrapper1195=((a,b,c)=>{const d=A(a,b,465,D);return r(d)});b.wbg.__wbindgen_closure_wrapper2618=((a,b,c)=>{const d=A(a,b,505,E);return r(d)});return b});var H=(async(a,b)=>{if(typeof Response===V&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===V){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var J=((a,b)=>{});var r=(a=>{if(i===b.length)b.push(b.length+ W);const c=i;i=b[c];b[c]=a;return c});var C=((b,c)=>{a.wasm_bindgen__convert__closures__invoke0_mut__hc63490ed88af2c8d(b,c)});var q=((a,b,c)=>{if(c===O){const c=o.encode(a);const d=b(c.length,W)>>>Q;n().subarray(d,d+ c.length).set(c);l=c.length;return d};let d=a.length;let e=b(d,W)>>>Q;const f=n();let g=Q;for(;g<d;g++){const b=a.charCodeAt(g);if(b>127)break;f[e+ g]=b};if(g!==d){if(g!==Q){a=a.slice(g)};e=c(e,d,d=g+ a.length*3,W)>>>Q;const b=n().subarray(e+ g,e+ d);const f=p(a,b);g+=f.written;e=c(e,d,g,W)>>>Q};l=g;return e});var f=(()=>{if(e===P||e.byteLength===Q){e=new Float64Array(a.memory.buffer)};return e});var j=(a=>{if(a<132)return;b[a]=i;i=a});var B=((b,c,d)=>{a.wasm_bindgen__convert__closures__invoke1_mut__h41b27db808de2f95(b,c,r(d))});var n=(()=>{if(m===P||m.byteLength===Q){m=new R(a.memory.buffer)};return m});var c=(a=>b[a]);var z=((b,c,d)=>{a.wasm_bindgen__convert__closures__invoke1__hd429cd3f891a047b(b,c,r(d))});var h=(()=>{if(g===P||g.byteLength===Q){g=new Int32Array(a.memory.buffer)};return g});var K=((b,c)=>{a=b.exports;M.__wbindgen_wasm_module=c;u=P;e=P;g=P;m=P;a.__wbindgen_start();return a});let a;const b=new N(128).fill(O);b.push(O,P,!0,!1);let e=P;let g=P;let i=b.length;let l=Q;let m=P;const o=typeof TextEncoder!==S?new TextEncoder(T):{encode:()=>{throw U(`TextEncoder not available`)}};const p=typeof o.encodeInto===V?((a,b)=>o.encodeInto(a,b)):((a,b)=>{const c=o.encode(a);b.set(c);return {read:a.length,written:c.length}});const s=typeof TextDecoder!==S?new TextDecoder(T,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw U(`TextDecoder not available`)}};if(typeof TextDecoder!==S){s.decode()};let u=P;const x=typeof a1===S?{register:()=>{},unregister:()=>{}}:new a1(b=>{a.__wbindgen_export_2.get(b.dtor)(b.a,b.b)});export default M;export{L as initSync}