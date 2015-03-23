#include <stdio.h>
#include <limits.h>
#include <stdlib.h>
#include <getopt.h>

int main (int argc, char **argv)
{
    /* Variables for parsing arguments */
    int c,
        option_index = 0;
    int
        buffer = 0,
        delta  = 0;
    unsigned char
        fancy = 0,
        human = 0;
    FILE
        *fout = NULL,
        *fin  = NULL;
    static struct option long_options[] = {
        {"buffer",  required_argument, 0, 'b'},
        {"delta",   required_argument, 0, 'd'},
        {"fancy",   no_argument,       0, 'f'},
        {"help",    no_argument,       0, 'h'},
        {"human",   no_argument,       0, 'H'},
        {"input",   required_argument, 0, 'i'},
        {"output",  required_argument, 0, 'o'},
//      {"spacer",  required_argument, 0, 's'}, // This will set the separator charter on output.
        {"version", no_argument,       0, 'v'},
        {0, 0, 0, 0}
    };

    /* Parsing arguments */
    while ( ( c = getopt_long(argc, argv, "b:d:fhHi:o:v", long_options, &option_index) ) != -1)
    {
        switch (c)
        {
            case 'b':
                sscanf(optarg,"%d",&buffer);
                break;
            case 'd':
                sscanf(optarg,"%d",&delta);
                break;
            case 'f':
                fancy = 1;
                break;
            case 'h':
                printf ("Usage: Just pipe same numbers and I will print some others!\n"
                "-b --buffer=[integer] | Set number of max input lines.\n"
                "-d --delta=[integer]  | Set row's delta.\n"
                "-f --fancy            | Plot awesome graphs!\n"
                "-h --help             | Print help (this) text and exit.\n"
                "-H --human            | Print same useful extra information.\n"
                "-i --input=[file]     | Set a file to input instead stdin.\n"
                "-o --output=[file]    | Set a file to output instead stdout.\n"
                "-v --version          | Print version and exit.\n");
                exit (0);
                break;
            case 'H':
                human=1;
                break;
            case 'i':
                fin = fopen(optarg,"r");
                break;
            case 'o':
                fout = fopen(optarg,"w");
                break;
            case 'v':
                printf ("histogram version dev!\n");
                exit (0);
                break;
            default:
                fprintf (stdout, "Can't parse same argument; Errno: %o\n", c);
                exit (1);
        }
    }

    /* Setting default values */
    if ( ! fin )      fin    = stdin;
    if ( ! fout )     fout   = stdout;
    if ( buffer < 1 ) buffer = BUFSIZ;

    /* Variables for parse data */
    int i,j,
        min = INT_MAX,
        max = INT_MIN;    
    int data[buffer];

    /* Parsing data */
    while ( ! feof(fin) )
    {
        fscanf(fin,"%d",&data[i]);
        if ( data[i] < min ) min = data[i];
        if ( data[i] > max ) max = data[i];
        i++;
        if ( i == buffer )
        {
            fprintf(stderr,
                "Buffer exceeded.\n"
                "Only the first %d values were evaluated.\n"
                "Try to use -b.\n" , buffer );
            break;
        }
    }

    /* Auxiliary variables */
    int
        range = max-min,
        total = i,
        maxrow = 0;
    if ( delta < 1 ) delta = range / 10; // This is a good default delta?
    int rowno = range / delta;
    int row [ rowno ];
    
    /* Print same useful data if '-H' is enabled */
    if (human) fprintf( fout,
        "Read values:   %d\n"
        "Maximum value: %d\n"
        "Minimum value: %d\n"
        "Range:         %d\n"
        "Delta:         %d\n"
        ,total,max,min,range,delta);
    
    
    for (i=0;i<rowno;i++) row[i] = 0;
    for (i=0;i<total;i++)
    {
        for (j=0;j<rowno;j++)
            if ( data[i] < delta * j )
            {
                row[j]++;
                if (row[j] > maxrow) maxrow = row[j];
                break;
            }
    }
    if (fancy)
    {
        char lol[] = "80";
        char *envcol = getenv("COLUMNS");
        if (! envcol ) envcol = lol;
        int col;
        sscanf(envcol,"%d",&col);
        fprintf(fout,"I'm so fancy, You already know! \u266A \n");
        for (i=0;i<rowno;i++)
        {
            fprintf(fout,"%d\t|", row[i]);
            for (j=0;j<((row[i] * col)/(2*maxrow));j++) fprintf(fout,"=");
            fprintf(fout,"\n");
        }
    }
    else
    {
        for (i=0;i<rowno;i++)
        {
            fprintf(fout,"%d %d\n", (i+1)*delta, row[i]);
        }
    }
    return 0;
}

