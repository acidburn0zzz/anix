/*Copyright (C) 2018-2019 Nicolas Fouquet 

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see https://www.gnu.org/licenses.
*/
#include <stdio.h>

struct initrd_header
{
	unsigned char magic;
	char name[64];
	char content[256];
	unsigned int offset;
	unsigned int length;
};

int main(char argc, char **argv)
{
	
	int nheaders = (argc - 1);
	struct initrd_header headers[64];
	printf("Size of header: %d\n", sizeof(struct initrd_header));
	unsigned int off = sizeof(struct initrd_header) * 64 + sizeof(int);

	int i;
	printf("Headers: %d\n", nheaders);
	for(i = 0; i < nheaders; i++)
	{
		printf("Writing file %s at 0x%x with content: ", argv[i + 1],  off);
		strcpy(headers[i].name, argv[i + 1]);
		headers[i].offset = off;
		FILE *stream = fopen(argv[i + 1], "r");
		if(stream == 0)
		{
			printf("Error: file not found: %s\n", argv[i + 1]);
			return 1;
		}
		fseek(stream, 0, SEEK_END);
		headers[i].length = ftell(stream);
		off += headers[i].length;
		fclose(stream);
		headers[i].magic = 0xBF;
		
		//Read the content of the file
		FILE *rstream = fopen(argv[i + 1], "r");
		unsigned char *buf = (unsigned char *)malloc(headers[i].length);
		fread(buf, 1, headers[i].length, rstream);
		
		int w;
		for(w = 0; w < headers[i].length; w++){
			headers[i].content[w] = buf[w];
		}
		
		printf("%s\n", headers[i].content);
		fclose(rstream);
		free(buf);
	}
	
	FILE *wstream = fopen("./initrd.img", "w");
	unsigned char *data = (unsigned char *)malloc(off);
	fwrite(&nheaders, sizeof(int), 1, wstream);
	fwrite(headers, sizeof(struct initrd_header), 64, wstream);
	
	fclose(wstream);
	free(data);

	return 0;	
}
