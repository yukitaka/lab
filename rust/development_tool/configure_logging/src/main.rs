mod custom_environment_logging;
mod log4_location;
mod per_module;
mod timestamp_in_log;

fn main() {
    log4_location::log_messages_to_a_custom_location();
    timestamp_in_log::include_timestamp_in_log_messages();
    custom_environment_logging::use_a_custom_environment_variable_to_set_up_logging();
    per_module::enable_log_levels_per_module();
}
