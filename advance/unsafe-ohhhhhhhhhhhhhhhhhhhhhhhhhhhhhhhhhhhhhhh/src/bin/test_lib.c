#include <stdio.h>

extern void call_from_c(void);

int main() {
    printf("Calling Rust from C...\n");
    call_from_c();
    return 0;
}
// gcc .\src\bin\test_lib.c -L .\target\release -lunsafe_oh_lib -o testlib
// cp .\target\release\unsafe_oh_lib.dll .\ /* copy or install lib: important */
// .\testlib.exe

// cl.exe .\src\bin\test_lib.c .\target\release\unsafe_oh_lib.lib ws2_32.lib ntdll.lib userenv.lib advapi32.lib kernel32.lib /Fe:testlib2.exe
