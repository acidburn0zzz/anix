#include "types.h"
#include <stdarg.h>

#define RAMSCREEN 0xB8000       /* début de la mémoire vidéo */
#define SIZESCREEN 0xFA0        /* 4000, nombres d'octets d'une page texte */
#define SCREENLIM 0xB8FA0

char kX = 0;                    /* position courante du curseur à l'écran */
char kY = 10;
char kattr = 0x0E;              /* attributs vidéo des caractères à afficher */


/* 
 * 'scrollup' scrolle l'écran (la console mappée en ram) vers le haut
 * de n lignes (de 0 a 25).
 */
void scrollup(unsigned int n)
{
        unsigned char *video, *tmp;

        for (video = (unsigned char *) RAMSCREEN;
             video < (unsigned char *) SCREENLIM; video += 2) {
                tmp = (unsigned char *) (video + n * 160);

                if (tmp < (unsigned char *) SCREENLIM) {
                        *video = *tmp;
                        *(video + 1) = *(tmp + 1);
                } else {
                        *video = 0;
                        *(video + 1) = 0x07;
                }
        }

        kY -= n;
        if (kY < 0)
                kY = 0;
}

void putcar(uchar c)
{
        unsigned char *video;
        int i;

        if (c == 10) {          /* CR-NL */
                kX = 0;
                kY++;
        } else if (c == 9) {    /* TAB */
                kX = kX + 8 - (kX % 8);
        } else if (c == 13) {   /* CR */
                kX = 0;
        } else {                /* autres caractères */
                video = (unsigned char *) (RAMSCREEN + 2 * kX + 160 * kY);
                *video = c;
                *(video + 1) = kattr;

                kX++;
                if (kX > 79) {
                        kX = 0;
                        kY++;
                }
        }

        if (kY > 24)
                scrollup(kY - 24);
}

int ky_value(){
	return kY;
}

/*
 * 'print' affiche à l'écran, à la position courante du curseur, une chaîne
 * de caractères terminée par \0.
 */
void print(char *string)
{
        while (*string != 0) {  /* tant que le caractère est différent de 0x0 */
                putcar(*string);
                string++;
        }
} 

void printk(char *s, ...)
{
	va_list ap;

	char buf[16];
	int i, j, size, buflen, neg;

	unsigned char c;
	int ival;
	unsigned int uival;

	va_start(ap, s);

	while ((c = *s++)) {
		size = 0;
		neg = 0;

		if (c == 0)
			break;
		else if (c == '%') {
			c = *s++;
			if (c >= '0' && c <= '9') {
				size = c - '0';
				c = *s++;
			}

			if (c == 'd') {
				ival = va_arg(ap, int);
				if (ival < 0) {
					uival = 0 - ival;
					neg++;
				} else
					uival = ival;
				itoa(buf, uival, 10);

				buflen = strlen(buf);
				if (buflen < size)
					for (i = size, j = buflen; i >= 0;
					     i--, j--)
						buf[i] =
						    (j >=
						     0) ? buf[j] : '0';

				if (neg)
					printk("-%s", buf);
				else
					printk(buf);
			} else if (c == 'u') {
				uival = va_arg(ap, int);
				itoa(buf, uival, 10);

				buflen = strlen(buf);
				if (buflen < size)
					for (i = size, j = buflen; i >= 0;
					     i--, j--)
						buf[i] =
						    (j >=
						     0) ? buf[j] : '0';

				printk(buf);
			} else if (c == 'x' || c == 'X') {
				uival = va_arg(ap, int);
				itoa(buf, uival, 16);

				buflen = strlen(buf);
				if (buflen < size)
					for (i = size, j = buflen; i >= 0;
					     i--, j--)
						buf[i] =
						    (j >=
						     0) ? buf[j] : '0';

				printk("0x%s", buf);
			} else if (c == 'p') {
				uival = va_arg(ap, int);
				itoa(buf, uival, 16);
				size = 8;

				buflen = strlen(buf);
				if (buflen < size)
					for (i = size, j = buflen; i >= 0;
					     i--, j--)
						buf[i] =
						    (j >=
						     0) ? buf[j] : '0';

				printk("0x%s", buf);
			} else if (c == 's') {
				printk((char *) va_arg(ap, int));
			} 
		} else
			putcar(c);
	}

	return;
}
