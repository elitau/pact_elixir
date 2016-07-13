#include <config.h>
#include <stdio.h>
#include <curl/curl.h>

int main (void) {
  puts ("Hello World!");
  puts ("This is " PACKAGE_STRING ".");
  return 0;
}
