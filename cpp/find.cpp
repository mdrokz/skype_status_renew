#include <windows.h>

#include <stdio.h>

int main(void)
{
    /* code */
    HWND skypeHandle = FindWindowA(NULL,"Skype");

    SetForegroundWindow(skypeHandle);

    printf("ERROR CODE %lu",GetLastError());

    return 0;
}
