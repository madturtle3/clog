#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define LOG_FILE_NAME "log"

enum record_types {
    HEADER='!',
    RECORD='*',
    UPDATE='$'
};

struct field
{
    char *text;
    struct field *right;
};

struct linestart
{
    enum record_types type;
    struct linestart *down;
    struct field *right;
};

struct linestart *read_log() {
    FILE *logfile = fopen(LOG_FILE_NAME,"r");
    enum record_types rtype;
    char fields[50];
    fscanf(logfile,"%c %50[^\n]s\n",&rtype,&fields);
    printf("%s\n",fields);
    switch (rtype)
    {
    case HEADER:
        /* code */
        break;
    case RECORD:
        break;
    case UPDATE:
        break;
    default:
        break;
    }
}

int main(int argc, char *argv[]) {
    if (argc >= 2) {
        char *command = argv[1];
        if (strcmp(command,"list")) {
            read_log();
        }
    }
    else {
        printf("No command provided\n");
        return 1;
    }
}