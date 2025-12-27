
// libholy.so - sub_37F30 (getS 함수) return 버퍼 hexdump + 파일 저장 Frida 스크립트
// getS 함수 시그니처: const char *getS(lua_State *L, void *ud, size_t *size)

Java.perform(function() {
    var moduleName = "libholy.so";
    var funcOffset = 0x37F30;
    var outputPath = "/data/user/0/com.sample.holyautotapper/files/extract.luac";
    var dumpCount = 0;
    
    // 파일 저장 함수
    function saveToFile(filePath, data, size) {
        try {
            var fopen = new NativeFunction(Module.getGlobalExportByName("fopen"), 'pointer', ['pointer', 'pointer']);
            var fwrite = new NativeFunction(Module.getGlobalExportByName("fwrite"), 'size_t', ['pointer', 'size_t', 'size_t', 'pointer']);
            var fclose = new NativeFunction(Module.getGlobalExportByName("fclose"), 'int', ['pointer']);
            
            var pathPtr = Memory.allocUtf8String(filePath);
            var modePtr = Memory.allocUtf8String("wb");
            
            var fp = fopen(pathPtr, modePtr);
            if (fp.isNull()) {
                console.log("[!] Failed to open file for writing: " + filePath);
                return false;
            }
            
            var written = fwrite(data, 1, size, fp);
            fclose(fp);
            
            if (written == size) {
                console.log("[+] Successfully saved " + size + " bytes to: " + filePath);
                return true;
            } else {
                console.log("[!] Write incomplete: " + written + "/" + size + " bytes");
                return false;
            }
        } catch(e) {
            console.log("[!] saveToFile error: " + e);
            return false;
        }
    }
    
    function hookFunction() {
        var moduleBase = Process.getModuleByName(moduleName).base;
        if (!moduleBase) {
            console.log("[*] " + moduleName + " not loaded yet, waiting...");
            setTimeout(hookFunction, 1000);
            return;
        }
        
        var targetAddr = moduleBase.add(funcOffset);
        console.log("[*] " + moduleName + " base: " + moduleBase);
        console.log("[*] Hooking getS (sub_37F30) at: " + targetAddr);
        console.log("[*] Output path: " + outputPath);
        
        Interceptor.attach(targetAddr, {
            onEnter: function(args) {
                // getS(lua_State *L, void *ud, size_t *size)
                this.luaState = args[0];
                this.ud = args[1];         // LoadS* 구조체
                this.sizePtr = args[2];    // size_t* - 출력 크기 포인터
                
                console.log("\n[+] getS called");
                console.log("    lua_State*: " + this.luaState);
                console.log("    ud (LoadS*): " + this.ud);
                console.log("    size*: " + this.sizePtr);
                
                // LoadS 구조체 미리 읽기 (s, size, bytesServed)
                if (!this.ud.isNull()) {
                    try {
                        // LoadS { const char *s; size_t size; size_t bytesServed; }
                        var ls_s = this.ud.readPointer();
                        var ls_size = this.ud.add(Process.pointerSize).readULong();
                        var ls_bytesServed = this.ud.add(Process.pointerSize * 2).readULong();
                        
                        console.log("    [LoadS] s: " + ls_s);
                        console.log("    [LoadS] size: " + ls_size);
                        console.log("    [LoadS] bytesServed: " + ls_bytesServed);
                    } catch(e) {
                        console.log("    [!] Failed to read LoadS: " + e);
                    }
                }
            },
            onLeave: function(retval) {
                console.log("[+] getS returned: " + retval);
                
                if (retval.isNull()) {
                    console.log("[*] Return is NULL (no more data)");
                    return;
                }
                
                // size 포인터에서 실제 크기 읽기
                var dumpSize = 0;
                try {
                    if (!this.sizePtr.isNull()) {
                        // ARM64: size_t는 8바이트, ARM32: 4바이트
                        dumpSize = this.sizePtr.readULong();
                        console.log("[*] Actual chunk size from *size: " + dumpSize + " bytes");
                    }
                } catch(e) {
                    console.log("[!] Failed to read size: " + e);
                    dumpSize = 256;  // fallback
                }
                
                if (dumpSize > 0 && dumpSize < 0x100000) {  // 1MB 제한
                    try {
                        console.log("\n[*] Hexdump of returned chunk (" + dumpSize + " bytes):");
                        console.log(hexdump(retval, {
                            offset: 0,
                            length: dumpSize,
                            header: true,
                            ansi: true
                        }));
                        
                        // 파일로 저장
                        var savePath = outputPath;
                        if (dumpCount > 0) {
                            // 여러번 호출되면 번호 붙이기
                            savePath = outputPath.replace(".luac", "_" + dumpCount + ".luac");
                        }
                        saveToFile(savePath, retval, dumpSize);
                        dumpCount++;
                        
                    } catch(e) {
                        console.log("[!] Hexdump failed: " + e);
                        // 작은 크기로 재시도
                        try {
                            var safeSize = Math.min(dumpSize, 256);
                            console.log("[*] Retrying with " + safeSize + " bytes:");
                            console.log(hexdump(retval, {
                                offset: 0,
                                length: safeSize,
                                header: true,
                                ansi: true
                            }));
                        } catch(e2) {
                            console.log("[!] Still failed: " + e2);
                        }
                    }
                } else {
                    console.log("[!] Invalid size: " + dumpSize);
                }
            }
        });
        
        console.log("[*] Hook installed successfully!");
    }
    
    hookFunction();
});