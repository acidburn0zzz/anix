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

#ifdef __cplusplus
extern "C" {
#endif

void itoa(char *buf, unsigned long int n, int base)
{
	unsigned long int tmp;
	int i, j;

	tmp = n;
	i = 0;

	do {
		tmp = n % base;
		buf[i++] = (tmp < 10) ? (tmp + '0') : (tmp + 'a' - 10);
	} while (n /= base);
	buf[i--] = 0;

	for (j = 0; j < i; j++, i--) {
		tmp = buf[j];
		buf[j] = buf[i];
		buf[i] = tmp;
	}
}

void *mmemcpy(char *dst, char *src, int n)
{
	char *p = dst;
	while (n--)
		*dst++ = *src++;
	return p;
}

int sstrcpy(char *dst, char *src)
{
	int i = 0;
	while ((dst[i] = src[i++]));

	return i;
}

int sstrcmp(char *dst, char *src)
{
	int i = 0;

	while ((dst[i] == src[i])) {
		if (src[i++] == 0)
			return 0;
	}

	return 1;
}

int sstrlen(char *s)
{
	int i = 0;
	while (*s++)
		i++;
	return i;
}

#ifdef __cplusplus
}
#endif

