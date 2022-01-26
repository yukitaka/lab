mod custom_environment_logging;
mod per_module;
mod timestamp_in_log;

fn main() {
    timestamp_in_log::include_timestamp_in_log_messages();
    custom_environment_logging::use_a_custom_environment_variable_to_set_up_logging();
    per_module::enable_log_levels_per_module();
}
