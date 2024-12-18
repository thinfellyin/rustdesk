use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_start_global_event_stream(port_: i64, app_type: *mut wire_uint_8_list) {
    wire_start_global_event_stream_impl(port_, app_type)
}

#[no_mangle]
pub extern "C" fn wire_stop_global_event_stream(port_: i64, app_type: *mut wire_uint_8_list) {
    wire_stop_global_event_stream_impl(port_, app_type)
}

#[no_mangle]
pub extern "C" fn wire_host_stop_system_key_propagate(port_: i64, _stopped: bool) {
    wire_host_stop_system_key_propagate_impl(port_, _stopped)
}

#[no_mangle]
pub extern "C" fn wire_peer_get_default_sessions_count(
    id: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_peer_get_default_sessions_count_impl(id)
}

#[no_mangle]
pub extern "C" fn wire_session_add_existed_sync(
    id: *mut wire_uint_8_list,
    session_id: *mut wire_uint_8_list,
    displays: *mut wire_int_32_list,
) -> support::WireSyncReturn {
    wire_session_add_existed_sync_impl(id, session_id, displays)
}

#[no_mangle]
pub extern "C" fn wire_session_add_sync(
    session_id: *mut wire_uint_8_list,
    id: *mut wire_uint_8_list,
    is_file_transfer: bool,
    is_port_forward: bool,
    is_rdp: bool,
    switch_uuid: *mut wire_uint_8_list,
    force_relay: bool,
    password: *mut wire_uint_8_list,
    is_shared_password: bool,
    conn_token: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_session_add_sync_impl(
        session_id,
        id,
        is_file_transfer,
        is_port_forward,
        is_rdp,
        switch_uuid,
        force_relay,
        password,
        is_shared_password,
        conn_token,
    )
}

#[no_mangle]
pub extern "C" fn wire_session_start(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    id: *mut wire_uint_8_list,
) {
    wire_session_start_impl(port_, session_id, id)
}

#[no_mangle]
pub extern "C" fn wire_session_start_with_displays(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    id: *mut wire_uint_8_list,
    displays: *mut wire_int_32_list,
) {
    wire_session_start_with_displays_impl(port_, session_id, id, displays)
}

#[no_mangle]
pub extern "C" fn wire_session_get_remember(port_: i64, session_id: *mut wire_uint_8_list) {
    wire_session_get_remember_impl(port_, session_id)
}

#[no_mangle]
pub extern "C" fn wire_session_get_toggle_option(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    arg: *mut wire_uint_8_list,
) {
    wire_session_get_toggle_option_impl(port_, session_id, arg)
}

#[no_mangle]
pub extern "C" fn wire_session_get_toggle_option_sync(
    session_id: *mut wire_uint_8_list,
    arg: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_session_get_toggle_option_sync_impl(session_id, arg)
}

#[no_mangle]
pub extern "C" fn wire_session_get_option(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    arg: *mut wire_uint_8_list,
) {
    wire_session_get_option_impl(port_, session_id, arg)
}

#[no_mangle]
pub extern "C" fn wire_session_login(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    os_username: *mut wire_uint_8_list,
    os_password: *mut wire_uint_8_list,
    password: *mut wire_uint_8_list,
    remember: bool,
) {
    wire_session_login_impl(
        port_,
        session_id,
        os_username,
        os_password,
        password,
        remember,
    )
}

#[no_mangle]
pub extern "C" fn wire_session_send2fa(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    code: *mut wire_uint_8_list,
    trust_this_device: bool,
) {
    wire_session_send2fa_impl(port_, session_id, code, trust_this_device)
}

#[no_mangle]
pub extern "C" fn wire_session_get_enable_trusted_devices(
    session_id: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_session_get_enable_trusted_devices_impl(session_id)
}

#[no_mangle]
pub extern "C" fn wire_session_close(port_: i64, session_id: *mut wire_uint_8_list) {
    wire_session_close_impl(port_, session_id)
}

#[no_mangle]
pub extern "C" fn wire_session_refresh(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    display: usize,
) {
    wire_session_refresh_impl(port_, session_id, display)
}

#[no_mangle]
pub extern "C" fn wire_session_is_multi_ui_session(
    session_id: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_session_is_multi_ui_session_impl(session_id)
}

#[no_mangle]
pub extern "C" fn wire_session_record_screen(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    start: bool,
) {
    wire_session_record_screen_impl(port_, session_id, start)
}

#[no_mangle]
pub extern "C" fn wire_session_get_is_recording(
    session_id: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_session_get_is_recording_impl(session_id)
}

#[no_mangle]
pub extern "C" fn wire_session_reconnect(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    force_relay: bool,
) {
    wire_session_reconnect_impl(port_, session_id, force_relay)
}

#[no_mangle]
pub extern "C" fn wire_session_toggle_option(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) {
    wire_session_toggle_option_impl(port_, session_id, value)
}

#[no_mangle]
pub extern "C" fn wire_session_toggle_privacy_mode(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    impl_key: *mut wire_uint_8_list,
    on: bool,
) {
    wire_session_toggle_privacy_mode_impl(port_, session_id, impl_key, on)
}

#[no_mangle]
pub extern "C" fn wire_session_get_flutter_option(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    k: *mut wire_uint_8_list,
) {
    wire_session_get_flutter_option_impl(port_, session_id, k)
}

#[no_mangle]
pub extern "C" fn wire_session_set_flutter_option(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    k: *mut wire_uint_8_list,
    v: *mut wire_uint_8_list,
) {
    wire_session_set_flutter_option_impl(port_, session_id, k, v)
}

#[no_mangle]
pub extern "C" fn wire_get_next_texture_key() -> support::WireSyncReturn {
    wire_get_next_texture_key_impl()
}

#[no_mangle]
pub extern "C" fn wire_get_local_flutter_option(
    k: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_get_local_flutter_option_impl(k)
}

#[no_mangle]
pub extern "C" fn wire_set_local_flutter_option(
    port_: i64,
    k: *mut wire_uint_8_list,
    v: *mut wire_uint_8_list,
) {
    wire_set_local_flutter_option_impl(port_, k, v)
}

#[no_mangle]
pub extern "C" fn wire_get_local_kb_layout_type() -> support::WireSyncReturn {
    wire_get_local_kb_layout_type_impl()
}

#[no_mangle]
pub extern "C" fn wire_set_local_kb_layout_type(port_: i64, kb_layout_type: *mut wire_uint_8_list) {
    wire_set_local_kb_layout_type_impl(port_, kb_layout_type)
}

#[no_mangle]
pub extern "C" fn wire_session_get_view_style(port_: i64, session_id: *mut wire_uint_8_list) {
    wire_session_get_view_style_impl(port_, session_id)
}

#[no_mangle]
pub extern "C" fn wire_session_set_view_style(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) {
    wire_session_set_view_style_impl(port_, session_id, value)
}

#[no_mangle]
pub extern "C" fn wire_session_get_scroll_style(port_: i64, session_id: *mut wire_uint_8_list) {
    wire_session_get_scroll_style_impl(port_, session_id)
}

#[no_mangle]
pub extern "C" fn wire_session_set_scroll_style(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) {
    wire_session_set_scroll_style_impl(port_, session_id, value)
}

#[no_mangle]
pub extern "C" fn wire_session_get_image_quality(port_: i64, session_id: *mut wire_uint_8_list) {
    wire_session_get_image_quality_impl(port_, session_id)
}

#[no_mangle]
pub extern "C" fn wire_session_set_image_quality(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) {
    wire_session_set_image_quality_impl(port_, session_id, value)
}

#[no_mangle]
pub extern "C" fn wire_session_get_keyboard_mode(port_: i64, session_id: *mut wire_uint_8_list) {
    wire_session_get_keyboard_mode_impl(port_, session_id)
}

#[no_mangle]
pub extern "C" fn wire_session_set_keyboard_mode(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) {
    wire_session_set_keyboard_mode_impl(port_, session_id, value)
}

#[no_mangle]
pub extern "C" fn wire_session_get_reverse_mouse_wheel_sync(
    session_id: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_session_get_reverse_mouse_wheel_sync_impl(session_id)
}

#[no_mangle]
pub extern "C" fn wire_session_set_reverse_mouse_wheel(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) {
    wire_session_set_reverse_mouse_wheel_impl(port_, session_id, value)
}

#[no_mangle]
pub extern "C" fn wire_session_get_displays_as_individual_windows(
    session_id: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_session_get_displays_as_individual_windows_impl(session_id)
}

#[no_mangle]
pub extern "C" fn wire_session_set_displays_as_individual_windows(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) {
    wire_session_set_displays_as_individual_windows_impl(port_, session_id, value)
}

#[no_mangle]
pub extern "C" fn wire_session_get_use_all_my_displays_for_the_remote_session(
    session_id: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_session_get_use_all_my_displays_for_the_remote_session_impl(session_id)
}

#[no_mangle]
pub extern "C" fn wire_session_set_use_all_my_displays_for_the_remote_session(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) {
    wire_session_set_use_all_my_displays_for_the_remote_session_impl(port_, session_id, value)
}

#[no_mangle]
pub extern "C" fn wire_session_get_custom_image_quality(
    port_: i64,
    session_id: *mut wire_uint_8_list,
) {
    wire_session_get_custom_image_quality_impl(port_, session_id)
}

#[no_mangle]
pub extern "C" fn wire_session_is_keyboard_mode_supported(
    session_id: *mut wire_uint_8_list,
    mode: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_session_is_keyboard_mode_supported_impl(session_id, mode)
}

#[no_mangle]
pub extern "C" fn wire_session_set_custom_image_quality(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    value: i32,
) {
    wire_session_set_custom_image_quality_impl(port_, session_id, value)
}

#[no_mangle]
pub extern "C" fn wire_session_set_custom_fps(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    fps: i32,
) {
    wire_session_set_custom_fps_impl(port_, session_id, fps)
}

#[no_mangle]
pub extern "C" fn wire_session_lock_screen(port_: i64, session_id: *mut wire_uint_8_list) {
    wire_session_lock_screen_impl(port_, session_id)
}

#[no_mangle]
pub extern "C" fn wire_session_ctrl_alt_del(port_: i64, session_id: *mut wire_uint_8_list) {
    wire_session_ctrl_alt_del_impl(port_, session_id)
}

#[no_mangle]
pub extern "C" fn wire_session_switch_display(
    port_: i64,
    is_desktop: bool,
    session_id: *mut wire_uint_8_list,
    value: *mut wire_int_32_list,
) {
    wire_session_switch_display_impl(port_, is_desktop, session_id, value)
}

#[no_mangle]
pub extern "C" fn wire_session_handle_flutter_key_event(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    character: *mut wire_uint_8_list,
    usb_hid: i32,
    lock_modes: i32,
    down_or_up: bool,
) {
    wire_session_handle_flutter_key_event_impl(
        port_, session_id, character, usb_hid, lock_modes, down_or_up,
    )
}

#[no_mangle]
pub extern "C" fn wire_session_handle_flutter_raw_key_event(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    name: *mut wire_uint_8_list,
    platform_code: i32,
    position_code: i32,
    lock_modes: i32,
    down_or_up: bool,
) {
    wire_session_handle_flutter_raw_key_event_impl(
        port_,
        session_id,
        name,
        platform_code,
        position_code,
        lock_modes,
        down_or_up,
    )
}

#[no_mangle]
pub extern "C" fn wire_session_enter_or_leave(
    _session_id: *mut wire_uint_8_list,
    _enter: bool,
) -> support::WireSyncReturn {
    wire_session_enter_or_leave_impl(_session_id, _enter)
}

#[no_mangle]
pub extern "C" fn wire_session_input_key(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    name: *mut wire_uint_8_list,
    down: bool,
    press: bool,
    alt: bool,
    ctrl: bool,
    shift: bool,
    command: bool,
) {
    wire_session_input_key_impl(
        port_, session_id, name, down, press, alt, ctrl, shift, command,
    )
}

#[no_mangle]
pub extern "C" fn wire_session_input_string(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) {
    wire_session_input_string_impl(port_, session_id, value)
}

#[no_mangle]
pub extern "C" fn wire_session_send_chat(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    text: *mut wire_uint_8_list,
) {
    wire_session_send_chat_impl(port_, session_id, text)
}

#[no_mangle]
pub extern "C" fn wire_session_peer_option(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    name: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) {
    wire_session_peer_option_impl(port_, session_id, name, value)
}

#[no_mangle]
pub extern "C" fn wire_session_get_peer_option(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    name: *mut wire_uint_8_list,
) {
    wire_session_get_peer_option_impl(port_, session_id, name)
}

#[no_mangle]
pub extern "C" fn wire_session_input_os_password(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) {
    wire_session_input_os_password_impl(port_, session_id, value)
}

#[no_mangle]
pub extern "C" fn wire_session_read_remote_dir(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    path: *mut wire_uint_8_list,
    include_hidden: bool,
) {
    wire_session_read_remote_dir_impl(port_, session_id, path, include_hidden)
}

#[no_mangle]
pub extern "C" fn wire_session_send_files(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    act_id: i32,
    path: *mut wire_uint_8_list,
    to: *mut wire_uint_8_list,
    file_num: i32,
    include_hidden: bool,
    is_remote: bool,
    _is_dir: bool,
) {
    wire_session_send_files_impl(
        port_,
        session_id,
        act_id,
        path,
        to,
        file_num,
        include_hidden,
        is_remote,
        _is_dir,
    )
}

#[no_mangle]
pub extern "C" fn wire_session_set_confirm_override_file(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    act_id: i32,
    file_num: i32,
    need_override: bool,
    remember: bool,
    is_upload: bool,
) {
    wire_session_set_confirm_override_file_impl(
        port_,
        session_id,
        act_id,
        file_num,
        need_override,
        remember,
        is_upload,
    )
}

#[no_mangle]
pub extern "C" fn wire_session_remove_file(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    act_id: i32,
    path: *mut wire_uint_8_list,
    file_num: i32,
    is_remote: bool,
) {
    wire_session_remove_file_impl(port_, session_id, act_id, path, file_num, is_remote)
}

#[no_mangle]
pub extern "C" fn wire_session_read_dir_to_remove_recursive(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    act_id: i32,
    path: *mut wire_uint_8_list,
    is_remote: bool,
    show_hidden: bool,
) {
    wire_session_read_dir_to_remove_recursive_impl(
        port_,
        session_id,
        act_id,
        path,
        is_remote,
        show_hidden,
    )
}

#[no_mangle]
pub extern "C" fn wire_session_remove_all_empty_dirs(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    act_id: i32,
    path: *mut wire_uint_8_list,
    is_remote: bool,
) {
    wire_session_remove_all_empty_dirs_impl(port_, session_id, act_id, path, is_remote)
}

#[no_mangle]
pub extern "C" fn wire_session_cancel_job(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    act_id: i32,
) {
    wire_session_cancel_job_impl(port_, session_id, act_id)
}

#[no_mangle]
pub extern "C" fn wire_session_create_dir(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    act_id: i32,
    path: *mut wire_uint_8_list,
    is_remote: bool,
) {
    wire_session_create_dir_impl(port_, session_id, act_id, path, is_remote)
}

#[no_mangle]
pub extern "C" fn wire_session_read_local_dir_sync(
    port_: i64,
    _session_id: *mut wire_uint_8_list,
    path: *mut wire_uint_8_list,
    show_hidden: bool,
) {
    wire_session_read_local_dir_sync_impl(port_, _session_id, path, show_hidden)
}

#[no_mangle]
pub extern "C" fn wire_session_read_local_empty_dirs_recursive_sync(
    port_: i64,
    _session_id: *mut wire_uint_8_list,
    path: *mut wire_uint_8_list,
    include_hidden: bool,
) {
    wire_session_read_local_empty_dirs_recursive_sync_impl(port_, _session_id, path, include_hidden)
}

#[no_mangle]
pub extern "C" fn wire_session_read_remote_empty_dirs_recursive_sync(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    path: *mut wire_uint_8_list,
    include_hidden: bool,
) {
    wire_session_read_remote_empty_dirs_recursive_sync_impl(port_, session_id, path, include_hidden)
}

#[no_mangle]
pub extern "C" fn wire_session_get_platform(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    is_remote: bool,
) {
    wire_session_get_platform_impl(port_, session_id, is_remote)
}

#[no_mangle]
pub extern "C" fn wire_session_load_last_transfer_jobs(
    port_: i64,
    session_id: *mut wire_uint_8_list,
) {
    wire_session_load_last_transfer_jobs_impl(port_, session_id)
}

#[no_mangle]
pub extern "C" fn wire_session_add_job(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    act_id: i32,
    path: *mut wire_uint_8_list,
    to: *mut wire_uint_8_list,
    file_num: i32,
    include_hidden: bool,
    is_remote: bool,
) {
    wire_session_add_job_impl(
        port_,
        session_id,
        act_id,
        path,
        to,
        file_num,
        include_hidden,
        is_remote,
    )
}

#[no_mangle]
pub extern "C" fn wire_session_resume_job(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    act_id: i32,
    is_remote: bool,
) {
    wire_session_resume_job_impl(port_, session_id, act_id, is_remote)
}

#[no_mangle]
pub extern "C" fn wire_session_rename_file(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    act_id: i32,
    path: *mut wire_uint_8_list,
    new_name: *mut wire_uint_8_list,
    is_remote: bool,
) {
    wire_session_rename_file_impl(port_, session_id, act_id, path, new_name, is_remote)
}

#[no_mangle]
pub extern "C" fn wire_session_elevate_direct(port_: i64, session_id: *mut wire_uint_8_list) {
    wire_session_elevate_direct_impl(port_, session_id)
}

#[no_mangle]
pub extern "C" fn wire_session_elevate_with_logon(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    username: *mut wire_uint_8_list,
    password: *mut wire_uint_8_list,
) {
    wire_session_elevate_with_logon_impl(port_, session_id, username, password)
}

#[no_mangle]
pub extern "C" fn wire_session_switch_sides(port_: i64, session_id: *mut wire_uint_8_list) {
    wire_session_switch_sides_impl(port_, session_id)
}

#[no_mangle]
pub extern "C" fn wire_session_change_resolution(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    display: i32,
    width: i32,
    height: i32,
) {
    wire_session_change_resolution_impl(port_, session_id, display, width, height)
}

#[no_mangle]
pub extern "C" fn wire_session_set_size(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    display: usize,
    width: usize,
    height: usize,
) {
    wire_session_set_size_impl(port_, session_id, display, width, height)
}

#[no_mangle]
pub extern "C" fn wire_session_send_selected_session_id(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    sid: *mut wire_uint_8_list,
) {
    wire_session_send_selected_session_id_impl(port_, session_id, sid)
}

#[no_mangle]
pub extern "C" fn wire_main_get_sound_inputs(port_: i64) {
    wire_main_get_sound_inputs_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_login_device_info() -> support::WireSyncReturn {
    wire_main_get_login_device_info_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_change_id(port_: i64, new_id: *mut wire_uint_8_list) {
    wire_main_change_id_impl(port_, new_id)
}

#[no_mangle]
pub extern "C" fn wire_main_get_async_status(port_: i64) {
    wire_main_get_async_status_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_http_status(port_: i64, url: *mut wire_uint_8_list) {
    wire_main_get_http_status_impl(port_, url)
}

#[no_mangle]
pub extern "C" fn wire_main_get_option(port_: i64, key: *mut wire_uint_8_list) {
    wire_main_get_option_impl(port_, key)
}

#[no_mangle]
pub extern "C" fn wire_main_get_option_sync(key: *mut wire_uint_8_list) -> support::WireSyncReturn {
    wire_main_get_option_sync_impl(key)
}

#[no_mangle]
pub extern "C" fn wire_main_get_error(port_: i64) {
    wire_main_get_error_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_show_option(_key: *mut wire_uint_8_list) -> support::WireSyncReturn {
    wire_main_show_option_impl(_key)
}

#[no_mangle]
pub extern "C" fn wire_main_set_option(
    port_: i64,
    key: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) {
    wire_main_set_option_impl(port_, key, value)
}

#[no_mangle]
pub extern "C" fn wire_main_get_options(port_: i64) {
    wire_main_get_options_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_options_sync() -> support::WireSyncReturn {
    wire_main_get_options_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_set_options(port_: i64, json: *mut wire_uint_8_list) {
    wire_main_set_options_impl(port_, json)
}

#[no_mangle]
pub extern "C" fn wire_main_test_if_valid_server(
    port_: i64,
    server: *mut wire_uint_8_list,
    test_with_proxy: bool,
) {
    wire_main_test_if_valid_server_impl(port_, server, test_with_proxy)
}

#[no_mangle]
pub extern "C" fn wire_main_set_socks(
    port_: i64,
    proxy: *mut wire_uint_8_list,
    username: *mut wire_uint_8_list,
    password: *mut wire_uint_8_list,
) {
    wire_main_set_socks_impl(port_, proxy, username, password)
}

#[no_mangle]
pub extern "C" fn wire_main_get_proxy_status(port_: i64) {
    wire_main_get_proxy_status_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_socks(port_: i64) {
    wire_main_get_socks_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_app_name(port_: i64) {
    wire_main_get_app_name_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_app_name_sync() -> support::WireSyncReturn {
    wire_main_get_app_name_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_uri_prefix_sync() -> support::WireSyncReturn {
    wire_main_uri_prefix_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_get_license(port_: i64) {
    wire_main_get_license_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_version(port_: i64) {
    wire_main_get_version_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_fav(port_: i64) {
    wire_main_get_fav_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_store_fav(port_: i64, favs: *mut wire_StringList) {
    wire_main_store_fav_impl(port_, favs)
}

#[no_mangle]
pub extern "C" fn wire_main_get_peer_sync(id: *mut wire_uint_8_list) -> support::WireSyncReturn {
    wire_main_get_peer_sync_impl(id)
}

#[no_mangle]
pub extern "C" fn wire_main_get_lan_peers(port_: i64) {
    wire_main_get_lan_peers_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_connect_status(port_: i64) {
    wire_main_get_connect_status_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_check_connect_status(port_: i64) {
    wire_main_check_connect_status_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_is_using_public_server(port_: i64) {
    wire_main_is_using_public_server_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_discover(port_: i64) {
    wire_main_discover_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_api_server(port_: i64) {
    wire_main_get_api_server_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_http_request(
    port_: i64,
    url: *mut wire_uint_8_list,
    method: *mut wire_uint_8_list,
    body: *mut wire_uint_8_list,
    header: *mut wire_uint_8_list,
) {
    wire_main_http_request_impl(port_, url, method, body, header)
}

#[no_mangle]
pub extern "C" fn wire_main_get_local_option(
    key: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_main_get_local_option_impl(key)
}

#[no_mangle]
pub extern "C" fn wire_main_get_use_texture_render() -> support::WireSyncReturn {
    wire_main_get_use_texture_render_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_get_env(key: *mut wire_uint_8_list) -> support::WireSyncReturn {
    wire_main_get_env_impl(key)
}

#[no_mangle]
pub extern "C" fn wire_main_set_local_option(
    port_: i64,
    key: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) {
    wire_main_set_local_option_impl(port_, key, value)
}

#[no_mangle]
pub extern "C" fn wire_main_handle_wayland_screencast_restore_token(
    port_: i64,
    _key: *mut wire_uint_8_list,
    _value: *mut wire_uint_8_list,
) {
    wire_main_handle_wayland_screencast_restore_token_impl(port_, _key, _value)
}

#[no_mangle]
pub extern "C" fn wire_main_get_input_source() -> support::WireSyncReturn {
    wire_main_get_input_source_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_set_input_source(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) {
    wire_main_set_input_source_impl(port_, session_id, value)
}

#[no_mangle]
pub extern "C" fn wire_main_get_my_id(port_: i64) {
    wire_main_get_my_id_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_uuid(port_: i64) {
    wire_main_get_uuid_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_peer_option(
    port_: i64,
    id: *mut wire_uint_8_list,
    key: *mut wire_uint_8_list,
) {
    wire_main_get_peer_option_impl(port_, id, key)
}

#[no_mangle]
pub extern "C" fn wire_main_get_peer_option_sync(
    id: *mut wire_uint_8_list,
    key: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_main_get_peer_option_sync_impl(id, key)
}

#[no_mangle]
pub extern "C" fn wire_main_get_peer_flutter_option_sync(
    id: *mut wire_uint_8_list,
    k: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_main_get_peer_flutter_option_sync_impl(id, k)
}

#[no_mangle]
pub extern "C" fn wire_main_set_peer_flutter_option_sync(
    id: *mut wire_uint_8_list,
    k: *mut wire_uint_8_list,
    v: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_main_set_peer_flutter_option_sync_impl(id, k, v)
}

#[no_mangle]
pub extern "C" fn wire_main_set_peer_option(
    port_: i64,
    id: *mut wire_uint_8_list,
    key: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) {
    wire_main_set_peer_option_impl(port_, id, key, value)
}

#[no_mangle]
pub extern "C" fn wire_main_set_peer_option_sync(
    id: *mut wire_uint_8_list,
    key: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_main_set_peer_option_sync_impl(id, key, value)
}

#[no_mangle]
pub extern "C" fn wire_main_set_peer_alias(
    port_: i64,
    id: *mut wire_uint_8_list,
    alias: *mut wire_uint_8_list,
) {
    wire_main_set_peer_alias_impl(port_, id, alias)
}

#[no_mangle]
pub extern "C" fn wire_main_get_new_stored_peers(port_: i64) {
    wire_main_get_new_stored_peers_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_forget_password(port_: i64, id: *mut wire_uint_8_list) {
    wire_main_forget_password_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_main_peer_has_password(port_: i64, id: *mut wire_uint_8_list) {
    wire_main_peer_has_password_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_main_peer_exists(port_: i64, id: *mut wire_uint_8_list) {
    wire_main_peer_exists_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_main_load_recent_peers(port_: i64) {
    wire_main_load_recent_peers_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_load_recent_peers_sync() -> support::WireSyncReturn {
    wire_main_load_recent_peers_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_load_lan_peers_sync() -> support::WireSyncReturn {
    wire_main_load_lan_peers_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_load_recent_peers_for_ab(port_: i64, filter: *mut wire_uint_8_list) {
    wire_main_load_recent_peers_for_ab_impl(port_, filter)
}

#[no_mangle]
pub extern "C" fn wire_main_load_fav_peers(port_: i64) {
    wire_main_load_fav_peers_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_load_lan_peers(port_: i64) {
    wire_main_load_lan_peers_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_remove_discovered(port_: i64, id: *mut wire_uint_8_list) {
    wire_main_remove_discovered_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_main_change_theme(port_: i64, dark: *mut wire_uint_8_list) {
    wire_main_change_theme_impl(port_, dark)
}

#[no_mangle]
pub extern "C" fn wire_main_change_language(port_: i64, lang: *mut wire_uint_8_list) {
    wire_main_change_language_impl(port_, lang)
}

#[no_mangle]
pub extern "C" fn wire_main_video_save_directory(root: bool) -> support::WireSyncReturn {
    wire_main_video_save_directory_impl(root)
}

#[no_mangle]
pub extern "C" fn wire_main_set_user_default_option(
    port_: i64,
    key: *mut wire_uint_8_list,
    value: *mut wire_uint_8_list,
) {
    wire_main_set_user_default_option_impl(port_, key, value)
}

#[no_mangle]
pub extern "C" fn wire_main_get_user_default_option(
    key: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_main_get_user_default_option_impl(key)
}

#[no_mangle]
pub extern "C" fn wire_main_handle_relay_id(port_: i64, id: *mut wire_uint_8_list) {
    wire_main_handle_relay_id_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_main_is_option_fixed(key: *mut wire_uint_8_list) -> support::WireSyncReturn {
    wire_main_is_option_fixed_impl(key)
}

#[no_mangle]
pub extern "C" fn wire_main_get_main_display() -> support::WireSyncReturn {
    wire_main_get_main_display_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_get_displays() -> support::WireSyncReturn {
    wire_main_get_displays_impl()
}

#[no_mangle]
pub extern "C" fn wire_session_add_port_forward(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    local_port: i32,
    remote_host: *mut wire_uint_8_list,
    remote_port: i32,
) {
    wire_session_add_port_forward_impl(port_, session_id, local_port, remote_host, remote_port)
}

#[no_mangle]
pub extern "C" fn wire_session_remove_port_forward(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    local_port: i32,
) {
    wire_session_remove_port_forward_impl(port_, session_id, local_port)
}

#[no_mangle]
pub extern "C" fn wire_session_new_rdp(port_: i64, session_id: *mut wire_uint_8_list) {
    wire_session_new_rdp_impl(port_, session_id)
}

#[no_mangle]
pub extern "C" fn wire_session_request_voice_call(port_: i64, session_id: *mut wire_uint_8_list) {
    wire_session_request_voice_call_impl(port_, session_id)
}

#[no_mangle]
pub extern "C" fn wire_session_close_voice_call(port_: i64, session_id: *mut wire_uint_8_list) {
    wire_session_close_voice_call_impl(port_, session_id)
}

#[no_mangle]
pub extern "C" fn wire_session_get_conn_token(
    session_id: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_session_get_conn_token_impl(session_id)
}

#[no_mangle]
pub extern "C" fn wire_cm_handle_incoming_voice_call(port_: i64, id: i32, accept: bool) {
    wire_cm_handle_incoming_voice_call_impl(port_, id, accept)
}

#[no_mangle]
pub extern "C" fn wire_cm_close_voice_call(port_: i64, id: i32) {
    wire_cm_close_voice_call_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_set_voice_call_input_device(
    port_: i64,
    _is_cm: bool,
    _device: *mut wire_uint_8_list,
) {
    wire_set_voice_call_input_device_impl(port_, _is_cm, _device)
}

#[no_mangle]
pub extern "C" fn wire_get_voice_call_input_device(port_: i64, _is_cm: bool) {
    wire_get_voice_call_input_device_impl(port_, _is_cm)
}

#[no_mangle]
pub extern "C" fn wire_main_get_last_remote_id(port_: i64) {
    wire_main_get_last_remote_id_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_software_update_url(port_: i64) {
    wire_main_get_software_update_url_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_home_dir(port_: i64) {
    wire_main_get_home_dir_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_langs(port_: i64) {
    wire_main_get_langs_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_temporary_password(port_: i64) {
    wire_main_get_temporary_password_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_permanent_password(port_: i64) {
    wire_main_get_permanent_password_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_fingerprint(port_: i64) {
    wire_main_get_fingerprint_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_cm_get_clients_state(port_: i64) {
    wire_cm_get_clients_state_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_cm_check_clients_length(port_: i64, length: usize) {
    wire_cm_check_clients_length_impl(port_, length)
}

#[no_mangle]
pub extern "C" fn wire_cm_get_clients_length(port_: i64) {
    wire_cm_get_clients_length_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_init(
    port_: i64,
    app_dir: *mut wire_uint_8_list,
    custom_client_config: *mut wire_uint_8_list,
) {
    wire_main_init_impl(port_, app_dir, custom_client_config)
}

#[no_mangle]
pub extern "C" fn wire_main_device_id(port_: i64, id: *mut wire_uint_8_list) {
    wire_main_device_id_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_main_device_name(port_: i64, name: *mut wire_uint_8_list) {
    wire_main_device_name_impl(port_, name)
}

#[no_mangle]
pub extern "C" fn wire_main_remove_peer(port_: i64, id: *mut wire_uint_8_list) {
    wire_main_remove_peer_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_main_has_hwcodec() -> support::WireSyncReturn {
    wire_main_has_hwcodec_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_has_vram() -> support::WireSyncReturn {
    wire_main_has_vram_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_supported_hwdecodings() -> support::WireSyncReturn {
    wire_main_supported_hwdecodings_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_is_root(port_: i64) {
    wire_main_is_root_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_double_click_time() -> support::WireSyncReturn {
    wire_get_double_click_time_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_start_dbus_server(port_: i64) {
    wire_main_start_dbus_server_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_save_ab(port_: i64, json: *mut wire_uint_8_list) {
    wire_main_save_ab_impl(port_, json)
}

#[no_mangle]
pub extern "C" fn wire_main_clear_ab(port_: i64) {
    wire_main_clear_ab_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_load_ab(port_: i64) {
    wire_main_load_ab_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_save_group(port_: i64, json: *mut wire_uint_8_list) {
    wire_main_save_group_impl(port_, json)
}

#[no_mangle]
pub extern "C" fn wire_main_clear_group(port_: i64) {
    wire_main_clear_group_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_load_group(port_: i64) {
    wire_main_load_group_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_session_send_pointer(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    msg: *mut wire_uint_8_list,
) {
    wire_session_send_pointer_impl(port_, session_id, msg)
}

#[no_mangle]
pub extern "C" fn wire_session_send_mouse(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    msg: *mut wire_uint_8_list,
) {
    wire_session_send_mouse_impl(port_, session_id, msg)
}

#[no_mangle]
pub extern "C" fn wire_session_restart_remote_device(
    port_: i64,
    session_id: *mut wire_uint_8_list,
) {
    wire_session_restart_remote_device_impl(port_, session_id)
}

#[no_mangle]
pub extern "C" fn wire_session_get_audit_server_sync(
    session_id: *mut wire_uint_8_list,
    typ: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_session_get_audit_server_sync_impl(session_id, typ)
}

#[no_mangle]
pub extern "C" fn wire_session_send_note(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    note: *mut wire_uint_8_list,
) {
    wire_session_send_note_impl(port_, session_id, note)
}

#[no_mangle]
pub extern "C" fn wire_session_alternative_codecs(port_: i64, session_id: *mut wire_uint_8_list) {
    wire_session_alternative_codecs_impl(port_, session_id)
}

#[no_mangle]
pub extern "C" fn wire_session_change_prefer_codec(port_: i64, session_id: *mut wire_uint_8_list) {
    wire_session_change_prefer_codec_impl(port_, session_id)
}

#[no_mangle]
pub extern "C" fn wire_session_on_waiting_for_image_dialog_show(
    port_: i64,
    session_id: *mut wire_uint_8_list,
) {
    wire_session_on_waiting_for_image_dialog_show_impl(port_, session_id)
}

#[no_mangle]
pub extern "C" fn wire_session_toggle_virtual_display(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    index: i32,
    on: bool,
) {
    wire_session_toggle_virtual_display_impl(port_, session_id, index, on)
}

#[no_mangle]
pub extern "C" fn wire_main_set_home_dir(port_: i64, _home: *mut wire_uint_8_list) {
    wire_main_set_home_dir_impl(port_, _home)
}

#[no_mangle]
pub extern "C" fn wire_main_get_data_dir_ios() -> support::WireSyncReturn {
    wire_main_get_data_dir_ios_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_stop_service(port_: i64) {
    wire_main_stop_service_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_start_service(port_: i64) {
    wire_main_start_service_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_update_temporary_password(port_: i64) {
    wire_main_update_temporary_password_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_set_permanent_password(port_: i64, password: *mut wire_uint_8_list) {
    wire_main_set_permanent_password_impl(port_, password)
}

#[no_mangle]
pub extern "C" fn wire_main_check_super_user_permission(port_: i64) {
    wire_main_check_super_user_permission_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_unlock_pin() -> support::WireSyncReturn {
    wire_main_get_unlock_pin_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_set_unlock_pin(pin: *mut wire_uint_8_list) -> support::WireSyncReturn {
    wire_main_set_unlock_pin_impl(pin)
}

#[no_mangle]
pub extern "C" fn wire_main_check_mouse_time(port_: i64) {
    wire_main_check_mouse_time_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_mouse_time(port_: i64) {
    wire_main_get_mouse_time_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_wol(port_: i64, id: *mut wire_uint_8_list) {
    wire_main_wol_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_main_create_shortcut(port_: i64, _id: *mut wire_uint_8_list) {
    wire_main_create_shortcut_impl(port_, _id)
}

#[no_mangle]
pub extern "C" fn wire_cm_send_chat(port_: i64, conn_id: i32, msg: *mut wire_uint_8_list) {
    wire_cm_send_chat_impl(port_, conn_id, msg)
}

#[no_mangle]
pub extern "C" fn wire_cm_login_res(port_: i64, conn_id: i32, res: bool) {
    wire_cm_login_res_impl(port_, conn_id, res)
}

#[no_mangle]
pub extern "C" fn wire_cm_close_connection(port_: i64, conn_id: i32) {
    wire_cm_close_connection_impl(port_, conn_id)
}

#[no_mangle]
pub extern "C" fn wire_cm_remove_disconnected_connection(port_: i64, conn_id: i32) {
    wire_cm_remove_disconnected_connection_impl(port_, conn_id)
}

#[no_mangle]
pub extern "C" fn wire_cm_check_click_time(port_: i64, conn_id: i32) {
    wire_cm_check_click_time_impl(port_, conn_id)
}

#[no_mangle]
pub extern "C" fn wire_cm_get_click_time(port_: i64) {
    wire_cm_get_click_time_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_cm_switch_permission(
    port_: i64,
    conn_id: i32,
    name: *mut wire_uint_8_list,
    enabled: bool,
) {
    wire_cm_switch_permission_impl(port_, conn_id, name, enabled)
}

#[no_mangle]
pub extern "C" fn wire_cm_can_elevate() -> support::WireSyncReturn {
    wire_cm_can_elevate_impl()
}

#[no_mangle]
pub extern "C" fn wire_cm_elevate_portable(port_: i64, conn_id: i32) {
    wire_cm_elevate_portable_impl(port_, conn_id)
}

#[no_mangle]
pub extern "C" fn wire_cm_switch_back(port_: i64, conn_id: i32) {
    wire_cm_switch_back_impl(port_, conn_id)
}

#[no_mangle]
pub extern "C" fn wire_cm_get_config(port_: i64, name: *mut wire_uint_8_list) {
    wire_cm_get_config_impl(port_, name)
}

#[no_mangle]
pub extern "C" fn wire_main_get_build_date(port_: i64) {
    wire_main_get_build_date_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_translate(
    name: *mut wire_uint_8_list,
    locale: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_translate_impl(name, locale)
}

#[no_mangle]
pub extern "C" fn wire_session_get_rgba_size(
    session_id: *mut wire_uint_8_list,
    display: usize,
) -> support::WireSyncReturn {
    wire_session_get_rgba_size_impl(session_id, display)
}

#[no_mangle]
pub extern "C" fn wire_session_next_rgba(
    session_id: *mut wire_uint_8_list,
    display: usize,
) -> support::WireSyncReturn {
    wire_session_next_rgba_impl(session_id, display)
}

#[no_mangle]
pub extern "C" fn wire_session_register_pixelbuffer_texture(
    session_id: *mut wire_uint_8_list,
    display: usize,
    ptr: usize,
) -> support::WireSyncReturn {
    wire_session_register_pixelbuffer_texture_impl(session_id, display, ptr)
}

#[no_mangle]
pub extern "C" fn wire_session_register_gpu_texture(
    session_id: *mut wire_uint_8_list,
    display: usize,
    ptr: usize,
) -> support::WireSyncReturn {
    wire_session_register_gpu_texture_impl(session_id, display, ptr)
}

#[no_mangle]
pub extern "C" fn wire_query_onlines(port_: i64, ids: *mut wire_StringList) {
    wire_query_onlines_impl(port_, ids)
}

#[no_mangle]
pub extern "C" fn wire_version_to_number(v: *mut wire_uint_8_list) -> support::WireSyncReturn {
    wire_version_to_number_impl(v)
}

#[no_mangle]
pub extern "C" fn wire_option_synced(port_: i64) {
    wire_option_synced_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_is_installed() -> support::WireSyncReturn {
    wire_main_is_installed_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_init_input_source() -> support::WireSyncReturn {
    wire_main_init_input_source_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_is_installed_lower_version() -> support::WireSyncReturn {
    wire_main_is_installed_lower_version_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_is_installed_daemon(prompt: bool) -> support::WireSyncReturn {
    wire_main_is_installed_daemon_impl(prompt)
}

#[no_mangle]
pub extern "C" fn wire_main_is_process_trusted(prompt: bool) -> support::WireSyncReturn {
    wire_main_is_process_trusted_impl(prompt)
}

#[no_mangle]
pub extern "C" fn wire_main_is_can_screen_recording(prompt: bool) -> support::WireSyncReturn {
    wire_main_is_can_screen_recording_impl(prompt)
}

#[no_mangle]
pub extern "C" fn wire_main_is_can_input_monitoring(prompt: bool) -> support::WireSyncReturn {
    wire_main_is_can_input_monitoring_impl(prompt)
}

#[no_mangle]
pub extern "C" fn wire_main_is_share_rdp() -> support::WireSyncReturn {
    wire_main_is_share_rdp_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_set_share_rdp(port_: i64, enable: bool) {
    wire_main_set_share_rdp_impl(port_, enable)
}

#[no_mangle]
pub extern "C" fn wire_main_goto_install() -> support::WireSyncReturn {
    wire_main_goto_install_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_get_new_version() -> support::WireSyncReturn {
    wire_main_get_new_version_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_update_me() -> support::WireSyncReturn {
    wire_main_update_me_impl()
}

#[no_mangle]
pub extern "C" fn wire_set_cur_session_id(port_: i64, session_id: *mut wire_uint_8_list) {
    wire_set_cur_session_id_impl(port_, session_id)
}

#[no_mangle]
pub extern "C" fn wire_install_show_run_without_install() -> support::WireSyncReturn {
    wire_install_show_run_without_install_impl()
}

#[no_mangle]
pub extern "C" fn wire_install_run_without_install(port_: i64) {
    wire_install_run_without_install_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_install_install_me(
    port_: i64,
    options: *mut wire_uint_8_list,
    path: *mut wire_uint_8_list,
) {
    wire_install_install_me_impl(port_, options, path)
}

#[no_mangle]
pub extern "C" fn wire_install_install_path() -> support::WireSyncReturn {
    wire_install_install_path_impl()
}

#[no_mangle]
pub extern "C" fn wire_install_install_options() -> support::WireSyncReturn {
    wire_install_install_options_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_account_auth(port_: i64, op: *mut wire_uint_8_list, remember_me: bool) {
    wire_main_account_auth_impl(port_, op, remember_me)
}

#[no_mangle]
pub extern "C" fn wire_main_account_auth_cancel(port_: i64) {
    wire_main_account_auth_cancel_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_account_auth_result(port_: i64) {
    wire_main_account_auth_result_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_on_main_window_close(port_: i64) {
    wire_main_on_main_window_close_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_current_is_wayland() -> support::WireSyncReturn {
    wire_main_current_is_wayland_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_is_login_wayland() -> support::WireSyncReturn {
    wire_main_is_login_wayland_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_hide_dock() -> support::WireSyncReturn {
    wire_main_hide_dock_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_has_file_clipboard() -> support::WireSyncReturn {
    wire_main_has_file_clipboard_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_has_gpu_texture_render() -> support::WireSyncReturn {
    wire_main_has_gpu_texture_render_impl()
}

#[no_mangle]
pub extern "C" fn wire_cm_init(port_: i64) {
    wire_cm_init_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_start_ipc_url_server(port_: i64) {
    wire_main_start_ipc_url_server_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_test_wallpaper(port_: i64, _second: u64) {
    wire_main_test_wallpaper_impl(port_, _second)
}

#[no_mangle]
pub extern "C" fn wire_main_support_remove_wallpaper(port_: i64) {
    wire_main_support_remove_wallpaper_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_is_incoming_only() -> support::WireSyncReturn {
    wire_is_incoming_only_impl()
}

#[no_mangle]
pub extern "C" fn wire_is_outgoing_only() -> support::WireSyncReturn {
    wire_is_outgoing_only_impl()
}

#[no_mangle]
pub extern "C" fn wire_is_custom_client() -> support::WireSyncReturn {
    wire_is_custom_client_impl()
}

#[no_mangle]
pub extern "C" fn wire_is_disable_settings() -> support::WireSyncReturn {
    wire_is_disable_settings_impl()
}

#[no_mangle]
pub extern "C" fn wire_is_disable_ab() -> support::WireSyncReturn {
    wire_is_disable_ab_impl()
}

#[no_mangle]
pub extern "C" fn wire_is_disable_account() -> support::WireSyncReturn {
    wire_is_disable_account_impl()
}

#[no_mangle]
pub extern "C" fn wire_is_disable_group_panel() -> support::WireSyncReturn {
    wire_is_disable_group_panel_impl()
}

#[no_mangle]
pub extern "C" fn wire_is_disable_installation() -> support::WireSyncReturn {
    wire_is_disable_installation_impl()
}

#[no_mangle]
pub extern "C" fn wire_is_preset_password(port_: i64) {
    wire_is_preset_password_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_is_preset_password_mobile_only() -> support::WireSyncReturn {
    wire_is_preset_password_mobile_only_impl()
}

#[no_mangle]
pub extern "C" fn wire_send_url_scheme(port_: i64, _url: *mut wire_uint_8_list) {
    wire_send_url_scheme_impl(port_, _url)
}

#[no_mangle]
pub extern "C" fn wire_plugin_event(
    port_: i64,
    _id: *mut wire_uint_8_list,
    _peer: *mut wire_uint_8_list,
    _event: *mut wire_uint_8_list,
) {
    wire_plugin_event_impl(port_, _id, _peer, _event)
}

#[no_mangle]
pub extern "C" fn wire_plugin_register_event_stream(port_: i64, _id: *mut wire_uint_8_list) {
    wire_plugin_register_event_stream_impl(port_, _id)
}

#[no_mangle]
pub extern "C" fn wire_plugin_get_session_option(
    _id: *mut wire_uint_8_list,
    _peer: *mut wire_uint_8_list,
    _key: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_plugin_get_session_option_impl(_id, _peer, _key)
}

#[no_mangle]
pub extern "C" fn wire_plugin_set_session_option(
    port_: i64,
    _id: *mut wire_uint_8_list,
    _peer: *mut wire_uint_8_list,
    _key: *mut wire_uint_8_list,
    _value: *mut wire_uint_8_list,
) {
    wire_plugin_set_session_option_impl(port_, _id, _peer, _key, _value)
}

#[no_mangle]
pub extern "C" fn wire_plugin_get_shared_option(
    _id: *mut wire_uint_8_list,
    _key: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_plugin_get_shared_option_impl(_id, _key)
}

#[no_mangle]
pub extern "C" fn wire_plugin_set_shared_option(
    port_: i64,
    _id: *mut wire_uint_8_list,
    _key: *mut wire_uint_8_list,
    _value: *mut wire_uint_8_list,
) {
    wire_plugin_set_shared_option_impl(port_, _id, _key, _value)
}

#[no_mangle]
pub extern "C" fn wire_plugin_reload(port_: i64, _id: *mut wire_uint_8_list) {
    wire_plugin_reload_impl(port_, _id)
}

#[no_mangle]
pub extern "C" fn wire_plugin_enable(
    _id: *mut wire_uint_8_list,
    _v: bool,
) -> support::WireSyncReturn {
    wire_plugin_enable_impl(_id, _v)
}

#[no_mangle]
pub extern "C" fn wire_plugin_is_enabled(_id: *mut wire_uint_8_list) -> support::WireSyncReturn {
    wire_plugin_is_enabled_impl(_id)
}

#[no_mangle]
pub extern "C" fn wire_plugin_feature_is_enabled() -> support::WireSyncReturn {
    wire_plugin_feature_is_enabled_impl()
}

#[no_mangle]
pub extern "C" fn wire_plugin_sync_ui(port_: i64, _sync_to: *mut wire_uint_8_list) {
    wire_plugin_sync_ui_impl(port_, _sync_to)
}

#[no_mangle]
pub extern "C" fn wire_plugin_list_reload(port_: i64) {
    wire_plugin_list_reload_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_plugin_install(port_: i64, _id: *mut wire_uint_8_list, _b: bool) {
    wire_plugin_install_impl(port_, _id, _b)
}

#[no_mangle]
pub extern "C" fn wire_is_support_multi_ui_session(
    version: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_is_support_multi_ui_session_impl(version)
}

#[no_mangle]
pub extern "C" fn wire_is_selinux_enforcing() -> support::WireSyncReturn {
    wire_is_selinux_enforcing_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_default_privacy_mode_impl() -> support::WireSyncReturn {
    wire_main_default_privacy_mode_impl_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_supported_privacy_mode_impls() -> support::WireSyncReturn {
    wire_main_supported_privacy_mode_impls_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_supported_input_source() -> support::WireSyncReturn {
    wire_main_supported_input_source_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_generate2fa(port_: i64) {
    wire_main_generate2fa_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_verify2fa(port_: i64, code: *mut wire_uint_8_list) {
    wire_main_verify2fa_impl(port_, code)
}

#[no_mangle]
pub extern "C" fn wire_main_has_valid_2fa_sync() -> support::WireSyncReturn {
    wire_main_has_valid_2fa_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_verify_bot(port_: i64, token: *mut wire_uint_8_list) {
    wire_main_verify_bot_impl(port_, token)
}

#[no_mangle]
pub extern "C" fn wire_main_has_valid_bot_sync() -> support::WireSyncReturn {
    wire_main_has_valid_bot_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_main_get_hard_option(key: *mut wire_uint_8_list) -> support::WireSyncReturn {
    wire_main_get_hard_option_impl(key)
}

#[no_mangle]
pub extern "C" fn wire_main_get_buildin_option(
    key: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_main_get_buildin_option_impl(key)
}

#[no_mangle]
pub extern "C" fn wire_main_check_hwcodec(port_: i64) {
    wire_main_check_hwcodec_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_get_trusted_devices(port_: i64) {
    wire_main_get_trusted_devices_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_remove_trusted_devices(port_: i64, json: *mut wire_uint_8_list) {
    wire_main_remove_trusted_devices_impl(port_, json)
}

#[no_mangle]
pub extern "C" fn wire_main_clear_trusted_devices(port_: i64) {
    wire_main_clear_trusted_devices_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_main_max_encrypt_len() -> support::WireSyncReturn {
    wire_main_max_encrypt_len_impl()
}

#[no_mangle]
pub extern "C" fn wire_session_request_new_display_init_msgs(
    port_: i64,
    session_id: *mut wire_uint_8_list,
    display: usize,
) {
    wire_session_request_new_display_init_msgs_impl(port_, session_id, display)
}

#[no_mangle]
pub extern "C" fn wire_main_audio_support_loopback() -> support::WireSyncReturn {
    wire_main_audio_support_loopback_impl()
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_StringList_0(len: i32) -> *mut wire_StringList {
    let wrap = wire_StringList {
        ptr: support::new_leak_vec_ptr(<*mut wire_uint_8_list>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_int_32_list_0(len: i32) -> *mut wire_int_32_list {
    let ans = wire_int_32_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}
impl Wire2Api<Vec<String>> for *mut wire_StringList {
    fn wire2api(self) -> Vec<String> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<uuid::Uuid> for *mut wire_uint_8_list {
    fn wire2api(self) -> uuid::Uuid {
        let single: Vec<u8> = self.wire2api();
        wire2api_uuid_ref(single.as_slice())
    }
}

impl Wire2Api<Vec<i32>> for *mut wire_int_32_list {
    fn wire2api(self) -> Vec<i32> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_StringList {
    ptr: *mut *mut wire_uint_8_list,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_int_32_list {
    ptr: *mut i32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
