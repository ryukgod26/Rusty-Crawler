let wasm_bindgen = (function(exports) {
    let script_src;
    if (typeof document !== 'undefined' && document.currentScript !== null) {
        script_src = new URL(document.currentScript.src, location.href).toString();
    }

    function __wbg_get_imports() {
        const import0 = {
            __proto__: null,
            __wbg___wbindgen_boolean_get_c0f3f60bac5a78d1: function(arg0) {
                const v = arg0;
                const ret = typeof(v) === 'boolean' ? v : undefined;
                return isLikeNone(ret) ? 0xFFFFFF : ret ? 1 : 0;
            },
            __wbg___wbindgen_debug_string_5398f5bb970e0daa: function(arg0, arg1) {
                const ret = debugString(arg1);
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg___wbindgen_is_undefined_52709e72fb9f179c: function(arg0) {
                const ret = arg0 === undefined;
                return ret;
            },
            __wbg___wbindgen_throw_6ddd609b62940d55: function(arg0, arg1) {
                throw new Error(getStringFromWasm0(arg0, arg1));
            },
            __wbg__wbg_cb_unref_6b5b6b8576d35cb1: function(arg0) {
                arg0._wbg_cb_unref();
            },
            __wbg_attachShader_6426e8576a115345: function(arg0, arg1, arg2) {
                arg0.attachShader(arg1, arg2);
            },
            __wbg_attachShader_e557f37438249ff7: function(arg0, arg1, arg2) {
                arg0.attachShader(arg1, arg2);
            },
            __wbg_bindBuffer_142694a9732bc098: function(arg0, arg1, arg2) {
                arg0.bindBuffer(arg1 >>> 0, arg2);
            },
            __wbg_bindBuffer_d2a4f6cfb33336fb: function(arg0, arg1, arg2) {
                arg0.bindBuffer(arg1 >>> 0, arg2);
            },
            __wbg_bindFramebuffer_4643a12ca1c72776: function(arg0, arg1, arg2) {
                arg0.bindFramebuffer(arg1 >>> 0, arg2);
            },
            __wbg_bindFramebuffer_fdc7c38f1c700e64: function(arg0, arg1, arg2) {
                arg0.bindFramebuffer(arg1 >>> 0, arg2);
            },
            __wbg_bindTexture_6a0892cd752b41d9: function(arg0, arg1, arg2) {
                arg0.bindTexture(arg1 >>> 0, arg2);
            },
            __wbg_bindTexture_6e7e157d0aabe457: function(arg0, arg1, arg2) {
                arg0.bindTexture(arg1 >>> 0, arg2);
            },
            __wbg_bindVertexArrayOES_082b0791772327fa: function(arg0, arg1) {
                arg0.bindVertexArrayOES(arg1);
            },
            __wbg_bindVertexArray_c307251f3ff61930: function(arg0, arg1) {
                arg0.bindVertexArray(arg1);
            },
            __wbg_blendFunc_2e98c5f57736e5f3: function(arg0, arg1, arg2) {
                arg0.blendFunc(arg1 >>> 0, arg2 >>> 0);
            },
            __wbg_blendFunc_4ce0991003a9468e: function(arg0, arg1, arg2) {
                arg0.blendFunc(arg1 >>> 0, arg2 >>> 0);
            },
            __wbg_bufferData_d20232e3d5dcdc62: function(arg0, arg1, arg2, arg3) {
                arg0.bufferData(arg1 >>> 0, arg2, arg3 >>> 0);
            },
            __wbg_bufferData_d3bd8c69ff4b7254: function(arg0, arg1, arg2, arg3) {
                arg0.bufferData(arg1 >>> 0, arg2, arg3 >>> 0);
            },
            __wbg_charCode_cc02509bd29dbb3a: function(arg0) {
                const ret = arg0.charCode;
                return ret;
            },
            __wbg_clearColor_080c8446c8438f8e: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.clearColor(arg1, arg2, arg3, arg4);
            },
            __wbg_clearColor_113259ba4c187edb: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.clearColor(arg1, arg2, arg3, arg4);
            },
            __wbg_clear_3d6ad4729e206aac: function(arg0, arg1) {
                arg0.clear(arg1 >>> 0);
            },
            __wbg_clear_5a0606f7c62ad39a: function(arg0, arg1) {
                arg0.clear(arg1 >>> 0);
            },
            __wbg_code_3c69123dcbcf263d: function(arg0, arg1) {
                const ret = arg1.code;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_compileShader_623a1051cf49494b: function(arg0, arg1) {
                arg0.compileShader(arg1);
            },
            __wbg_compileShader_7ca66245c2798601: function(arg0, arg1) {
                arg0.compileShader(arg1);
            },
            __wbg_createBuffer_1aa34315dc9585a2: function(arg0) {
                const ret = arg0.createBuffer();
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_createBuffer_8e47b88217a98607: function(arg0) {
                const ret = arg0.createBuffer();
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_createFramebuffer_911d55689ff8358e: function(arg0) {
                const ret = arg0.createFramebuffer();
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_createFramebuffer_97d39363cdd9242a: function(arg0) {
                const ret = arg0.createFramebuffer();
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_createProgram_1fa32901e4db13cd: function(arg0) {
                const ret = arg0.createProgram();
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_createProgram_8eb14525e7fcffb8: function(arg0) {
                const ret = arg0.createProgram();
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_createShader_9ffc9dc1832608d7: function(arg0, arg1) {
                const ret = arg0.createShader(arg1 >>> 0);
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_createShader_a00913b8c6489e6b: function(arg0, arg1) {
                const ret = arg0.createShader(arg1 >>> 0);
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_createTexture_9b1b4f40cab0097b: function(arg0) {
                const ret = arg0.createTexture();
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_createTexture_ceb367c3528574ec: function(arg0) {
                const ret = arg0.createTexture();
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_createVertexArrayOES_1b30eca82fb89274: function(arg0) {
                const ret = arg0.createVertexArrayOES();
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_createVertexArray_420460898dc8d838: function(arg0) {
                const ret = arg0.createVertexArray();
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_disable_62ec2189c50a0db7: function(arg0, arg1) {
                arg0.disable(arg1 >>> 0);
            },
            __wbg_disable_7731e2f3362ef1c5: function(arg0, arg1) {
                arg0.disable(arg1 >>> 0);
            },
            __wbg_document_c0320cd4183c6d9b: function(arg0) {
                const ret = arg0.document;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_drawArrays_13005ccff75e4210: function(arg0, arg1, arg2, arg3) {
                arg0.drawArrays(arg1 >>> 0, arg2, arg3);
            },
            __wbg_drawArrays_c20dedf441392005: function(arg0, arg1, arg2, arg3) {
                arg0.drawArrays(arg1 >>> 0, arg2, arg3);
            },
            __wbg_drawElements_3e2ee8b50927f361: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.drawElements(arg1 >>> 0, arg2, arg3 >>> 0, arg4);
            },
            __wbg_drawElements_5783df6a7ca41c48: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.drawElements(arg1 >>> 0, arg2, arg3 >>> 0, arg4);
            },
            __wbg_enableVertexAttribArray_60dadea3a00e104a: function(arg0, arg1) {
                arg0.enableVertexAttribArray(arg1 >>> 0);
            },
            __wbg_enableVertexAttribArray_626e8d2d9d1fdff9: function(arg0, arg1) {
                arg0.enableVertexAttribArray(arg1 >>> 0);
            },
            __wbg_enable_3728894fa8c1d348: function(arg0, arg1) {
                arg0.enable(arg1 >>> 0);
            },
            __wbg_enable_91dff7f43064bb54: function(arg0, arg1) {
                arg0.enable(arg1 >>> 0);
            },
            __wbg_error_a6fa202b58aa1cd3: function(arg0, arg1) {
                let deferred0_0;
                let deferred0_1;
                try {
                    deferred0_0 = arg0;
                    deferred0_1 = arg1;
                    console.error(getStringFromWasm0(arg0, arg1));
                } finally {
                    wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
                }
            },
            __wbg_framebufferTexture2D_bf4d47f4027a3682: function(arg0, arg1, arg2, arg3, arg4, arg5) {
                arg0.framebufferTexture2D(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4, arg5);
            },
            __wbg_framebufferTexture2D_e2f7d82e6707010e: function(arg0, arg1, arg2, arg3, arg4, arg5) {
                arg0.framebufferTexture2D(arg1 >>> 0, arg2 >>> 0, arg3 >>> 0, arg4, arg5);
            },
            __wbg_getContext_f04bf8f22dcb2d53: function() { return handleError(function (arg0, arg1, arg2) {
                const ret = arg0.getContext(getStringFromWasm0(arg1, arg2));
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            }, arguments); },
            __wbg_getElementById_d1f25d287b19a833: function(arg0, arg1, arg2) {
                const ret = arg0.getElementById(getStringFromWasm0(arg1, arg2));
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_getError_56a3fc6263f9b4b0: function(arg0) {
                const ret = arg0.getError();
                return ret;
            },
            __wbg_getError_57ff2f0e800634ca: function(arg0) {
                const ret = arg0.getError();
                return ret;
            },
            __wbg_getExtension_0b8543b0c6b3068d: function() { return handleError(function (arg0, arg1, arg2) {
                const ret = arg0.getExtension(getStringFromWasm0(arg1, arg2));
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            }, arguments); },
            __wbg_getModifierState_b0dd1ee94358290b: function(arg0, arg1, arg2) {
                const ret = arg0.getModifierState(getStringFromWasm0(arg1, arg2));
                return ret;
            },
            __wbg_getProgramInfoLog_50443ddea7475f57: function(arg0, arg1, arg2) {
                const ret = arg1.getProgramInfoLog(arg2);
                var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                var len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_getProgramInfoLog_e03efa51473d657e: function(arg0, arg1, arg2) {
                const ret = arg1.getProgramInfoLog(arg2);
                var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                var len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_getProgramParameter_46e2d49878b56edd: function(arg0, arg1, arg2) {
                const ret = arg0.getProgramParameter(arg1, arg2 >>> 0);
                return ret;
            },
            __wbg_getProgramParameter_7d3bd54ec02de007: function(arg0, arg1, arg2) {
                const ret = arg0.getProgramParameter(arg1, arg2 >>> 0);
                return ret;
            },
            __wbg_getShaderInfoLog_22f9e8c90a52f38d: function(arg0, arg1, arg2) {
                const ret = arg1.getShaderInfoLog(arg2);
                var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                var len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_getShaderInfoLog_40c6a4ae67d82dde: function(arg0, arg1, arg2) {
                const ret = arg1.getShaderInfoLog(arg2);
                var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                var len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_getShaderParameter_46f64f7ca5d534db: function(arg0, arg1, arg2) {
                const ret = arg0.getShaderParameter(arg1, arg2 >>> 0);
                return ret;
            },
            __wbg_getShaderParameter_82c275299b111f1b: function(arg0, arg1, arg2) {
                const ret = arg0.getShaderParameter(arg1, arg2 >>> 0);
                return ret;
            },
            __wbg_getUniformLocation_5eb08673afa04eee: function(arg0, arg1, arg2, arg3) {
                const ret = arg0.getUniformLocation(arg1, getStringFromWasm0(arg2, arg3));
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_getUniformLocation_90cdff44c2fceeb9: function(arg0, arg1, arg2, arg3) {
                const ret = arg0.getUniformLocation(arg1, getStringFromWasm0(arg2, arg3));
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_get_3ef1eba1850ade27: function() { return handleError(function (arg0, arg1) {
                const ret = Reflect.get(arg0, arg1);
                return ret;
            }, arguments); },
            __wbg_instanceof_HtmlCanvasElement_26125339f936be50: function(arg0) {
                let result;
                try {
                    result = arg0 instanceof HTMLCanvasElement;
                } catch (_) {
                    result = false;
                }
                const ret = result;
                return ret;
            },
            __wbg_instanceof_WebGl2RenderingContext_349f232f715e6bc2: function(arg0) {
                let result;
                try {
                    result = arg0 instanceof WebGL2RenderingContext;
                } catch (_) {
                    result = false;
                }
                const ret = result;
                return ret;
            },
            __wbg_instanceof_Window_23e677d2c6843922: function(arg0) {
                let result;
                try {
                    result = arg0 instanceof Window;
                } catch (_) {
                    result = false;
                }
                const ret = result;
                return ret;
            },
            __wbg_keyCode_bec0b9a76cae4555: function(arg0) {
                const ret = arg0.keyCode;
                return ret;
            },
            __wbg_linkProgram_b969f67969a850b5: function(arg0, arg1) {
                arg0.linkProgram(arg1);
            },
            __wbg_linkProgram_e626a3e7d78e1738: function(arg0, arg1) {
                arg0.linkProgram(arg1);
            },
            __wbg_log_f0870e08a87f5187: function(arg0, arg1) {
                console.log(getStringFromWasm0(arg0, arg1));
            },
            __wbg_new_227d7c05414eb861: function() {
                const ret = new Error();
                return ret;
            },
            __wbg_now_c6d7a7d35f74f6f1: function(arg0) {
                const ret = arg0.now();
                return ret;
            },
            __wbg_offsetX_45192a86d6db23e9: function(arg0) {
                const ret = arg0.offsetX;
                return ret;
            },
            __wbg_offsetY_533545ac9565ff65: function(arg0) {
                const ret = arg0.offsetY;
                return ret;
            },
            __wbg_performance_28be169151161678: function(arg0) {
                const ret = arg0.performance;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_requestAnimationFrame_206c97f410e7a383: function() { return handleError(function (arg0, arg1) {
                const ret = arg0.requestAnimationFrame(arg1);
                return ret;
            }, arguments); },
            __wbg_set_height_b6548a01bdcb689a: function(arg0, arg1) {
                arg0.height = arg1 >>> 0;
            },
            __wbg_set_onkeydown_d7fce864296308d1: function(arg0, arg1) {
                arg0.onkeydown = arg1;
            },
            __wbg_set_onkeyup_2caabed6aa37634f: function(arg0, arg1) {
                arg0.onkeyup = arg1;
            },
            __wbg_set_onmousedown_2691bce106adccab: function(arg0, arg1) {
                arg0.onmousedown = arg1;
            },
            __wbg_set_onmousemove_3050ac765cc5caba: function(arg0, arg1) {
                arg0.onmousemove = arg1;
            },
            __wbg_set_onmouseup_fadf0c7835be8a09: function(arg0, arg1) {
                arg0.onmouseup = arg1;
            },
            __wbg_set_width_c0fcaa2da53cd540: function(arg0, arg1) {
                arg0.width = arg1 >>> 0;
            },
            __wbg_shaderSource_06639e7b476e6ac2: function(arg0, arg1, arg2, arg3) {
                arg0.shaderSource(arg1, getStringFromWasm0(arg2, arg3));
            },
            __wbg_shaderSource_2bca0edc97475e95: function(arg0, arg1, arg2, arg3) {
                arg0.shaderSource(arg1, getStringFromWasm0(arg2, arg3));
            },
            __wbg_stack_3b0d974bbf31e44f: function(arg0, arg1) {
                const ret = arg1.stack;
                const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
                const len1 = WASM_VECTOR_LEN;
                getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
                getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
            },
            __wbg_static_accessor_GLOBAL_8adb955bd33fac2f: function() {
                const ret = typeof global === 'undefined' ? null : global;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_static_accessor_GLOBAL_THIS_ad356e0db91c7913: function() {
                const ret = typeof globalThis === 'undefined' ? null : globalThis;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_static_accessor_SELF_f207c857566db248: function() {
                const ret = typeof self === 'undefined' ? null : self;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_static_accessor_WINDOW_bb9f1ba69d61b386: function() {
                const ret = typeof window === 'undefined' ? null : window;
                return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
            },
            __wbg_texImage2D_c8f6fc33e0c1fb2c: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) {
                arg0.texImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9 === 0 ? undefined : getArrayU8FromWasm0(arg9, arg10));
            }, arguments); },
            __wbg_texImage2D_e173270e74e0a987: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) {
                arg0.texImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9 === 0 ? undefined : getArrayU8FromWasm0(arg9, arg10));
            }, arguments); },
            __wbg_texParameteri_f4b1596185f5432d: function(arg0, arg1, arg2, arg3) {
                arg0.texParameteri(arg1 >>> 0, arg2 >>> 0, arg3);
            },
            __wbg_texParameteri_fcdec30159061963: function(arg0, arg1, arg2, arg3) {
                arg0.texParameteri(arg1 >>> 0, arg2 >>> 0, arg3);
            },
            __wbg_uniform1i_953040fb972e9fab: function(arg0, arg1, arg2) {
                arg0.uniform1i(arg1, arg2);
            },
            __wbg_uniform1i_acd89bea81085be4: function(arg0, arg1, arg2) {
                arg0.uniform1i(arg1, arg2);
            },
            __wbg_uniform3f_1f319f9f4d116e54: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.uniform3f(arg1, arg2, arg3, arg4);
            },
            __wbg_uniform3f_832a24e94c7e0d0f: function(arg0, arg1, arg2, arg3, arg4) {
                arg0.uniform3f(arg1, arg2, arg3, arg4);
            },
            __wbg_useProgram_49b77c7558a0646a: function(arg0, arg1) {
                arg0.useProgram(arg1);
            },
            __wbg_useProgram_5405b431988b837b: function(arg0, arg1) {
                arg0.useProgram(arg1);
            },
            __wbg_vertexAttribPointer_ea73fc4cc5b7d647: function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
                arg0.vertexAttribPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4 !== 0, arg5, arg6);
            },
            __wbg_vertexAttribPointer_f63675d7fad431e6: function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
                arg0.vertexAttribPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4 !== 0, arg5, arg6);
            },
            __wbindgen_cast_0000000000000001: function(arg0, arg1) {
                // Cast intrinsic for `Closure(Closure { dtor_idx: 296, function: Function { arguments: [NamedExternref("KeyboardEvent")], shim_idx: 297, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
                const ret = makeMutClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__h4589f87110aa1d98, wasm_bindgen__convert__closures_____invoke__h8b6a69e57a494e30);
                return ret;
            },
            __wbindgen_cast_0000000000000002: function(arg0, arg1) {
                // Cast intrinsic for `Closure(Closure { dtor_idx: 296, function: Function { arguments: [NamedExternref("MouseEvent")], shim_idx: 297, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
                const ret = makeMutClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__h4589f87110aa1d98, wasm_bindgen__convert__closures_____invoke__h8b6a69e57a494e30_1);
                return ret;
            },
            __wbindgen_cast_0000000000000003: function(arg0, arg1) {
                // Cast intrinsic for `Closure(Closure { dtor_idx: 320, function: Function { arguments: [], shim_idx: 321, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
                const ret = makeMutClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__hee05f91bdd482beb, wasm_bindgen__convert__closures_____invoke__h17bc59ac840875fa);
                return ret;
            },
            __wbindgen_cast_0000000000000004: function(arg0, arg1) {
                // Cast intrinsic for `Ref(Slice(U8)) -> NamedExternref("Uint8Array")`.
                const ret = getArrayU8FromWasm0(arg0, arg1);
                return ret;
            },
            __wbindgen_cast_0000000000000005: function(arg0, arg1) {
                // Cast intrinsic for `Ref(String) -> Externref`.
                const ret = getStringFromWasm0(arg0, arg1);
                return ret;
            },
            __wbindgen_init_externref_table: function() {
                const table = wasm.__wbindgen_externrefs;
                const offset = table.grow(4);
                table.set(0, undefined);
                table.set(offset + 0, undefined);
                table.set(offset + 1, null);
                table.set(offset + 2, true);
                table.set(offset + 3, false);
            },
        };
        return {
            __proto__: null,
            "./rusty_crawler_bg.js": import0,
        };
    }

    function wasm_bindgen__convert__closures_____invoke__h17bc59ac840875fa(arg0, arg1) {
        wasm.wasm_bindgen__convert__closures_____invoke__h17bc59ac840875fa(arg0, arg1);
    }

    function wasm_bindgen__convert__closures_____invoke__h8b6a69e57a494e30(arg0, arg1, arg2) {
        wasm.wasm_bindgen__convert__closures_____invoke__h8b6a69e57a494e30(arg0, arg1, arg2);
    }

    function wasm_bindgen__convert__closures_____invoke__h8b6a69e57a494e30_1(arg0, arg1, arg2) {
        wasm.wasm_bindgen__convert__closures_____invoke__h8b6a69e57a494e30_1(arg0, arg1, arg2);
    }

    function addToExternrefTable0(obj) {
        const idx = wasm.__externref_table_alloc();
        wasm.__wbindgen_externrefs.set(idx, obj);
        return idx;
    }

    const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
        ? { register: () => {}, unregister: () => {} }
        : new FinalizationRegistry(state => state.dtor(state.a, state.b));

    function debugString(val) {
        // primitive types
        const type = typeof val;
        if (type == 'number' || type == 'boolean' || val == null) {
            return  `${val}`;
        }
        if (type == 'string') {
            return `"${val}"`;
        }
        if (type == 'symbol') {
            const description = val.description;
            if (description == null) {
                return 'Symbol';
            } else {
                return `Symbol(${description})`;
            }
        }
        if (type == 'function') {
            const name = val.name;
            if (typeof name == 'string' && name.length > 0) {
                return `Function(${name})`;
            } else {
                return 'Function';
            }
        }
        // objects
        if (Array.isArray(val)) {
            const length = val.length;
            let debug = '[';
            if (length > 0) {
                debug += debugString(val[0]);
            }
            for(let i = 1; i < length; i++) {
                debug += ', ' + debugString(val[i]);
            }
            debug += ']';
            return debug;
        }
        // Test for built-in
        const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
        let className;
        if (builtInMatches && builtInMatches.length > 1) {
            className = builtInMatches[1];
        } else {
            // Failed to match the standard '[object ClassName]'
            return toString.call(val);
        }
        if (className == 'Object') {
            // we're a user defined class or Object
            // JSON.stringify avoids problems with cycles, and is generally much
            // easier than looping through ownProperties of `val`.
            try {
                return 'Object(' + JSON.stringify(val) + ')';
            } catch (_) {
                return 'Object';
            }
        }
        // errors
        if (val instanceof Error) {
            return `${val.name}: ${val.message}\n${val.stack}`;
        }
        // TODO we could test for more things here, like `Set`s and `Map`s.
        return className;
    }

    function getArrayU8FromWasm0(ptr, len) {
        ptr = ptr >>> 0;
        return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
    }

    let cachedDataViewMemory0 = null;
    function getDataViewMemory0() {
        if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
            cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
        }
        return cachedDataViewMemory0;
    }

    function getStringFromWasm0(ptr, len) {
        ptr = ptr >>> 0;
        return decodeText(ptr, len);
    }

    let cachedUint8ArrayMemory0 = null;
    function getUint8ArrayMemory0() {
        if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
            cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
        }
        return cachedUint8ArrayMemory0;
    }

    function handleError(f, args) {
        try {
            return f.apply(this, args);
        } catch (e) {
            const idx = addToExternrefTable0(e);
            wasm.__wbindgen_exn_store(idx);
        }
    }

    function isLikeNone(x) {
        return x === undefined || x === null;
    }

    function makeMutClosure(arg0, arg1, dtor, f) {
        const state = { a: arg0, b: arg1, cnt: 1, dtor };
        const real = (...args) => {

            // First up with a closure we increment the internal reference
            // count. This ensures that the Rust closure environment won't
            // be deallocated while we're invoking it.
            state.cnt++;
            const a = state.a;
            state.a = 0;
            try {
                return f(a, state.b, ...args);
            } finally {
                state.a = a;
                real._wbg_cb_unref();
            }
        };
        real._wbg_cb_unref = () => {
            if (--state.cnt === 0) {
                state.dtor(state.a, state.b);
                state.a = 0;
                CLOSURE_DTORS.unregister(state);
            }
        };
        CLOSURE_DTORS.register(real, state, state);
        return real;
    }

    function passStringToWasm0(arg, malloc, realloc) {
        if (realloc === undefined) {
            const buf = cachedTextEncoder.encode(arg);
            const ptr = malloc(buf.length, 1) >>> 0;
            getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
            WASM_VECTOR_LEN = buf.length;
            return ptr;
        }

        let len = arg.length;
        let ptr = malloc(len, 1) >>> 0;

        const mem = getUint8ArrayMemory0();

        let offset = 0;

        for (; offset < len; offset++) {
            const code = arg.charCodeAt(offset);
            if (code > 0x7F) break;
            mem[ptr + offset] = code;
        }
        if (offset !== len) {
            if (offset !== 0) {
                arg = arg.slice(offset);
            }
            ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
            const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
            const ret = cachedTextEncoder.encodeInto(arg, view);

            offset += ret.written;
            ptr = realloc(ptr, len, offset, 1) >>> 0;
        }

        WASM_VECTOR_LEN = offset;
        return ptr;
    }

    let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
    cachedTextDecoder.decode();
    function decodeText(ptr, len) {
        return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
    }

    const cachedTextEncoder = new TextEncoder();

    if (!('encodeInto' in cachedTextEncoder)) {
        cachedTextEncoder.encodeInto = function (arg, view) {
            const buf = cachedTextEncoder.encode(arg);
            view.set(buf);
            return {
                read: arg.length,
                written: buf.length
            };
        };
    }

    let WASM_VECTOR_LEN = 0;

    let wasmModule, wasm;
    function __wbg_finalize_init(instance, module) {
        wasm = instance.exports;
        wasmModule = module;
        cachedDataViewMemory0 = null;
        cachedUint8ArrayMemory0 = null;
        wasm.__wbindgen_start();
        return wasm;
    }

    async function __wbg_load(module, imports) {
        if (typeof Response === 'function' && module instanceof Response) {
            if (typeof WebAssembly.instantiateStreaming === 'function') {
                try {
                    return await WebAssembly.instantiateStreaming(module, imports);
                } catch (e) {
                    const validResponse = module.ok && expectedResponseType(module.type);

                    if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                        console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                    } else { throw e; }
                }
            }

            const bytes = await module.arrayBuffer();
            return await WebAssembly.instantiate(bytes, imports);
        } else {
            const instance = await WebAssembly.instantiate(module, imports);

            if (instance instanceof WebAssembly.Instance) {
                return { instance, module };
            } else {
                return instance;
            }
        }

        function expectedResponseType(type) {
            switch (type) {
                case 'basic': case 'cors': case 'default': return true;
            }
            return false;
        }
    }

    function initSync(module) {
        if (wasm !== undefined) return wasm;


        if (module !== undefined) {
            if (Object.getPrototypeOf(module) === Object.prototype) {
                ({module} = module)
            } else {
                console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
            }
        }

        const imports = __wbg_get_imports();
        if (!(module instanceof WebAssembly.Module)) {
            module = new WebAssembly.Module(module);
        }
        const instance = new WebAssembly.Instance(module, imports);
        return __wbg_finalize_init(instance, module);
    }

    async function __wbg_init(module_or_path) {
        if (wasm !== undefined) return wasm;


        if (module_or_path !== undefined) {
            if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
                ({module_or_path} = module_or_path)
            } else {
                console.warn('using deprecated parameters for the initialization function; pass a single object instead')
            }
        }

        if (module_or_path === undefined && script_src !== undefined) {
            module_or_path = script_src.replace(/\.js$/, "_bg.wasm");
        }
        const imports = __wbg_get_imports();

        if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
            module_or_path = fetch(module_or_path);
        }

        const { instance, module } = await __wbg_load(await module_or_path, imports);

        return __wbg_finalize_init(instance, module);
    }

    return Object.assign(__wbg_init, { initSync }, exports);
})({ __proto__: null });
