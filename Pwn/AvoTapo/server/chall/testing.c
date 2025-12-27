#include <stdio.h>
#include <stdlib.h>

int main() {
    char ip[] = "http://211.222.57.18:8080";
    char cmd[300];
    
    snprintf(cmd, sizeof(cmd), "curl %s >/dev/null 2>&1", ip);
    int result = system(cmd);
    
    FILE *file = fopen("logfile.txt", "a");
    fprintf(file, "===============================\n");
    fprintf(file, "Test Camera IP : %s\n", ip);
    fprintf(file, "Streaming      : ffmpeg\n");
    fprintf(file, "Type           : YUY2\n");
    fprintf(file, "IsSuccess      : %s\n", (result == 0) ? "Success" : "Failed");
    fprintf(file, "===============================\n");
    fclose(file);
    
    return 0;
}