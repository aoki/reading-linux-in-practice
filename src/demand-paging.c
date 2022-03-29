#include <unistd.h>
#include <time.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <err.h>

#define BUFFER_SIZE (100 * 1024 * 1024)
#define NCYCLE 10
#define PAGE_SIZE 4096

int main(void)
{
  char *p;
  time_t t;
  char *s;

  // メモリ獲得前であることを示すメッセージ
  t = time(NULL);
  s = ctime(&t);
  printf("%.*s: before allocation. please press Entrer key\n", (int)(strlen(s) - 1), s);
  getchar();

  // 100 MB のメモリを確保する
  p = malloc(BUFFER_SIZE);
  if (p == NULL)
  {
    err(EXIT_FAILURE, "malloc() failed");
  }

  // メモリ獲得後であることを示すメッセージ
  t = time(NULL);
  s = ctime(&t);
  printf("%.*s: allocated %d MB. please press Enter key\n", (int)(strlen(s) - 1), s, BUFFER_SIZE / (1024 * 1024));
  getchar();

  //
  int i;
  for (i = 0; i < BUFFER_SIZE; i += PAGE_SIZE)
  {
    p[i] = 0;
    int cycle = i / (BUFFER_SIZE / NCYCLE);
    if (cycle != 0 && i % (BUFFER_SIZE / NCYCLE) == 0)
    {
      t = time(NULL);
      s = ctime(&t);
      printf("%.*s: touched %d MB\n", (int)(strlen(s) - 1), s, i / (1024 * 1024));
      sleep(1);
    }
  }

  t = time(NULL);
  s = ctime(&t);
  printf("%.*s: touched %d MB, please press Enter key\n", (int)(strlen(s) - 1), s, BUFFER_SIZE / (1024 * 1024));
  getchar();

  exit(EXIT_SUCCESS);
}
