// gcc -o chall ./chall.c -fno-stack-protector

#include <stdio.h>
#include <unistd.h>

void vuln()
{
    char buf[0x10];
    read(0, buf, 0x19);
    puts(buf);
}

int main()
{
    vuln(); 
}