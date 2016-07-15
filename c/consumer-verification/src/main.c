#include <config.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <dlfcn.h>
#include <curl/curl.h>

char *append_filename(char *executable, char *filename) {
  int executable_len = strlen(executable);
  int filename_len = strlen(filename);
  int package_len = strlen(PACKAGE_NAME);
  int diff = executable_len - package_len;
  char *string = malloc(diff + filename_len + 1);
  memcpy(string, executable, diff);
  memcpy(string + diff, filename, filename_len);
  string[diff + filename_len] = 0;
  return string;
}

char *slurp_file(char *filename) {
  FILE *fp = fopen(filename, "rb");
  if (fp) {
    fseek(fp, 0, SEEK_END);
    long fsize = ftell(fp);
    fseek(fp, 0, SEEK_SET);
    char *string = malloc(fsize + 1);
    int read = fread(string, fsize, 1, fp);
    string[fsize] = 0;
    fclose(fp);
    return string;
  } else {
    printf("Failed to read %s\n", filename);
    return 0;
  }
}

/*
Definitions of the exported functions from the pact mock server library
*/
typedef int32_t (*lib_create_mock_server)(char *, int32_t);
typedef int32_t (*lib_mock_server_matched)(int32_t);
typedef int32_t (*lib_cleanup_mock_server)(int32_t);

/* Execute the basic test against the provider server */
void execute_basic_test(int port) {
  CURLcode code = curl_global_init(CURL_GLOBAL_ALL);
  CURL *curl = curl_easy_init();
  if (curl) {
    char url[64];
    sprintf(url, "http://localhost:%d/mallory?name=ron&status=good", port);
    printf("Executing request against %s\n", url);
    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_VERBOSE, 1L);
    CURLcode res = curl_easy_perform(curl);
    if (res != CURLE_OK) {
      puts("\nRequest failed");
    }
    puts("\n");
    curl_easy_cleanup(curl);
  } else {
    puts("Could not initialise the curl library.");
  }
}

/*
  Run a basic test sing the simple_pact.js file
*/
void basic(char *executable, char *mock_server_lib) {
  /* Load the pact file into memory */
  char *pactfile = append_filename(executable, "simple_pact.js");
  char *pact = slurp_file(pactfile);
  if (pact) {
    /* Get a handle to the pact mock server library*/
    void *handle = dlopen(mock_server_lib, RTLD_NOW | RTLD_GLOBAL);
    if (handle) {
      /* We have a handle, so lookup the functions we need */
      lib_create_mock_server create_mock_server = dlsym(handle, "create_mock_server");
      lib_mock_server_matched mock_server_matched = dlsym(handle, "mock_server_matched");
      lib_cleanup_mock_server cleanup_mock_server = dlsym(handle, "cleanup_mock_server");
      if (create_mock_server) {
        /* Create the mock server from the pact file. The mock server port will be returned */
        int port = create_mock_server(pact, 0);
        printf("Mock server started on port %d\n", port);

        /* Now we execute out test against the mock server */
        execute_basic_test(port);

        /* Check the result */
        if (mock_server_matched(port)) {
          puts("OK: Mock server verified all requests, as expected");
        } else {
          puts("FAILED: Mock server did not match all requests!!");
        }

        /* Lastly, we need to shutdown and cleanup the mock server */
        cleanup_mock_server(port);
      } else {
        puts("Could not find 'create_mock_server' function in shared library");
      }
    } else {
      printf("Failed to open shared library %s\n", dlerror());
    }
    free(pact);
  } else {
    printf("Failed to read %s\n", pactfile);
  }
  free(pactfile);
}

int main (int argc, char **argv) {
  puts("This is " PACKAGE_STRING ".");

  if (argc < 3 || (strcmp(argv[1], "basic") != 0 && strcmp(argv[1], "error") != 0 && strcmp(argv[1], "v2") != 0)) {
    puts("You need to specify the test to run: basic, error, v2 and the path to the rust DLL");
    return 1;
  }

  if (strcmp(argv[1], "basic") == 0) {
    puts("Running basic pact test");
    basic(argv[0], argv[2]);
  }

  return 0;
}
