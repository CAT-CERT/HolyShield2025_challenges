#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <signal.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <fcntl.h>
#include <errno.h>
#include <libgen.h>
#include <time.h>
#include <dirent.h>
#include <sys/wait.h>

#define MAX_CLIENTS 100
#define BUFFER_SIZE 4096
#define MAX_PATH 512
#define MAX_USERNAME_LEN 32
#define MAX_PASSWORD_LEN 64
#define MAX_JSON_SIZE 4096
#define MAX_FIELD_SIZE 256

#ifndef HAVE_JSON_C
typedef struct {
    char *string;
    size_t length;
    size_t capacity;
} json_object;

json_object* json_tokener_parse(const char *str) {
    if (!str) return NULL;
    
    size_t len = strlen(str);
    if (len > MAX_JSON_SIZE) return NULL;
    
    json_object *obj = malloc(sizeof(json_object));
    if (obj) {
        obj->capacity = len + 1;
        obj->string = malloc(obj->capacity);
        if (obj->string) {
            memcpy(obj->string, str, len);
            obj->string[len] = '\0';
            obj->length = len;
        } else {
            free(obj);
            return NULL;
        }
    }
    return obj;
}

void json_object_put(json_object *obj) {
    if (obj) {
        if (obj->string) {
            memset(obj->string, 0, obj->capacity);
            free(obj->string);
        }
        memset(obj, 0, sizeof(json_object));
        free(obj);
    }
}

int json_object_object_get_ex(json_object *obj, const char *key, json_object **value) {
    if (!obj || !obj->string || !key || !value) return 0;
    
    size_t key_len = strlen(key);
    if (key_len > 64) return 0;
    
    char search[128];
    int ret = snprintf(search, sizeof(search), "\"%s\":", key);
    if (ret < 0 || ret >= sizeof(search)) return 0;
    
    char *found = strstr(obj->string, search);
    if (found) {
        found = strchr(found, ':');
        if (!found) return 0;
        found++;
        
        while (*found == ' ' || *found == '\t') found++;
        if (*found == '"') {
            found++;
            char *end = strchr(found, '"');
            if (end) {
                size_t value_len = end - found;
                if (value_len > MAX_FIELD_SIZE) return 0;
                
                *value = malloc(sizeof(json_object));
                if (*value) {
                    (*value)->capacity = value_len + 1;
                    (*value)->string = malloc((*value)->capacity);
                    if ((*value)->string) {
                        memcpy((*value)->string, found, value_len);
                        (*value)->string[value_len] = '\0';
                        (*value)->length = value_len;
                        return 1;
                    } else {
                        free(*value);
                        *value = NULL;
                    }
                }
            }
        }
    }
    return 0;
}

const char* json_object_get_string(json_object *obj) {
    return (obj && obj->string) ? obj->string : NULL;
}

const char* json_object_to_json_string(json_object *obj) {
    return (obj && obj->string) ? obj->string : "{}";
}
#endif

struct {
    char docroot[MAX_PATH];
    char realm[256];
    int max_requests;
    int network_timeout;
    int no_symlinks;
    int no_dirlists;
    int rfc1918_filter;
    int daemon_mode;
} conf;

struct client {
    int fd;
    struct sockaddr_in addr;
    time_t connect_time;
    char buffer[BUFFER_SIZE];
    int buffer_len;
};

static struct client clients[MAX_CLIENTS];
static int server_fd = -1;
static int running = 1;

typedef struct json_obj {
    char *data;
    size_t capacity;
    struct json_obj *next;
} json_obj;

void signal_handler(int sig);
int create_server_socket(const char *addr, int port);
void handle_client(struct client *cl);
int do_login(struct client *cl, json_object *req);
int setLanguage(struct client *cl, json_object *req);
void send_response(struct client *cl, int code, const char *content_type, const char *body);
void send_error(struct client *cl, int code, const char *message);
int slp_http_response_err_code(struct client *cl, int error_code);
char* load_file(const char *filename);
void trim_string(char *str);
int safe_parse_multipart_field(const char *body, const char *field_name, char *output, size_t output_size);
int safe_parse_urlencoded_field(const char *body, const char *field_name, char *output, size_t output_size);
json_obj* jso_new_obj(void);
int jso_free_obj(json_obj *obj);
int jso_is_obj(json_object *obj);
void jso_add_int(json_obj *obj, const char *key, int value);
const char* json_object_to_json_string_for_obj(json_obj *obj);
int jso_obj_get_int(json_obj *obj, const char *key, int *value);
json_obj* exec_and_read_json(const char *command, char *buf, int bufsize);
void slp_http_response_json(struct client *cl, const char *json_str);
json_obj* jso_from_string(const char *json_str);
int user_exists(const char *username);
int check_password(const char *username, const char *password);
void send_auth_success(struct client *cl, const char *username, const char *role);
void send_unauth_resp(struct client *cl, int error_code);
void set_working_directory_to_executable_dir(const char *argv0);

void signal_handler(int sig) {
    running = 0;
    if (server_fd >= 0) {
        close(server_fd);
    }
    exit(0);
}

int create_server_socket(const char *addr, int port) {
    int fd, opt = 1;
    struct sockaddr_in server_addr;

    fd = socket(AF_INET, SOCK_STREAM, 0);
    if (fd < 0) {
        perror("socket");
        return -1;
    }

    setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, &opt, sizeof(opt));

    memset(&server_addr, 0, sizeof(server_addr));
    server_addr.sin_family = AF_INET;
    server_addr.sin_port = htons(port);
    
    if (addr && strlen(addr) > 0) {
        inet_pton(AF_INET, addr, &server_addr.sin_addr);
    } else {
        server_addr.sin_addr.s_addr = INADDR_ANY;
    }

    if (bind(fd, (struct sockaddr*)&server_addr, sizeof(server_addr)) < 0) {
        perror("bind");
        close(fd);
        return -1;
    }

    if (listen(fd, 10) < 0) {
        perror("listen");
        close(fd);
        return -1;
    }

    return fd;
}

void send_response(struct client *cl, int code, const char *content_type, const char *body) {
    char response[BUFFER_SIZE * 2];
    const char *status_text;
    
    switch (code) {
        case 200: status_text = "OK"; break;
        case 400: status_text = "Bad Request"; break;
        case 401: status_text = "Unauthorized"; break;
        case 403: status_text = "Forbidden"; break;
        case 404: status_text = "Not Found"; break;
        case 500: status_text = "Internal Server Error"; break;
        default: status_text = "Unknown"; break;
    }

    int body_len = body ? strlen(body) : 0;

    int header_len = snprintf(response, sizeof(response),
        "HTTP/1.1 %d %s\r\n"
        "Content-Type: %s\r\n"
        "Content-Length: %d\r\n"
        "Connection: close\r\n"
        "X-Content-Type-Options: nosniff\r\n"
        "X-Frame-Options: DENY\r\n"
        "X-XSS-Protection: 1; mode=block\r\n"
        "Cache-Control: no-cache, no-store, must-revalidate\r\n"
        "\r\n",
        code, status_text, content_type, body_len);

    if (header_len > 0 && header_len < sizeof(response)) {
        send(cl->fd, response, header_len, 0);
        
        if (body && body_len > 0) {
            send(cl->fd, body, body_len, 0);
        }
    }
}

void send_error(struct client *cl, int code, const char *message) {
    if (!message) message = "Unknown error";

    char safe_message[256];
    size_t msg_len = strlen(message);
    if (msg_len >= sizeof(safe_message)) msg_len = sizeof(safe_message) - 1;
    
    size_t j = 0;
    for (size_t i = 0; i < msg_len && j < sizeof(safe_message) - 1; i++) {
        char c = message[i];
        if (c == '<' || c == '>' || c == '"' || c == '&') {
            continue;
        }
        safe_message[j++] = c;
    }
    safe_message[j] = '\0';
    
    char body[512];
    snprintf(body, sizeof(body), 
        "<html><head><title>Error %d</title></head>"
        "<body><h1>Error %d</h1><p>%s</p></body></html>", 
        code, code, safe_message);
    send_response(cl, code, "text/html", body);
}

void trim_string(char *str) {
    if (!str) return;
    char *start = str;
    while (*start && (*start == ' ' || *start == '\t' || *start == '\r' || *start == '\n')) {
        start++;
    }

    char *end = start + strlen(start) - 1;
    while (end > start && (*end == ' ' || *end == '\t' || *end == '\r' || *end == '\n')) {
        *end = '\0';
        end--;
    }

    if (start != str) {
        memmove(str, start, strlen(start) + 1);
    }
}

char* load_file(const char *filename) {
    if (!filename) return NULL;
    
    if (strstr(filename, "..") || strchr(filename, '/')) {
        return NULL;
    }
    
    FILE *fp = fopen(filename, "rb");
    if (!fp) {
        return NULL;
    }
    
    fseek(fp, 0, SEEK_END);
    long file_size = ftell(fp);
    fseek(fp, 0, SEEK_SET);
    
    if (file_size <= 0 || file_size > 1024 * 1024) {
        fclose(fp);
        return NULL;
    }
    
    char *content = malloc(file_size + 1);
    if (!content) {
        fclose(fp);
        return NULL;
    }
    
    size_t bytes_read = fread(content, 1, file_size, fp);
    content[bytes_read] = '\0';
    fclose(fp);
    
    return content;
}

void set_working_directory_to_executable_dir(const char *argv0) {
    char exe_path[MAX_PATH];
    char path_copy[MAX_PATH];
    int have_path = 0;

    if (!argv0) {
        return;
    }

#if defined(__linux__)
    ssize_t len = readlink("/proc/self/exe", exe_path, sizeof(exe_path) - 1);
    if (len > 0 && len < (ssize_t)sizeof(exe_path)) {
        exe_path[len] = '\0';
        have_path = 1;
    }
#endif

    if (!have_path) {
        char *resolved = realpath(argv0, exe_path);
        if (!resolved) {
            return;
        }
        have_path = 1;
    }

    if (!have_path) {
        return;
    }

    strncpy(path_copy, exe_path, sizeof(path_copy) - 1);
    path_copy[sizeof(path_copy) - 1] = '\0';

    char *dir = dirname(path_copy);
    if (!dir || dir[0] == '\0') {
        return;
    }

    if (chdir(dir) == 0) {
        strncpy(conf.docroot, dir, sizeof(conf.docroot) - 1);
        conf.docroot[sizeof(conf.docroot) - 1] = '\0';
    }
}

int safe_parse_multipart_field(const char *body, const char *field_name, char *output, size_t output_size) {
    if (!body || !field_name || !output || output_size == 0) {
        return -1;
    }
    
    char search_pattern[128];
    int ret = snprintf(search_pattern, sizeof(search_pattern), "name=\"%s\"", field_name);
    if (ret < 0 || ret >= sizeof(search_pattern)) {
        return -1;
    }
    
    char *field_start = strstr(body, search_pattern);
    if (!field_start) return -1;
    
    field_start = strchr(field_start, '\n');
    if (!field_start) return -1;
    field_start++;
    
    char *field_end = strstr(field_start, "\n------");
    if (!field_end) field_end = strstr(field_start, "\r\n------");
    if (!field_end) return -1;
    
    if (field_end <= field_start) return -1;
    
    size_t len = field_end - field_start;
    if (len >= output_size) len = output_size - 1;
    
    memcpy(output, field_start, len);
    output[len] = '\0';
    trim_string(output);
    
    return 0;
}

int safe_parse_urlencoded_field(const char *body, const char *field_name, char *output, size_t output_size) {
    if (!body || !field_name || !output || output_size == 0) {
        return -1;
    }
    
    char search_pattern[128];
    int ret = snprintf(search_pattern, sizeof(search_pattern), "%s=", field_name);
    if (ret < 0 || ret >= sizeof(search_pattern)) {
        return -1;
    }
    
    char *field_start = strstr(body, search_pattern);
    if (!field_start) return -1;
    
    field_start += strlen(search_pattern);
    
    char *field_end = strchr(field_start, '&');
    if (!field_end) field_end = field_start + strlen(field_start);
    if (field_end <= field_start) return -1;
    
    size_t len = field_end - field_start;
    if (len >= output_size) len = output_size - 1;
    
    memcpy(output, field_start, len);
    output[len] = '\0';
    
    return 0;
}

json_obj* jso_new_obj(void) {
    json_obj *obj = malloc(sizeof(json_obj));
    if (obj) {
        obj->capacity = 1024;
        obj->data = malloc(obj->capacity);
        if (obj->data) {
            obj->data[0] = '\0';
            obj->next = NULL;
            strcpy(obj->data, "{}");
        } else {
            free(obj);
            return NULL;
        }
    }
    return obj;
}

int jso_free_obj(json_obj *obj) {
    if (obj) {
        if (obj->data) {
            memset(obj->data, 0, obj->capacity);
            free(obj->data);
        }
        memset(obj, 0, sizeof(json_obj));
        free(obj);
    }
    return 0;
}

int jso_is_obj(json_object *obj) {
    return (obj != NULL);
}

void jso_add_int(json_obj *obj, const char *key, int value) {
    if (obj && obj->data && key) {
        char temp[512];
        int ret = snprintf(temp, sizeof(temp), "{\"%s\":%d}", key, value);
        if (ret > 0 && ret < sizeof(temp) && ret < obj->capacity) {
            strcpy(obj->data, temp);
        }
    }
}

const char* json_object_to_json_string_for_obj(json_obj *obj) {
    return (obj && obj->data) ? obj->data : "{}";
}

int jso_obj_get_int(json_obj *obj, const char *key, int *value) {
    if (!obj || !obj->data || !key || !value) return -1;
    
    char search[128];
    int ret = snprintf(search, sizeof(search), "\"%s\":", key);
    if (ret < 0 || ret >= sizeof(search)) return -1;
    
    char *found = strstr(obj->data, search);
    if (found) {
        found += strlen(search);
        *value = atoi(found);
        return 0;
    }
    return -1;
}

json_obj* jso_from_string(const char *json_str) {
    return (json_obj*) json_tokener_parse(json_str);
}

json_obj* exec_and_read_json(const char *command, char *buf, int bufsize) {
    FILE *res;

    if (!command || !buf || bufsize <= 0)
        return NULL;

    write(1, command, strlen(command));
    puts("\n");
    res = popen(command, "r");
    if (!res)
        return NULL;

    memset(buf, 0, bufsize);
    fread(buf, 1, bufsize - 1, res);
    pclose(res);

    return jso_from_string(buf);
}

void slp_http_response_json(struct client *cl, const char *json_str) {
    if (!json_str) json_str = "{}";
    send_response(cl, 200, "application/json", json_str);
}

int slp_http_response_err_code(struct client *cl, int error_code) {
    char response_body[256];
    const char *message;
    
    switch (error_code) {
        case -40209: message = "Invalid request format"; break;
        case -40101: message = "Operation failed"; break;
        default: message = "Unknown error"; break;
    }
    
    snprintf(response_body, sizeof(response_body), 
        "{\"error_code\":%d,\"message\":\"%s\"}", error_code, message);
    send_response(cl, 200, "application/json", response_body);
    return 0;
}

int setLanguage(struct client *cl, json_object *req) {
    char command[512];
    char buf[512];
    int err_code_array[3] = {0};
    json_obj *result_obj;
    json_obj *response_obj;
    const char *response_str;

    memset(command, 0, sizeof(command));
    memset(buf, 0, sizeof(buf));

    if (!jso_is_obj(req)) {
        return slp_http_response_err_code(cl, -40209);
    }

    const char *args = json_object_to_json_string(req);
    if (!args) args = "{}";

    snprintf(command, sizeof(command), "./setlang.sh '%s'", args);

    result_obj = exec_and_read_json(command, buf, sizeof(buf));
    if (!result_obj) {
        return slp_http_response_err_code(cl, -40101);
    }

    if (jso_obj_get_int(result_obj, "err_code", err_code_array) < 0 || err_code_array[0] != 0) {
        jso_free_obj(result_obj);
        return slp_http_response_err_code(cl, -40101);
    }

    jso_free_obj(result_obj);

    response_obj = jso_new_obj();
    if (!response_obj) {
        return slp_http_response_err_code(cl, -40101);
    }

    jso_add_int(response_obj, "error_code", err_code_array[0]);
    response_str = json_object_to_json_string_for_obj(response_obj);
    slp_http_response_json(cl, response_str);
    return jso_free_obj(response_obj);
}

int user_exists(const char *username) {
    return (username != NULL && strlen(username) > 0 && strlen(username) < MAX_USERNAME_LEN);
}

int check_password(const char *username, const char *password) {
    if (!username || !password) return 0;
    return (strcmp(username, "admin") == 0 && strcmp(password, "admin") == 0);
}

void send_auth_success(struct client *cl, const char *username, const char *role) {
    if (!username) username = "unknown";
    if (!role) role = "user";
    
    char response_body[512];
    snprintf(response_body, sizeof(response_body), 
        "{\"error_code\":0,\"message\":\"Login successful\",\"user\":\"%.32s\",\"role\":\"%.32s\"}", 
        username, role);
    send_response(cl, 200, "application/json", response_body);
}

void send_unauth_resp(struct client *cl, int error_code) {
    char response_body[256];
    const char *message;
    
    switch (error_code) {
        case -40209: message = "Invalid request format"; break;
        case -60502: message = "User does not exist"; break;
        case -40404: message = "Device is locked"; break;
        case -40410: message = "Invalid nonce"; break;
        case -40401: message = "Invalid credentials"; break;
        case -90000: message = "FFS bind error"; break;
        default: message = "Authentication failed"; break;
    }
    
    snprintf(response_body, sizeof(response_body), 
        "{\"error_code\":%d,\"message\":\"%s\"}", error_code, message);
    send_response(cl, 401, "application/json", response_body);
}

int do_login(struct client *cl, json_object *req) {
    if (!req) {
        return slp_http_response_err_code(cl, -40209);
    }
    
    json_object *username_obj = NULL;
    const char *username = NULL;
    if (json_object_object_get_ex(req, "username", &username_obj)) {
        username = json_object_get_string(username_obj);
    }
    
    if (!username || !user_exists(username)) {
        send_unauth_resp(cl, -60502);
        return -1;
    }
    
    json_object *password_obj = NULL;
    const char *password = NULL;
    if (json_object_object_get_ex(req, "password", &password_obj)) {
        password = json_object_get_string(password_obj);
    }
    
    if (username && password && check_password(username, password)) {
        send_auth_success(cl, username, "admin");
        return 0;
    }
    
    send_unauth_resp(cl, -40401);
    return -1;
}

void handle_client(struct client *cl) {
    char *method, *path, *version;
    char buffer_copy[BUFFER_SIZE];
    
    if (!cl || cl->fd < 0) return;
    
    cl->buffer[BUFFER_SIZE - 1] = '\0';
    
    if (strlen(cl->buffer) < 14) {
        send_error(cl, 400, "Request too short");
        return;
    }
    
    strncpy(buffer_copy, cl->buffer, BUFFER_SIZE - 1);
    buffer_copy[BUFFER_SIZE - 1] = '\0';
    
    char *line = buffer_copy;
    
    method = strtok(line, " ");
    path = strtok(NULL, " ");
    version = strtok(NULL, "\r\n");

    if (!method || !path || !version) {
        send_error(cl, 400, "Invalid request");
        return;
    }
    
    if (strcmp(method, "GET") != 0 && strcmp(method, "POST") != 0) {
        send_error(cl, 405, "Method not allowed");
        return;
    }
    
    if (strlen(path) > 256) {
        send_error(cl, 414, "URI too long");
        return;
    }
    
    if (strcmp(path, "/login") == 0 && strcmp(method, "POST") == 0) {
        char *body = strstr(cl->buffer, "\r\n\r\n");
        if (!body) {
            send_error(cl, 400, "No request body");
            return;
        }
        body += 4;
        
        size_t body_len = strlen(body);
        if (body_len == 0) {
            send_error(cl, 400, "Empty request body");
            return;
        }
        
        if (body_len > MAX_JSON_SIZE) {
            send_error(cl, 413, "Request entity too large");
            return;
        }
        
        if (strncmp(body, "{", 1) == 0) {
            json_object *req = json_tokener_parse(body);
            if (req) {
                do_login(cl, req);
                json_object_put(req);
            } else {
                send_error(cl, 400, "Invalid JSON");
            }
        } else if (strstr(cl->buffer, "Content-Disposition: form-data")) {
            char username[MAX_USERNAME_LEN + 1] = {0};
            char password[MAX_PASSWORD_LEN + 1] = {0};
            
            if (safe_parse_multipart_field(body, "username", username, sizeof(username)) == 0 &&
                safe_parse_multipart_field(body, "password", password, sizeof(password)) == 0) {
                
                char json_body[512];
                snprintf(json_body, sizeof(json_body), 
                    "{\"username\":\"%.31s\",\"password\":\"%.63s\"}", 
                    username, password);
                
                memset(username, 0, sizeof(username));
                memset(password, 0, sizeof(password));
                
                json_object *req = json_tokener_parse(json_body);
                if (req) {
                    do_login(cl, req);
                    json_object_put(req);
                } else {
                    send_error(cl, 400, "Failed to create JSON");
                }
                
                memset(json_body, 0, sizeof(json_body));
            } else {
                send_error(cl, 400, "Failed to parse form data");
            }
        } else {
            char username[MAX_USERNAME_LEN + 1] = {0};
            char password[MAX_PASSWORD_LEN + 1] = {0};
            
            if (safe_parse_urlencoded_field(body, "username", username, sizeof(username)) == 0 &&
                safe_parse_urlencoded_field(body, "password", password, sizeof(password)) == 0) {
                
                char json_body[512];
                snprintf(json_body, sizeof(json_body), 
                    "{\"username\":\"%.31s\",\"password\":\"%.63s\"}", 
                    username, password);
                
                memset(username, 0, sizeof(username));
                memset(password, 0, sizeof(password));
                
                json_object *req = json_tokener_parse(json_body);
                if (req) {
                    do_login(cl, req);
                    json_object_put(req);
                } else {
                    send_error(cl, 400, "Failed to create JSON");
                }
                
                memset(json_body, 0, sizeof(json_body));
            } else {
                send_error(cl, 400, "Failed to parse form data");
            }
        }
    }
    else if (strcmp(path, "/") == 0 && strcmp(method, "POST") == 0) {
        char *body = strstr(cl->buffer, "\r\n\r\n");
        if (body) {
            body += 4;
            
            if (strlen(body) > 0 && strlen(body) <= MAX_JSON_SIZE) {
                json_object *req = json_tokener_parse(body);
                if (req) {
                    setLanguage(cl, req);
                    json_object_put(req);
                } else {
                    slp_http_response_err_code(cl, -40209);
                }
            } else {
                slp_http_response_err_code(cl, -40209);
            }
        } else {
            slp_http_response_err_code(cl, -40209);
        }
    }
    else if (strcmp(path, "/func.js") == 0 && strcmp(method, "GET") == 0) {
        char *js_content = load_file("func.js");
        if (js_content) {
            send_response(cl, 200, "application/javascript", js_content);
            free(js_content);
        } else {
            send_error(cl, 404, "JavaScript file not found");
        }
    }
    else if ((strcmp(path, "/") == 0 || strcmp(path, "/index.html") == 0) && strcmp(method, "GET") == 0) {
        char *html_content = load_file("login.html");
        if (html_content) {
            send_response(cl, 200, "text/html", html_content);
            free(html_content);
        } else {
            const char *fallback_html = 
                "<!DOCTYPE html>\n"
                "<html><head>"
                "<title>Developing...</title>"
                "<meta charset=\"UTF-8\">"
                "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">"
                "</head>\n"
                "<body>\n"
                "<h1>Device Login</h1>\n"
                "<form method='post' action='/login'>\n"
                "<p>Username: <input type='text' name='username' value='admin' maxlength='31'></p>\n"
                "<p>Password: <input type='password' name='password' value='admin' maxlength='63'></p>\n"
                "<p><input type='submit' value='Login'></p>\n"
                "</form>\n"
                "<p><strong>Note:</strong> login.html file not found. Using fallback.</p>\n"
                "</body></html>";
            send_response(cl, 200, "text/html", fallback_html);
        }
    }
    else {
        send_error(cl, 404, "Not Found");
    }
}

int main(int argc, char *argv[]) {
    int port = 8080;
    char *bind_addr = NULL;
    int opt;

    memset(&conf, 0, sizeof(conf));
    strcpy(conf.docroot, ".");
    strcpy(conf.realm, "Protected Area");
    conf.max_requests = 3;
    conf.network_timeout = 30;
    set_working_directory_to_executable_dir(argv[0]);

    while ((opt = getopt(argc, argv, "p:h:c:fSDR")) != -1) {
        switch (opt) {
            case 'p':
                port = atoi(optarg);
                if (port <= 0 || port > 65535) {
                    fprintf(stderr, "Invalid port number\n");
                    exit(1);
                }
                break;
            case 'h':
                if (strlen(optarg) >= sizeof(conf.docroot)) {
                    fprintf(stderr, "Document root path too long\n");
                    exit(1);
                }
                strncpy(conf.docroot, optarg, sizeof(conf.docroot) - 1);
                break;
            case 'f':
                conf.daemon_mode = 0;
                break;
            case 'S':
                conf.no_symlinks = 1;
                break;
            case 'D':
                conf.no_dirlists = 1;
                break;
            case 'R':
                conf.rfc1918_filter = 1;
                break;
            default:
                printf("Usage: %s [-p port] [-h docroot] [-f] [-S] [-D] [-R]\n", argv[0]);
                exit(1);
        }
    }

    if (port < 1024 && getuid() != 0) {
        fprintf(stderr, "Need root privileges to bind to port %d\n", port);
        exit(1);
    }
    signal(SIGINT, signal_handler);
    signal(SIGTERM, signal_handler);
    signal(SIGPIPE, SIG_IGN);

    memset(clients, 0, sizeof(clients));

    server_fd = create_server_socket(bind_addr, port);
    if (server_fd < 0) {
        exit(1);
    }

    while (running) {
        fd_set readfds;
        int max_fd = server_fd;
        struct timeval timeout = {1, 0};

        FD_ZERO(&readfds);
        FD_SET(server_fd, &readfds);

        for (int i = 0; i < MAX_CLIENTS; i++) {
            if (clients[i].fd > 0) {
                FD_SET(clients[i].fd, &readfds);
                if (clients[i].fd > max_fd) {
                    max_fd = clients[i].fd;
                }
            }
        }

        int activity = select(max_fd + 1, &readfds, NULL, NULL, &timeout);
        if (activity < 0 && errno != EINTR) {
            perror("select");
            break;
        }

        if (FD_ISSET(server_fd, &readfds)) {
            struct sockaddr_in client_addr;
            socklen_t client_len = sizeof(client_addr);
            int client_fd = accept(server_fd, (struct sockaddr*)&client_addr, &client_len);
            
            if (client_fd >= 0) {
                struct timeval tv;
                tv.tv_sec = conf.network_timeout;
                tv.tv_usec = 0;
                setsockopt(client_fd, SOL_SOCKET, SO_RCVTIMEO, (const char*)&tv, sizeof tv);
                setsockopt(client_fd, SOL_SOCKET, SO_SNDTIMEO, (const char*)&tv, sizeof tv);
                
                int slot = -1;
                for (int i = 0; i < MAX_CLIENTS; i++) {
                    if (clients[i].fd == 0) {
                        slot = i;
                        break;
                    }
                }

                if (slot >= 0) {
                    clients[slot].fd = client_fd;
                    clients[slot].addr = client_addr;
                    clients[slot].connect_time = time(NULL);
                    clients[slot].buffer_len = 0;
                    memset(clients[slot].buffer, 0, BUFFER_SIZE);
                } else {
                    close(client_fd);
                }
            }
        }

        time_t current_time = time(NULL);
        for (int i = 0; i < MAX_CLIENTS; i++) {
            if (clients[i].fd > 0) {
                if (current_time - clients[i].connect_time > conf.network_timeout) {
                    close(clients[i].fd);
                    memset(&clients[i], 0, sizeof(struct client));
                    continue;
                }
                
                if (FD_ISSET(clients[i].fd, &readfds)) {
                    int bytes = recv(clients[i].fd, clients[i].buffer, BUFFER_SIZE - 1, 0);
                    if (bytes > 0) {
                        clients[i].buffer[bytes] = '\0';
                        clients[i].buffer_len = bytes;
                        
                        handle_client(&clients[i]);
                    }
                    
                    close(clients[i].fd);
                    memset(&clients[i], 0, sizeof(struct client));
                }
            }
        }
    }

    if (server_fd >= 0) {
        close(server_fd);
    }
    
    for (int i = 0; i < MAX_CLIENTS; i++) {
        if (clients[i].fd > 0) {
            close(clients[i].fd);
        }
        memset(&clients[i], 0, sizeof(struct client));
    }

    return 0;
}
