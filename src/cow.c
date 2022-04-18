#include <stdlib.h>
#include <stdio.h>
#include <err.h>
#include <unistd.h>
#include <sys/wait.h>

#include <sys/mman.h>
#include <sys/types.h>

#define BUFFER_SIZE (100 * 1024 * 1024)
#define PAGE_SIZE 4096
#define COMMAND_SIZE 4096

static char *p;
static char command[COMMAND_SIZE];

static void child_fn(char *p)
{
  printf("*** child(%d) ps info before memory access ***:\n", getpid());
  fflush(stdout);
  snprintf(command, COMMAND_SIZE, "ps -o pid,comm,vsz,rss,min_flt,maj_flt | grep '^ *%d'", getpid());
  system(command);
  printf("\n\n");

  printf("*** free memory info before memory access ***:\n");
  fflush(stdout);
  system("free");
  printf("\n\n");

  int i;
  for (i = 0; i < BUFFER_SIZE; i += PAGE_SIZE)
  {
    p[i] = 0;
  }

  printf("*** child ps info after memory access ***:\n");
  fflush(stdout);
  system(command);
  printf("\n\n");

  printf("*** free memory info after memory access ***:\n");
  fflush(stdout);
  system("free");
  printf("\n\n");

  exit(EXIT_SUCCESS);
}

static void parent_fn()
{
  wait(NULL);
  exit(EXIT_SUCCESS);
}

int main(void)
{
  printf("*** free memory info before malloc ***:\n");
  fflush(stdout);
  system("free");
  printf("\n\n");

  char *buf;
  p = malloc(BUFFER_SIZE);

  printf("*** free memory info before memory access ***:\n");
  fflush(stdout);
  system("free");
  printf("\n\n");

  if (p == NULL)
  {
    err(EXIT_FAILURE, "malloc() failed");
  }
  int i;
  for (i = 0; i < BUFFER_SIZE; i += PAGE_SIZE)
  {
    p[i] = 0;
  }

  printf("*** free memory info before fork ***:\n");
  fflush(stdout);
  system("free");
  printf("\n\n");

  pid_t ret;
  ret = fork();
  if (ret == -1)
  {
    err(EXIT_FAILURE, "fork() failed");
  }

  if (ret == 0)
  {
    child_fn(p);
  }
  else
  {
    parent_fn();
  }
  err(EXIT_FAILURE, "shouldn't reach here");
}
