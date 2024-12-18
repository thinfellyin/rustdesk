#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;

#define VIDEO_QUEUE_SIZE 120

#define AUDIO_BUFFER_MS 3000

#define CLIPBOARD_INTERVAL 333

#define ERR_SUCCESS 0

#define ERR_RUSTDESK_HANDLE_BASE 10000

#define ERR_PLUGIN_LOAD 10001

#define ERR_PLUGIN_MSG_INIT 10101

#define ERR_PLUGIN_MSG_INIT_INVALID 10102

#define ERR_PLUGIN_MSG_GET_LOCAL_PEER_ID 10103

#define ERR_PLUGIN_SIGNATURE_NOT_VERIFIED 10104

#define ERR_PLUGIN_SIGNATURE_VERIFICATION_FAILED 10105

#define ERR_CALL_UNIMPLEMENTED 10201

#define ERR_CALL_INVALID_METHOD 10202

#define ERR_CALL_NOT_SUPPORTED_METHOD 10203

#define ERR_CALL_INVALID_PEER 10204

#define ERR_CALL_INVALID_ARGS 10301

#define ERR_PEER_ID_MISMATCH 10302

#define ERR_CALL_CONFIG_VALUE 10303

#define ERR_NOT_HANDLED 10401

#define ERR_CALLBACK_HANDLE_BASE 20000

#define ERR_CALLBACK_PLUGIN_ID 20001

#define ERR_CALLBACK_INVALID_ARGS 20002

#define ERR_CALLBACK_INVALID_MSG 20003

#define ERR_CALLBACK_TARGET 20004

#define ERR_CALLBACK_TARGET_TYPE 20005

#define ERR_CALLBACK_PEER_NOT_FOUND 20006

#define ERR_CALLBACK_FAILED 21001

#define ERR_PLUGIN_HANDLE_BASE 30000

#define EER_CALL_FAILED 30021

#define ERR_PEER_ON_FAILED 40012

#define ERR_PEER_OFF_FAILED 40012

#define INVALID_PRIVACY_MODE_CONN_ID 0

typedef struct DartCObject DartCObject;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct DartCObject *WireSyncReturn;

typedef struct wire_int_32_list {
  int32_t *ptr;
  int32_t len;
} wire_int_32_list;

typedef struct wire_StringList {
  struct wire_uint_8_list **ptr;
  int32_t len;
} wire_StringList;

#define CONFIG_INPUT_SOURCE_DEFAULT CONFIG_INPUT_SOURCE_1

#define INJECTED_PROCESS_EXE WIN_TOPMOST_INJECTED_PROCESS_EXE

void store_dart_post_cobject(DartPostCObjectFnType ptr);

Dart_Handle get_dart_object(uintptr_t ptr);

void drop_dart_object(uintptr_t ptr);

uintptr_t new_dart_opaque(Dart_Handle handle);

intptr_t init_frb_dart_api_dl(void *obj);

void wire_start_global_event_stream(int64_t port_, struct wire_uint_8_list *app_type);

void wire_stop_global_event_stream(int64_t port_, struct wire_uint_8_list *app_type);

void wire_host_stop_system_key_propagate(int64_t port_, bool _stopped);

WireSyncReturn wire_peer_get_default_sessions_count(struct wire_uint_8_list *id);

WireSyncReturn wire_session_add_existed_sync(struct wire_uint_8_list *id,
                                             struct wire_uint_8_list *session_id,
                                             struct wire_int_32_list *displays);

WireSyncReturn wire_session_add_sync(struct wire_uint_8_list *session_id,
                                     struct wire_uint_8_list *id,
                                     bool is_file_transfer,
                                     bool is_port_forward,
                                     bool is_rdp,
                                     struct wire_uint_8_list *switch_uuid,
                                     bool force_relay,
                                     struct wire_uint_8_list *password,
                                     bool is_shared_password,
                                     struct wire_uint_8_list *conn_token);

void wire_session_start(int64_t port_,
                        struct wire_uint_8_list *session_id,
                        struct wire_uint_8_list *id);

void wire_session_start_with_displays(int64_t port_,
                                      struct wire_uint_8_list *session_id,
                                      struct wire_uint_8_list *id,
                                      struct wire_int_32_list *displays);

void wire_session_get_remember(int64_t port_, struct wire_uint_8_list *session_id);

void wire_session_get_toggle_option(int64_t port_,
                                    struct wire_uint_8_list *session_id,
                                    struct wire_uint_8_list *arg);

WireSyncReturn wire_session_get_toggle_option_sync(struct wire_uint_8_list *session_id,
                                                   struct wire_uint_8_list *arg);

void wire_session_get_option(int64_t port_,
                             struct wire_uint_8_list *session_id,
                             struct wire_uint_8_list *arg);

void wire_session_login(int64_t port_,
                        struct wire_uint_8_list *session_id,
                        struct wire_uint_8_list *os_username,
                        struct wire_uint_8_list *os_password,
                        struct wire_uint_8_list *password,
                        bool remember);

void wire_session_send2fa(int64_t port_,
                          struct wire_uint_8_list *session_id,
                          struct wire_uint_8_list *code,
                          bool trust_this_device);

WireSyncReturn wire_session_get_enable_trusted_devices(struct wire_uint_8_list *session_id);

void wire_session_close(int64_t port_, struct wire_uint_8_list *session_id);

void wire_session_refresh(int64_t port_, struct wire_uint_8_list *session_id, uintptr_t display);

WireSyncReturn wire_session_is_multi_ui_session(struct wire_uint_8_list *session_id);

void wire_session_record_screen(int64_t port_, struct wire_uint_8_list *session_id, bool start);

WireSyncReturn wire_session_get_is_recording(struct wire_uint_8_list *session_id);

void wire_session_reconnect(int64_t port_, struct wire_uint_8_list *session_id, bool force_relay);

void wire_session_toggle_option(int64_t port_,
                                struct wire_uint_8_list *session_id,
                                struct wire_uint_8_list *value);

void wire_session_toggle_privacy_mode(int64_t port_,
                                      struct wire_uint_8_list *session_id,
                                      struct wire_uint_8_list *impl_key,
                                      bool on);

void wire_session_get_flutter_option(int64_t port_,
                                     struct wire_uint_8_list *session_id,
                                     struct wire_uint_8_list *k);

void wire_session_set_flutter_option(int64_t port_,
                                     struct wire_uint_8_list *session_id,
                                     struct wire_uint_8_list *k,
                                     struct wire_uint_8_list *v);

WireSyncReturn wire_get_next_texture_key(void);

WireSyncReturn wire_get_local_flutter_option(struct wire_uint_8_list *k);

void wire_set_local_flutter_option(int64_t port_,
                                   struct wire_uint_8_list *k,
                                   struct wire_uint_8_list *v);

WireSyncReturn wire_get_local_kb_layout_type(void);

void wire_set_local_kb_layout_type(int64_t port_, struct wire_uint_8_list *kb_layout_type);

void wire_session_get_view_style(int64_t port_, struct wire_uint_8_list *session_id);

void wire_session_set_view_style(int64_t port_,
                                 struct wire_uint_8_list *session_id,
                                 struct wire_uint_8_list *value);

void wire_session_get_scroll_style(int64_t port_, struct wire_uint_8_list *session_id);

void wire_session_set_scroll_style(int64_t port_,
                                   struct wire_uint_8_list *session_id,
                                   struct wire_uint_8_list *value);

void wire_session_get_image_quality(int64_t port_, struct wire_uint_8_list *session_id);

void wire_session_set_image_quality(int64_t port_,
                                    struct wire_uint_8_list *session_id,
                                    struct wire_uint_8_list *value);

void wire_session_get_keyboard_mode(int64_t port_, struct wire_uint_8_list *session_id);

void wire_session_set_keyboard_mode(int64_t port_,
                                    struct wire_uint_8_list *session_id,
                                    struct wire_uint_8_list *value);

WireSyncReturn wire_session_get_reverse_mouse_wheel_sync(struct wire_uint_8_list *session_id);

void wire_session_set_reverse_mouse_wheel(int64_t port_,
                                          struct wire_uint_8_list *session_id,
                                          struct wire_uint_8_list *value);

WireSyncReturn wire_session_get_displays_as_individual_windows(struct wire_uint_8_list *session_id);

void wire_session_set_displays_as_individual_windows(int64_t port_,
                                                     struct wire_uint_8_list *session_id,
                                                     struct wire_uint_8_list *value);

WireSyncReturn wire_session_get_use_all_my_displays_for_the_remote_session(struct wire_uint_8_list *session_id);

void wire_session_set_use_all_my_displays_for_the_remote_session(int64_t port_,
                                                                 struct wire_uint_8_list *session_id,
                                                                 struct wire_uint_8_list *value);

void wire_session_get_custom_image_quality(int64_t port_, struct wire_uint_8_list *session_id);

WireSyncReturn wire_session_is_keyboard_mode_supported(struct wire_uint_8_list *session_id,
                                                       struct wire_uint_8_list *mode);

void wire_session_set_custom_image_quality(int64_t port_,
                                           struct wire_uint_8_list *session_id,
                                           int32_t value);

void wire_session_set_custom_fps(int64_t port_, struct wire_uint_8_list *session_id, int32_t fps);

void wire_session_lock_screen(int64_t port_, struct wire_uint_8_list *session_id);

void wire_session_ctrl_alt_del(int64_t port_, struct wire_uint_8_list *session_id);

void wire_session_switch_display(int64_t port_,
                                 bool is_desktop,
                                 struct wire_uint_8_list *session_id,
                                 struct wire_int_32_list *value);

void wire_session_handle_flutter_key_event(int64_t port_,
                                           struct wire_uint_8_list *session_id,
                                           struct wire_uint_8_list *character,
                                           int32_t usb_hid,
                                           int32_t lock_modes,
                                           bool down_or_up);

void wire_session_handle_flutter_raw_key_event(int64_t port_,
                                               struct wire_uint_8_list *session_id,
                                               struct wire_uint_8_list *name,
                                               int32_t platform_code,
                                               int32_t position_code,
                                               int32_t lock_modes,
                                               bool down_or_up);

WireSyncReturn wire_session_enter_or_leave(struct wire_uint_8_list *_session_id, bool _enter);

void wire_session_input_key(int64_t port_,
                            struct wire_uint_8_list *session_id,
                            struct wire_uint_8_list *name,
                            bool down,
                            bool press,
                            bool alt,
                            bool ctrl,
                            bool shift,
                            bool command);

void wire_session_input_string(int64_t port_,
                               struct wire_uint_8_list *session_id,
                               struct wire_uint_8_list *value);

void wire_session_send_chat(int64_t port_,
                            struct wire_uint_8_list *session_id,
                            struct wire_uint_8_list *text);

void wire_session_peer_option(int64_t port_,
                              struct wire_uint_8_list *session_id,
                              struct wire_uint_8_list *name,
                              struct wire_uint_8_list *value);

void wire_session_get_peer_option(int64_t port_,
                                  struct wire_uint_8_list *session_id,
                                  struct wire_uint_8_list *name);

void wire_session_input_os_password(int64_t port_,
                                    struct wire_uint_8_list *session_id,
                                    struct wire_uint_8_list *value);

void wire_session_read_remote_dir(int64_t port_,
                                  struct wire_uint_8_list *session_id,
                                  struct wire_uint_8_list *path,
                                  bool include_hidden);

void wire_session_send_files(int64_t port_,
                             struct wire_uint_8_list *session_id,
                             int32_t act_id,
                             struct wire_uint_8_list *path,
                             struct wire_uint_8_list *to,
                             int32_t file_num,
                             bool include_hidden,
                             bool is_remote,
                             bool _is_dir);

void wire_session_set_confirm_override_file(int64_t port_,
                                            struct wire_uint_8_list *session_id,
                                            int32_t act_id,
                                            int32_t file_num,
                                            bool need_override,
                                            bool remember,
                                            bool is_upload);

void wire_session_remove_file(int64_t port_,
                              struct wire_uint_8_list *session_id,
                              int32_t act_id,
                              struct wire_uint_8_list *path,
                              int32_t file_num,
                              bool is_remote);

void wire_session_read_dir_to_remove_recursive(int64_t port_,
                                               struct wire_uint_8_list *session_id,
                                               int32_t act_id,
                                               struct wire_uint_8_list *path,
                                               bool is_remote,
                                               bool show_hidden);

void wire_session_remove_all_empty_dirs(int64_t port_,
                                        struct wire_uint_8_list *session_id,
                                        int32_t act_id,
                                        struct wire_uint_8_list *path,
                                        bool is_remote);

void wire_session_cancel_job(int64_t port_, struct wire_uint_8_list *session_id, int32_t act_id);

void wire_session_create_dir(int64_t port_,
                             struct wire_uint_8_list *session_id,
                             int32_t act_id,
                             struct wire_uint_8_list *path,
                             bool is_remote);

void wire_session_read_local_dir_sync(int64_t port_,
                                      struct wire_uint_8_list *_session_id,
                                      struct wire_uint_8_list *path,
                                      bool show_hidden);

void wire_session_read_local_empty_dirs_recursive_sync(int64_t port_,
                                                       struct wire_uint_8_list *_session_id,
                                                       struct wire_uint_8_list *path,
                                                       bool include_hidden);

void wire_session_read_remote_empty_dirs_recursive_sync(int64_t port_,
                                                        struct wire_uint_8_list *session_id,
                                                        struct wire_uint_8_list *path,
                                                        bool include_hidden);

void wire_session_get_platform(int64_t port_, struct wire_uint_8_list *session_id, bool is_remote);

void wire_session_load_last_transfer_jobs(int64_t port_, struct wire_uint_8_list *session_id);

void wire_session_add_job(int64_t port_,
                          struct wire_uint_8_list *session_id,
                          int32_t act_id,
                          struct wire_uint_8_list *path,
                          struct wire_uint_8_list *to,
                          int32_t file_num,
                          bool include_hidden,
                          bool is_remote);

void wire_session_resume_job(int64_t port_,
                             struct wire_uint_8_list *session_id,
                             int32_t act_id,
                             bool is_remote);

void wire_session_rename_file(int64_t port_,
                              struct wire_uint_8_list *session_id,
                              int32_t act_id,
                              struct wire_uint_8_list *path,
                              struct wire_uint_8_list *new_name,
                              bool is_remote);

void wire_session_elevate_direct(int64_t port_, struct wire_uint_8_list *session_id);

void wire_session_elevate_with_logon(int64_t port_,
                                     struct wire_uint_8_list *session_id,
                                     struct wire_uint_8_list *username,
                                     struct wire_uint_8_list *password);

void wire_session_switch_sides(int64_t port_, struct wire_uint_8_list *session_id);

void wire_session_change_resolution(int64_t port_,
                                    struct wire_uint_8_list *session_id,
                                    int32_t display,
                                    int32_t width,
                                    int32_t height);

void wire_session_set_size(int64_t port_,
                           struct wire_uint_8_list *session_id,
                           uintptr_t display,
                           uintptr_t width,
                           uintptr_t height);

void wire_session_send_selected_session_id(int64_t port_,
                                           struct wire_uint_8_list *session_id,
                                           struct wire_uint_8_list *sid);

void wire_main_get_sound_inputs(int64_t port_);

WireSyncReturn wire_main_get_login_device_info(void);

void wire_main_change_id(int64_t port_, struct wire_uint_8_list *new_id);

void wire_main_get_async_status(int64_t port_);

void wire_main_get_http_status(int64_t port_, struct wire_uint_8_list *url);

void wire_main_get_option(int64_t port_, struct wire_uint_8_list *key);

WireSyncReturn wire_main_get_option_sync(struct wire_uint_8_list *key);

void wire_main_get_error(int64_t port_);

WireSyncReturn wire_main_show_option(struct wire_uint_8_list *_key);

void wire_main_set_option(int64_t port_,
                          struct wire_uint_8_list *key,
                          struct wire_uint_8_list *value);

void wire_main_get_options(int64_t port_);

WireSyncReturn wire_main_get_options_sync(void);

void wire_main_set_options(int64_t port_, struct wire_uint_8_list *json);

void wire_main_test_if_valid_server(int64_t port_,
                                    struct wire_uint_8_list *server,
                                    bool test_with_proxy);

void wire_main_set_socks(int64_t port_,
                         struct wire_uint_8_list *proxy,
                         struct wire_uint_8_list *username,
                         struct wire_uint_8_list *password);

void wire_main_get_proxy_status(int64_t port_);

void wire_main_get_socks(int64_t port_);

void wire_main_get_app_name(int64_t port_);

WireSyncReturn wire_main_get_app_name_sync(void);

WireSyncReturn wire_main_uri_prefix_sync(void);

void wire_main_get_license(int64_t port_);

void wire_main_get_version(int64_t port_);

void wire_main_get_fav(int64_t port_);

void wire_main_store_fav(int64_t port_, struct wire_StringList *favs);

WireSyncReturn wire_main_get_peer_sync(struct wire_uint_8_list *id);

void wire_main_get_lan_peers(int64_t port_);

void wire_main_get_connect_status(int64_t port_);

void wire_main_check_connect_status(int64_t port_);

void wire_main_is_using_public_server(int64_t port_);

void wire_main_discover(int64_t port_);

void wire_main_get_api_server(int64_t port_);

void wire_main_http_request(int64_t port_,
                            struct wire_uint_8_list *url,
                            struct wire_uint_8_list *method,
                            struct wire_uint_8_list *body,
                            struct wire_uint_8_list *header);

WireSyncReturn wire_main_get_local_option(struct wire_uint_8_list *key);

WireSyncReturn wire_main_get_use_texture_render(void);

WireSyncReturn wire_main_get_env(struct wire_uint_8_list *key);

void wire_main_set_local_option(int64_t port_,
                                struct wire_uint_8_list *key,
                                struct wire_uint_8_list *value);

void wire_main_handle_wayland_screencast_restore_token(int64_t port_,
                                                       struct wire_uint_8_list *_key,
                                                       struct wire_uint_8_list *_value);

WireSyncReturn wire_main_get_input_source(void);

void wire_main_set_input_source(int64_t port_,
                                struct wire_uint_8_list *session_id,
                                struct wire_uint_8_list *value);

void wire_main_get_my_id(int64_t port_);

void wire_main_get_uuid(int64_t port_);

void wire_main_get_peer_option(int64_t port_,
                               struct wire_uint_8_list *id,
                               struct wire_uint_8_list *key);

WireSyncReturn wire_main_get_peer_option_sync(struct wire_uint_8_list *id,
                                              struct wire_uint_8_list *key);

WireSyncReturn wire_main_get_peer_flutter_option_sync(struct wire_uint_8_list *id,
                                                      struct wire_uint_8_list *k);

WireSyncReturn wire_main_set_peer_flutter_option_sync(struct wire_uint_8_list *id,
                                                      struct wire_uint_8_list *k,
                                                      struct wire_uint_8_list *v);

void wire_main_set_peer_option(int64_t port_,
                               struct wire_uint_8_list *id,
                               struct wire_uint_8_list *key,
                               struct wire_uint_8_list *value);

WireSyncReturn wire_main_set_peer_option_sync(struct wire_uint_8_list *id,
                                              struct wire_uint_8_list *key,
                                              struct wire_uint_8_list *value);

void wire_main_set_peer_alias(int64_t port_,
                              struct wire_uint_8_list *id,
                              struct wire_uint_8_list *alias);

void wire_main_get_new_stored_peers(int64_t port_);

void wire_main_forget_password(int64_t port_, struct wire_uint_8_list *id);

void wire_main_peer_has_password(int64_t port_, struct wire_uint_8_list *id);

void wire_main_peer_exists(int64_t port_, struct wire_uint_8_list *id);

void wire_main_load_recent_peers(int64_t port_);

WireSyncReturn wire_main_load_recent_peers_sync(void);

WireSyncReturn wire_main_load_lan_peers_sync(void);

void wire_main_load_recent_peers_for_ab(int64_t port_, struct wire_uint_8_list *filter);

void wire_main_load_fav_peers(int64_t port_);

void wire_main_load_lan_peers(int64_t port_);

void wire_main_remove_discovered(int64_t port_, struct wire_uint_8_list *id);

void wire_main_change_theme(int64_t port_, struct wire_uint_8_list *dark);

void wire_main_change_language(int64_t port_, struct wire_uint_8_list *lang);

WireSyncReturn wire_main_video_save_directory(bool root);

void wire_main_set_user_default_option(int64_t port_,
                                       struct wire_uint_8_list *key,
                                       struct wire_uint_8_list *value);

WireSyncReturn wire_main_get_user_default_option(struct wire_uint_8_list *key);

void wire_main_handle_relay_id(int64_t port_, struct wire_uint_8_list *id);

WireSyncReturn wire_main_is_option_fixed(struct wire_uint_8_list *key);

WireSyncReturn wire_main_get_main_display(void);

WireSyncReturn wire_main_get_displays(void);

void wire_session_add_port_forward(int64_t port_,
                                   struct wire_uint_8_list *session_id,
                                   int32_t local_port,
                                   struct wire_uint_8_list *remote_host,
                                   int32_t remote_port);

void wire_session_remove_port_forward(int64_t port_,
                                      struct wire_uint_8_list *session_id,
                                      int32_t local_port);

void wire_session_new_rdp(int64_t port_, struct wire_uint_8_list *session_id);

void wire_session_request_voice_call(int64_t port_, struct wire_uint_8_list *session_id);

void wire_session_close_voice_call(int64_t port_, struct wire_uint_8_list *session_id);

WireSyncReturn wire_session_get_conn_token(struct wire_uint_8_list *session_id);

void wire_cm_handle_incoming_voice_call(int64_t port_, int32_t id, bool accept);

void wire_cm_close_voice_call(int64_t port_, int32_t id);

void wire_set_voice_call_input_device(int64_t port_, bool _is_cm, struct wire_uint_8_list *_device);

void wire_get_voice_call_input_device(int64_t port_, bool _is_cm);

void wire_main_get_last_remote_id(int64_t port_);

void wire_main_get_software_update_url(int64_t port_);

void wire_main_get_home_dir(int64_t port_);

void wire_main_get_langs(int64_t port_);

void wire_main_get_temporary_password(int64_t port_);

void wire_main_get_permanent_password(int64_t port_);

void wire_main_get_fingerprint(int64_t port_);

void wire_cm_get_clients_state(int64_t port_);

void wire_cm_check_clients_length(int64_t port_, uintptr_t length);

void wire_cm_get_clients_length(int64_t port_);

void wire_main_init(int64_t port_,
                    struct wire_uint_8_list *app_dir,
                    struct wire_uint_8_list *custom_client_config);

void wire_main_device_id(int64_t port_, struct wire_uint_8_list *id);

void wire_main_device_name(int64_t port_, struct wire_uint_8_list *name);

void wire_main_remove_peer(int64_t port_, struct wire_uint_8_list *id);

WireSyncReturn wire_main_has_hwcodec(void);

WireSyncReturn wire_main_has_vram(void);

WireSyncReturn wire_main_supported_hwdecodings(void);

void wire_main_is_root(int64_t port_);

WireSyncReturn wire_get_double_click_time(void);

void wire_main_start_dbus_server(int64_t port_);

void wire_main_save_ab(int64_t port_, struct wire_uint_8_list *json);

void wire_main_clear_ab(int64_t port_);

void wire_main_load_ab(int64_t port_);

void wire_main_save_group(int64_t port_, struct wire_uint_8_list *json);

void wire_main_clear_group(int64_t port_);

void wire_main_load_group(int64_t port_);

void wire_session_send_pointer(int64_t port_,
                               struct wire_uint_8_list *session_id,
                               struct wire_uint_8_list *msg);

void wire_session_send_mouse(int64_t port_,
                             struct wire_uint_8_list *session_id,
                             struct wire_uint_8_list *msg);

void wire_session_restart_remote_device(int64_t port_, struct wire_uint_8_list *session_id);

WireSyncReturn wire_session_get_audit_server_sync(struct wire_uint_8_list *session_id,
                                                  struct wire_uint_8_list *typ);

void wire_session_send_note(int64_t port_,
                            struct wire_uint_8_list *session_id,
                            struct wire_uint_8_list *note);

void wire_session_alternative_codecs(int64_t port_, struct wire_uint_8_list *session_id);

void wire_session_change_prefer_codec(int64_t port_, struct wire_uint_8_list *session_id);

void wire_session_on_waiting_for_image_dialog_show(int64_t port_,
                                                   struct wire_uint_8_list *session_id);

void wire_session_toggle_virtual_display(int64_t port_,
                                         struct wire_uint_8_list *session_id,
                                         int32_t index,
                                         bool on);

void wire_main_set_home_dir(int64_t port_, struct wire_uint_8_list *_home);

WireSyncReturn wire_main_get_data_dir_ios(void);

void wire_main_stop_service(int64_t port_);

void wire_main_start_service(int64_t port_);

void wire_main_update_temporary_password(int64_t port_);

void wire_main_set_permanent_password(int64_t port_, struct wire_uint_8_list *password);

void wire_main_check_super_user_permission(int64_t port_);

WireSyncReturn wire_main_get_unlock_pin(void);

WireSyncReturn wire_main_set_unlock_pin(struct wire_uint_8_list *pin);

void wire_main_check_mouse_time(int64_t port_);

void wire_main_get_mouse_time(int64_t port_);

void wire_main_wol(int64_t port_, struct wire_uint_8_list *id);

void wire_main_create_shortcut(int64_t port_, struct wire_uint_8_list *_id);

void wire_cm_send_chat(int64_t port_, int32_t conn_id, struct wire_uint_8_list *msg);

void wire_cm_login_res(int64_t port_, int32_t conn_id, bool res);

void wire_cm_close_connection(int64_t port_, int32_t conn_id);

void wire_cm_remove_disconnected_connection(int64_t port_, int32_t conn_id);

void wire_cm_check_click_time(int64_t port_, int32_t conn_id);

void wire_cm_get_click_time(int64_t port_);

void wire_cm_switch_permission(int64_t port_,
                               int32_t conn_id,
                               struct wire_uint_8_list *name,
                               bool enabled);

WireSyncReturn wire_cm_can_elevate(void);

void wire_cm_elevate_portable(int64_t port_, int32_t conn_id);

void wire_cm_switch_back(int64_t port_, int32_t conn_id);

void wire_cm_get_config(int64_t port_, struct wire_uint_8_list *name);

void wire_main_get_build_date(int64_t port_);

WireSyncReturn wire_translate(struct wire_uint_8_list *name, struct wire_uint_8_list *locale);

WireSyncReturn wire_session_get_rgba_size(struct wire_uint_8_list *session_id, uintptr_t display);

WireSyncReturn wire_session_next_rgba(struct wire_uint_8_list *session_id, uintptr_t display);

WireSyncReturn wire_session_register_pixelbuffer_texture(struct wire_uint_8_list *session_id,
                                                         uintptr_t display,
                                                         uintptr_t ptr);

WireSyncReturn wire_session_register_gpu_texture(struct wire_uint_8_list *session_id,
                                                 uintptr_t display,
                                                 uintptr_t ptr);

void wire_query_onlines(int64_t port_, struct wire_StringList *ids);

WireSyncReturn wire_version_to_number(struct wire_uint_8_list *v);

void wire_option_synced(int64_t port_);

WireSyncReturn wire_main_is_installed(void);

WireSyncReturn wire_main_init_input_source(void);

WireSyncReturn wire_main_is_installed_lower_version(void);

WireSyncReturn wire_main_is_installed_daemon(bool prompt);

WireSyncReturn wire_main_is_process_trusted(bool prompt);

WireSyncReturn wire_main_is_can_screen_recording(bool prompt);

WireSyncReturn wire_main_is_can_input_monitoring(bool prompt);

WireSyncReturn wire_main_is_share_rdp(void);

void wire_main_set_share_rdp(int64_t port_, bool enable);

WireSyncReturn wire_main_goto_install(void);

WireSyncReturn wire_main_get_new_version(void);

WireSyncReturn wire_main_update_me(void);

void wire_set_cur_session_id(int64_t port_, struct wire_uint_8_list *session_id);

WireSyncReturn wire_install_show_run_without_install(void);

void wire_install_run_without_install(int64_t port_);

void wire_install_install_me(int64_t port_,
                             struct wire_uint_8_list *options,
                             struct wire_uint_8_list *path);

WireSyncReturn wire_install_install_path(void);

WireSyncReturn wire_install_install_options(void);

void wire_main_account_auth(int64_t port_, struct wire_uint_8_list *op, bool remember_me);

void wire_main_account_auth_cancel(int64_t port_);

void wire_main_account_auth_result(int64_t port_);

void wire_main_on_main_window_close(int64_t port_);

WireSyncReturn wire_main_current_is_wayland(void);

WireSyncReturn wire_main_is_login_wayland(void);

WireSyncReturn wire_main_hide_dock(void);

WireSyncReturn wire_main_has_file_clipboard(void);

WireSyncReturn wire_main_has_gpu_texture_render(void);

void wire_cm_init(int64_t port_);

void wire_main_start_ipc_url_server(int64_t port_);

void wire_main_test_wallpaper(int64_t port_, uint64_t _second);

void wire_main_support_remove_wallpaper(int64_t port_);

WireSyncReturn wire_is_incoming_only(void);

WireSyncReturn wire_is_outgoing_only(void);

WireSyncReturn wire_is_custom_client(void);

WireSyncReturn wire_is_disable_settings(void);

WireSyncReturn wire_is_disable_ab(void);

WireSyncReturn wire_is_disable_account(void);

WireSyncReturn wire_is_disable_group_panel(void);

WireSyncReturn wire_is_disable_installation(void);

void wire_is_preset_password(int64_t port_);

WireSyncReturn wire_is_preset_password_mobile_only(void);

void wire_send_url_scheme(int64_t port_, struct wire_uint_8_list *_url);

void wire_plugin_event(int64_t port_,
                       struct wire_uint_8_list *_id,
                       struct wire_uint_8_list *_peer,
                       struct wire_uint_8_list *_event);

void wire_plugin_register_event_stream(int64_t port_, struct wire_uint_8_list *_id);

WireSyncReturn wire_plugin_get_session_option(struct wire_uint_8_list *_id,
                                              struct wire_uint_8_list *_peer,
                                              struct wire_uint_8_list *_key);

void wire_plugin_set_session_option(int64_t port_,
                                    struct wire_uint_8_list *_id,
                                    struct wire_uint_8_list *_peer,
                                    struct wire_uint_8_list *_key,
                                    struct wire_uint_8_list *_value);

WireSyncReturn wire_plugin_get_shared_option(struct wire_uint_8_list *_id,
                                             struct wire_uint_8_list *_key);

void wire_plugin_set_shared_option(int64_t port_,
                                   struct wire_uint_8_list *_id,
                                   struct wire_uint_8_list *_key,
                                   struct wire_uint_8_list *_value);

void wire_plugin_reload(int64_t port_, struct wire_uint_8_list *_id);

WireSyncReturn wire_plugin_enable(struct wire_uint_8_list *_id, bool _v);

WireSyncReturn wire_plugin_is_enabled(struct wire_uint_8_list *_id);

WireSyncReturn wire_plugin_feature_is_enabled(void);

void wire_plugin_sync_ui(int64_t port_, struct wire_uint_8_list *_sync_to);

void wire_plugin_list_reload(int64_t port_);

void wire_plugin_install(int64_t port_, struct wire_uint_8_list *_id, bool _b);

WireSyncReturn wire_is_support_multi_ui_session(struct wire_uint_8_list *version);

WireSyncReturn wire_is_selinux_enforcing(void);

WireSyncReturn wire_main_default_privacy_mode_impl(void);

WireSyncReturn wire_main_supported_privacy_mode_impls(void);

WireSyncReturn wire_main_supported_input_source(void);

void wire_main_generate2fa(int64_t port_);

void wire_main_verify2fa(int64_t port_, struct wire_uint_8_list *code);

WireSyncReturn wire_main_has_valid_2fa_sync(void);

void wire_main_verify_bot(int64_t port_, struct wire_uint_8_list *token);

WireSyncReturn wire_main_has_valid_bot_sync(void);

WireSyncReturn wire_main_get_hard_option(struct wire_uint_8_list *key);

WireSyncReturn wire_main_get_buildin_option(struct wire_uint_8_list *key);

void wire_main_check_hwcodec(int64_t port_);

void wire_main_get_trusted_devices(int64_t port_);

void wire_main_remove_trusted_devices(int64_t port_, struct wire_uint_8_list *json);

void wire_main_clear_trusted_devices(int64_t port_);

WireSyncReturn wire_main_max_encrypt_len(void);

void wire_session_request_new_display_init_msgs(int64_t port_,
                                                struct wire_uint_8_list *session_id,
                                                uintptr_t display);

WireSyncReturn wire_main_audio_support_loopback(void);

struct wire_StringList *new_StringList_0(int32_t len);

struct wire_int_32_list *new_int_32_list_0(int32_t len);

struct wire_uint_8_list *new_uint_8_list_0(int32_t len);

void free_WireSyncReturn(WireSyncReturn ptr);

/**
 * FFI for rustdesk core's main entry.
 * Return true if the app should continue running with UI(possibly Flutter), false if the app should exit.
 */
bool rustdesk_core_main(void);

void handle_applicationShouldOpenUntitledFile(void);

char **rustdesk_core_main_args(int *args_len);

void free_c_args(char **ptr, int len);

int32_t get_rustdesk_app_name(uint16_t *buffer, int32_t length);

const uint8_t *session_get_rgba(const uint32_t *session_uuid_str, uintptr_t display);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_start_global_event_stream);
    dummy_var ^= ((int64_t) (void*) wire_stop_global_event_stream);
    dummy_var ^= ((int64_t) (void*) wire_host_stop_system_key_propagate);
    dummy_var ^= ((int64_t) (void*) wire_peer_get_default_sessions_count);
    dummy_var ^= ((int64_t) (void*) wire_session_add_existed_sync);
    dummy_var ^= ((int64_t) (void*) wire_session_add_sync);
    dummy_var ^= ((int64_t) (void*) wire_session_start);
    dummy_var ^= ((int64_t) (void*) wire_session_start_with_displays);
    dummy_var ^= ((int64_t) (void*) wire_session_get_remember);
    dummy_var ^= ((int64_t) (void*) wire_session_get_toggle_option);
    dummy_var ^= ((int64_t) (void*) wire_session_get_toggle_option_sync);
    dummy_var ^= ((int64_t) (void*) wire_session_get_option);
    dummy_var ^= ((int64_t) (void*) wire_session_login);
    dummy_var ^= ((int64_t) (void*) wire_session_send2fa);
    dummy_var ^= ((int64_t) (void*) wire_session_get_enable_trusted_devices);
    dummy_var ^= ((int64_t) (void*) wire_session_close);
    dummy_var ^= ((int64_t) (void*) wire_session_refresh);
    dummy_var ^= ((int64_t) (void*) wire_session_is_multi_ui_session);
    dummy_var ^= ((int64_t) (void*) wire_session_record_screen);
    dummy_var ^= ((int64_t) (void*) wire_session_get_is_recording);
    dummy_var ^= ((int64_t) (void*) wire_session_reconnect);
    dummy_var ^= ((int64_t) (void*) wire_session_toggle_option);
    dummy_var ^= ((int64_t) (void*) wire_session_toggle_privacy_mode);
    dummy_var ^= ((int64_t) (void*) wire_session_get_flutter_option);
    dummy_var ^= ((int64_t) (void*) wire_session_set_flutter_option);
    dummy_var ^= ((int64_t) (void*) wire_get_next_texture_key);
    dummy_var ^= ((int64_t) (void*) wire_get_local_flutter_option);
    dummy_var ^= ((int64_t) (void*) wire_set_local_flutter_option);
    dummy_var ^= ((int64_t) (void*) wire_get_local_kb_layout_type);
    dummy_var ^= ((int64_t) (void*) wire_set_local_kb_layout_type);
    dummy_var ^= ((int64_t) (void*) wire_session_get_view_style);
    dummy_var ^= ((int64_t) (void*) wire_session_set_view_style);
    dummy_var ^= ((int64_t) (void*) wire_session_get_scroll_style);
    dummy_var ^= ((int64_t) (void*) wire_session_set_scroll_style);
    dummy_var ^= ((int64_t) (void*) wire_session_get_image_quality);
    dummy_var ^= ((int64_t) (void*) wire_session_set_image_quality);
    dummy_var ^= ((int64_t) (void*) wire_session_get_keyboard_mode);
    dummy_var ^= ((int64_t) (void*) wire_session_set_keyboard_mode);
    dummy_var ^= ((int64_t) (void*) wire_session_get_reverse_mouse_wheel_sync);
    dummy_var ^= ((int64_t) (void*) wire_session_set_reverse_mouse_wheel);
    dummy_var ^= ((int64_t) (void*) wire_session_get_displays_as_individual_windows);
    dummy_var ^= ((int64_t) (void*) wire_session_set_displays_as_individual_windows);
    dummy_var ^= ((int64_t) (void*) wire_session_get_use_all_my_displays_for_the_remote_session);
    dummy_var ^= ((int64_t) (void*) wire_session_set_use_all_my_displays_for_the_remote_session);
    dummy_var ^= ((int64_t) (void*) wire_session_get_custom_image_quality);
    dummy_var ^= ((int64_t) (void*) wire_session_is_keyboard_mode_supported);
    dummy_var ^= ((int64_t) (void*) wire_session_set_custom_image_quality);
    dummy_var ^= ((int64_t) (void*) wire_session_set_custom_fps);
    dummy_var ^= ((int64_t) (void*) wire_session_lock_screen);
    dummy_var ^= ((int64_t) (void*) wire_session_ctrl_alt_del);
    dummy_var ^= ((int64_t) (void*) wire_session_switch_display);
    dummy_var ^= ((int64_t) (void*) wire_session_handle_flutter_key_event);
    dummy_var ^= ((int64_t) (void*) wire_session_handle_flutter_raw_key_event);
    dummy_var ^= ((int64_t) (void*) wire_session_enter_or_leave);
    dummy_var ^= ((int64_t) (void*) wire_session_input_key);
    dummy_var ^= ((int64_t) (void*) wire_session_input_string);
    dummy_var ^= ((int64_t) (void*) wire_session_send_chat);
    dummy_var ^= ((int64_t) (void*) wire_session_peer_option);
    dummy_var ^= ((int64_t) (void*) wire_session_get_peer_option);
    dummy_var ^= ((int64_t) (void*) wire_session_input_os_password);
    dummy_var ^= ((int64_t) (void*) wire_session_read_remote_dir);
    dummy_var ^= ((int64_t) (void*) wire_session_send_files);
    dummy_var ^= ((int64_t) (void*) wire_session_set_confirm_override_file);
    dummy_var ^= ((int64_t) (void*) wire_session_remove_file);
    dummy_var ^= ((int64_t) (void*) wire_session_read_dir_to_remove_recursive);
    dummy_var ^= ((int64_t) (void*) wire_session_remove_all_empty_dirs);
    dummy_var ^= ((int64_t) (void*) wire_session_cancel_job);
    dummy_var ^= ((int64_t) (void*) wire_session_create_dir);
    dummy_var ^= ((int64_t) (void*) wire_session_read_local_dir_sync);
    dummy_var ^= ((int64_t) (void*) wire_session_read_local_empty_dirs_recursive_sync);
    dummy_var ^= ((int64_t) (void*) wire_session_read_remote_empty_dirs_recursive_sync);
    dummy_var ^= ((int64_t) (void*) wire_session_get_platform);
    dummy_var ^= ((int64_t) (void*) wire_session_load_last_transfer_jobs);
    dummy_var ^= ((int64_t) (void*) wire_session_add_job);
    dummy_var ^= ((int64_t) (void*) wire_session_resume_job);
    dummy_var ^= ((int64_t) (void*) wire_session_rename_file);
    dummy_var ^= ((int64_t) (void*) wire_session_elevate_direct);
    dummy_var ^= ((int64_t) (void*) wire_session_elevate_with_logon);
    dummy_var ^= ((int64_t) (void*) wire_session_switch_sides);
    dummy_var ^= ((int64_t) (void*) wire_session_change_resolution);
    dummy_var ^= ((int64_t) (void*) wire_session_set_size);
    dummy_var ^= ((int64_t) (void*) wire_session_send_selected_session_id);
    dummy_var ^= ((int64_t) (void*) wire_main_get_sound_inputs);
    dummy_var ^= ((int64_t) (void*) wire_main_get_login_device_info);
    dummy_var ^= ((int64_t) (void*) wire_main_change_id);
    dummy_var ^= ((int64_t) (void*) wire_main_get_async_status);
    dummy_var ^= ((int64_t) (void*) wire_main_get_http_status);
    dummy_var ^= ((int64_t) (void*) wire_main_get_option);
    dummy_var ^= ((int64_t) (void*) wire_main_get_option_sync);
    dummy_var ^= ((int64_t) (void*) wire_main_get_error);
    dummy_var ^= ((int64_t) (void*) wire_main_show_option);
    dummy_var ^= ((int64_t) (void*) wire_main_set_option);
    dummy_var ^= ((int64_t) (void*) wire_main_get_options);
    dummy_var ^= ((int64_t) (void*) wire_main_get_options_sync);
    dummy_var ^= ((int64_t) (void*) wire_main_set_options);
    dummy_var ^= ((int64_t) (void*) wire_main_test_if_valid_server);
    dummy_var ^= ((int64_t) (void*) wire_main_set_socks);
    dummy_var ^= ((int64_t) (void*) wire_main_get_proxy_status);
    dummy_var ^= ((int64_t) (void*) wire_main_get_socks);
    dummy_var ^= ((int64_t) (void*) wire_main_get_app_name);
    dummy_var ^= ((int64_t) (void*) wire_main_get_app_name_sync);
    dummy_var ^= ((int64_t) (void*) wire_main_uri_prefix_sync);
    dummy_var ^= ((int64_t) (void*) wire_main_get_license);
    dummy_var ^= ((int64_t) (void*) wire_main_get_version);
    dummy_var ^= ((int64_t) (void*) wire_main_get_fav);
    dummy_var ^= ((int64_t) (void*) wire_main_store_fav);
    dummy_var ^= ((int64_t) (void*) wire_main_get_peer_sync);
    dummy_var ^= ((int64_t) (void*) wire_main_get_lan_peers);
    dummy_var ^= ((int64_t) (void*) wire_main_get_connect_status);
    dummy_var ^= ((int64_t) (void*) wire_main_check_connect_status);
    dummy_var ^= ((int64_t) (void*) wire_main_is_using_public_server);
    dummy_var ^= ((int64_t) (void*) wire_main_discover);
    dummy_var ^= ((int64_t) (void*) wire_main_get_api_server);
    dummy_var ^= ((int64_t) (void*) wire_main_http_request);
    dummy_var ^= ((int64_t) (void*) wire_main_get_local_option);
    dummy_var ^= ((int64_t) (void*) wire_main_get_use_texture_render);
    dummy_var ^= ((int64_t) (void*) wire_main_get_env);
    dummy_var ^= ((int64_t) (void*) wire_main_set_local_option);
    dummy_var ^= ((int64_t) (void*) wire_main_handle_wayland_screencast_restore_token);
    dummy_var ^= ((int64_t) (void*) wire_main_get_input_source);
    dummy_var ^= ((int64_t) (void*) wire_main_set_input_source);
    dummy_var ^= ((int64_t) (void*) wire_main_get_my_id);
    dummy_var ^= ((int64_t) (void*) wire_main_get_uuid);
    dummy_var ^= ((int64_t) (void*) wire_main_get_peer_option);
    dummy_var ^= ((int64_t) (void*) wire_main_get_peer_option_sync);
    dummy_var ^= ((int64_t) (void*) wire_main_get_peer_flutter_option_sync);
    dummy_var ^= ((int64_t) (void*) wire_main_set_peer_flutter_option_sync);
    dummy_var ^= ((int64_t) (void*) wire_main_set_peer_option);
    dummy_var ^= ((int64_t) (void*) wire_main_set_peer_option_sync);
    dummy_var ^= ((int64_t) (void*) wire_main_set_peer_alias);
    dummy_var ^= ((int64_t) (void*) wire_main_get_new_stored_peers);
    dummy_var ^= ((int64_t) (void*) wire_main_forget_password);
    dummy_var ^= ((int64_t) (void*) wire_main_peer_has_password);
    dummy_var ^= ((int64_t) (void*) wire_main_peer_exists);
    dummy_var ^= ((int64_t) (void*) wire_main_load_recent_peers);
    dummy_var ^= ((int64_t) (void*) wire_main_load_recent_peers_sync);
    dummy_var ^= ((int64_t) (void*) wire_main_load_lan_peers_sync);
    dummy_var ^= ((int64_t) (void*) wire_main_load_recent_peers_for_ab);
    dummy_var ^= ((int64_t) (void*) wire_main_load_fav_peers);
    dummy_var ^= ((int64_t) (void*) wire_main_load_lan_peers);
    dummy_var ^= ((int64_t) (void*) wire_main_remove_discovered);
    dummy_var ^= ((int64_t) (void*) wire_main_change_theme);
    dummy_var ^= ((int64_t) (void*) wire_main_change_language);
    dummy_var ^= ((int64_t) (void*) wire_main_video_save_directory);
    dummy_var ^= ((int64_t) (void*) wire_main_set_user_default_option);
    dummy_var ^= ((int64_t) (void*) wire_main_get_user_default_option);
    dummy_var ^= ((int64_t) (void*) wire_main_handle_relay_id);
    dummy_var ^= ((int64_t) (void*) wire_main_is_option_fixed);
    dummy_var ^= ((int64_t) (void*) wire_main_get_main_display);
    dummy_var ^= ((int64_t) (void*) wire_main_get_displays);
    dummy_var ^= ((int64_t) (void*) wire_session_add_port_forward);
    dummy_var ^= ((int64_t) (void*) wire_session_remove_port_forward);
    dummy_var ^= ((int64_t) (void*) wire_session_new_rdp);
    dummy_var ^= ((int64_t) (void*) wire_session_request_voice_call);
    dummy_var ^= ((int64_t) (void*) wire_session_close_voice_call);
    dummy_var ^= ((int64_t) (void*) wire_session_get_conn_token);
    dummy_var ^= ((int64_t) (void*) wire_cm_handle_incoming_voice_call);
    dummy_var ^= ((int64_t) (void*) wire_cm_close_voice_call);
    dummy_var ^= ((int64_t) (void*) wire_set_voice_call_input_device);
    dummy_var ^= ((int64_t) (void*) wire_get_voice_call_input_device);
    dummy_var ^= ((int64_t) (void*) wire_main_get_last_remote_id);
    dummy_var ^= ((int64_t) (void*) wire_main_get_software_update_url);
    dummy_var ^= ((int64_t) (void*) wire_main_get_home_dir);
    dummy_var ^= ((int64_t) (void*) wire_main_get_langs);
    dummy_var ^= ((int64_t) (void*) wire_main_get_temporary_password);
    dummy_var ^= ((int64_t) (void*) wire_main_get_permanent_password);
    dummy_var ^= ((int64_t) (void*) wire_main_get_fingerprint);
    dummy_var ^= ((int64_t) (void*) wire_cm_get_clients_state);
    dummy_var ^= ((int64_t) (void*) wire_cm_check_clients_length);
    dummy_var ^= ((int64_t) (void*) wire_cm_get_clients_length);
    dummy_var ^= ((int64_t) (void*) wire_main_init);
    dummy_var ^= ((int64_t) (void*) wire_main_device_id);
    dummy_var ^= ((int64_t) (void*) wire_main_device_name);
    dummy_var ^= ((int64_t) (void*) wire_main_remove_peer);
    dummy_var ^= ((int64_t) (void*) wire_main_has_hwcodec);
    dummy_var ^= ((int64_t) (void*) wire_main_has_vram);
    dummy_var ^= ((int64_t) (void*) wire_main_supported_hwdecodings);
    dummy_var ^= ((int64_t) (void*) wire_main_is_root);
    dummy_var ^= ((int64_t) (void*) wire_get_double_click_time);
    dummy_var ^= ((int64_t) (void*) wire_main_start_dbus_server);
    dummy_var ^= ((int64_t) (void*) wire_main_save_ab);
    dummy_var ^= ((int64_t) (void*) wire_main_clear_ab);
    dummy_var ^= ((int64_t) (void*) wire_main_load_ab);
    dummy_var ^= ((int64_t) (void*) wire_main_save_group);
    dummy_var ^= ((int64_t) (void*) wire_main_clear_group);
    dummy_var ^= ((int64_t) (void*) wire_main_load_group);
    dummy_var ^= ((int64_t) (void*) wire_session_send_pointer);
    dummy_var ^= ((int64_t) (void*) wire_session_send_mouse);
    dummy_var ^= ((int64_t) (void*) wire_session_restart_remote_device);
    dummy_var ^= ((int64_t) (void*) wire_session_get_audit_server_sync);
    dummy_var ^= ((int64_t) (void*) wire_session_send_note);
    dummy_var ^= ((int64_t) (void*) wire_session_alternative_codecs);
    dummy_var ^= ((int64_t) (void*) wire_session_change_prefer_codec);
    dummy_var ^= ((int64_t) (void*) wire_session_on_waiting_for_image_dialog_show);
    dummy_var ^= ((int64_t) (void*) wire_session_toggle_virtual_display);
    dummy_var ^= ((int64_t) (void*) wire_main_set_home_dir);
    dummy_var ^= ((int64_t) (void*) wire_main_get_data_dir_ios);
    dummy_var ^= ((int64_t) (void*) wire_main_stop_service);
    dummy_var ^= ((int64_t) (void*) wire_main_start_service);
    dummy_var ^= ((int64_t) (void*) wire_main_update_temporary_password);
    dummy_var ^= ((int64_t) (void*) wire_main_set_permanent_password);
    dummy_var ^= ((int64_t) (void*) wire_main_check_super_user_permission);
    dummy_var ^= ((int64_t) (void*) wire_main_get_unlock_pin);
    dummy_var ^= ((int64_t) (void*) wire_main_set_unlock_pin);
    dummy_var ^= ((int64_t) (void*) wire_main_check_mouse_time);
    dummy_var ^= ((int64_t) (void*) wire_main_get_mouse_time);
    dummy_var ^= ((int64_t) (void*) wire_main_wol);
    dummy_var ^= ((int64_t) (void*) wire_main_create_shortcut);
    dummy_var ^= ((int64_t) (void*) wire_cm_send_chat);
    dummy_var ^= ((int64_t) (void*) wire_cm_login_res);
    dummy_var ^= ((int64_t) (void*) wire_cm_close_connection);
    dummy_var ^= ((int64_t) (void*) wire_cm_remove_disconnected_connection);
    dummy_var ^= ((int64_t) (void*) wire_cm_check_click_time);
    dummy_var ^= ((int64_t) (void*) wire_cm_get_click_time);
    dummy_var ^= ((int64_t) (void*) wire_cm_switch_permission);
    dummy_var ^= ((int64_t) (void*) wire_cm_can_elevate);
    dummy_var ^= ((int64_t) (void*) wire_cm_elevate_portable);
    dummy_var ^= ((int64_t) (void*) wire_cm_switch_back);
    dummy_var ^= ((int64_t) (void*) wire_cm_get_config);
    dummy_var ^= ((int64_t) (void*) wire_main_get_build_date);
    dummy_var ^= ((int64_t) (void*) wire_translate);
    dummy_var ^= ((int64_t) (void*) wire_session_get_rgba_size);
    dummy_var ^= ((int64_t) (void*) wire_session_next_rgba);
    dummy_var ^= ((int64_t) (void*) wire_session_register_pixelbuffer_texture);
    dummy_var ^= ((int64_t) (void*) wire_session_register_gpu_texture);
    dummy_var ^= ((int64_t) (void*) wire_query_onlines);
    dummy_var ^= ((int64_t) (void*) wire_version_to_number);
    dummy_var ^= ((int64_t) (void*) wire_option_synced);
    dummy_var ^= ((int64_t) (void*) wire_main_is_installed);
    dummy_var ^= ((int64_t) (void*) wire_main_init_input_source);
    dummy_var ^= ((int64_t) (void*) wire_main_is_installed_lower_version);
    dummy_var ^= ((int64_t) (void*) wire_main_is_installed_daemon);
    dummy_var ^= ((int64_t) (void*) wire_main_is_process_trusted);
    dummy_var ^= ((int64_t) (void*) wire_main_is_can_screen_recording);
    dummy_var ^= ((int64_t) (void*) wire_main_is_can_input_monitoring);
    dummy_var ^= ((int64_t) (void*) wire_main_is_share_rdp);
    dummy_var ^= ((int64_t) (void*) wire_main_set_share_rdp);
    dummy_var ^= ((int64_t) (void*) wire_main_goto_install);
    dummy_var ^= ((int64_t) (void*) wire_main_get_new_version);
    dummy_var ^= ((int64_t) (void*) wire_main_update_me);
    dummy_var ^= ((int64_t) (void*) wire_set_cur_session_id);
    dummy_var ^= ((int64_t) (void*) wire_install_show_run_without_install);
    dummy_var ^= ((int64_t) (void*) wire_install_run_without_install);
    dummy_var ^= ((int64_t) (void*) wire_install_install_me);
    dummy_var ^= ((int64_t) (void*) wire_install_install_path);
    dummy_var ^= ((int64_t) (void*) wire_install_install_options);
    dummy_var ^= ((int64_t) (void*) wire_main_account_auth);
    dummy_var ^= ((int64_t) (void*) wire_main_account_auth_cancel);
    dummy_var ^= ((int64_t) (void*) wire_main_account_auth_result);
    dummy_var ^= ((int64_t) (void*) wire_main_on_main_window_close);
    dummy_var ^= ((int64_t) (void*) wire_main_current_is_wayland);
    dummy_var ^= ((int64_t) (void*) wire_main_is_login_wayland);
    dummy_var ^= ((int64_t) (void*) wire_main_hide_dock);
    dummy_var ^= ((int64_t) (void*) wire_main_has_file_clipboard);
    dummy_var ^= ((int64_t) (void*) wire_main_has_gpu_texture_render);
    dummy_var ^= ((int64_t) (void*) wire_cm_init);
    dummy_var ^= ((int64_t) (void*) wire_main_start_ipc_url_server);
    dummy_var ^= ((int64_t) (void*) wire_main_test_wallpaper);
    dummy_var ^= ((int64_t) (void*) wire_main_support_remove_wallpaper);
    dummy_var ^= ((int64_t) (void*) wire_is_incoming_only);
    dummy_var ^= ((int64_t) (void*) wire_is_outgoing_only);
    dummy_var ^= ((int64_t) (void*) wire_is_custom_client);
    dummy_var ^= ((int64_t) (void*) wire_is_disable_settings);
    dummy_var ^= ((int64_t) (void*) wire_is_disable_ab);
    dummy_var ^= ((int64_t) (void*) wire_is_disable_account);
    dummy_var ^= ((int64_t) (void*) wire_is_disable_group_panel);
    dummy_var ^= ((int64_t) (void*) wire_is_disable_installation);
    dummy_var ^= ((int64_t) (void*) wire_is_preset_password);
    dummy_var ^= ((int64_t) (void*) wire_is_preset_password_mobile_only);
    dummy_var ^= ((int64_t) (void*) wire_send_url_scheme);
    dummy_var ^= ((int64_t) (void*) wire_plugin_event);
    dummy_var ^= ((int64_t) (void*) wire_plugin_register_event_stream);
    dummy_var ^= ((int64_t) (void*) wire_plugin_get_session_option);
    dummy_var ^= ((int64_t) (void*) wire_plugin_set_session_option);
    dummy_var ^= ((int64_t) (void*) wire_plugin_get_shared_option);
    dummy_var ^= ((int64_t) (void*) wire_plugin_set_shared_option);
    dummy_var ^= ((int64_t) (void*) wire_plugin_reload);
    dummy_var ^= ((int64_t) (void*) wire_plugin_enable);
    dummy_var ^= ((int64_t) (void*) wire_plugin_is_enabled);
    dummy_var ^= ((int64_t) (void*) wire_plugin_feature_is_enabled);
    dummy_var ^= ((int64_t) (void*) wire_plugin_sync_ui);
    dummy_var ^= ((int64_t) (void*) wire_plugin_list_reload);
    dummy_var ^= ((int64_t) (void*) wire_plugin_install);
    dummy_var ^= ((int64_t) (void*) wire_is_support_multi_ui_session);
    dummy_var ^= ((int64_t) (void*) wire_is_selinux_enforcing);
    dummy_var ^= ((int64_t) (void*) wire_main_default_privacy_mode_impl);
    dummy_var ^= ((int64_t) (void*) wire_main_supported_privacy_mode_impls);
    dummy_var ^= ((int64_t) (void*) wire_main_supported_input_source);
    dummy_var ^= ((int64_t) (void*) wire_main_generate2fa);
    dummy_var ^= ((int64_t) (void*) wire_main_verify2fa);
    dummy_var ^= ((int64_t) (void*) wire_main_has_valid_2fa_sync);
    dummy_var ^= ((int64_t) (void*) wire_main_verify_bot);
    dummy_var ^= ((int64_t) (void*) wire_main_has_valid_bot_sync);
    dummy_var ^= ((int64_t) (void*) wire_main_get_hard_option);
    dummy_var ^= ((int64_t) (void*) wire_main_get_buildin_option);
    dummy_var ^= ((int64_t) (void*) wire_main_check_hwcodec);
    dummy_var ^= ((int64_t) (void*) wire_main_get_trusted_devices);
    dummy_var ^= ((int64_t) (void*) wire_main_remove_trusted_devices);
    dummy_var ^= ((int64_t) (void*) wire_main_clear_trusted_devices);
    dummy_var ^= ((int64_t) (void*) wire_main_max_encrypt_len);
    dummy_var ^= ((int64_t) (void*) wire_session_request_new_display_init_msgs);
    dummy_var ^= ((int64_t) (void*) wire_main_audio_support_loopback);
    dummy_var ^= ((int64_t) (void*) new_StringList_0);
    dummy_var ^= ((int64_t) (void*) new_int_32_list_0);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_0);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturn);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    return dummy_var;
}
