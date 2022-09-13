#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,              // there are a lot, and can't be automatically fixed
    clippy::assign_op_pattern,
    clippy::collapsible_else_if,
    clippy::collapsible_if,
    clippy::comparison_chain,
    clippy::derive_partial_eq_without_eq,
    clippy::expect_fun_call,         // meh
    clippy::identity_op,             // actually more readable
    clippy::int_plus_one,
    clippy::len_zero,
    clippy::manual_range_contains,
    clippy::missing_safety_doc,
    clippy::new_without_default,
    clippy::nonminimal_bool,         // actually more readable
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::wildcard_in_or_patterns, // actually more readable
)]

mod active_obj;
mod cat_play;
pub mod catacomb;
mod catasm;
mod class_type;
mod control_struct;
mod cpanel;
mod cpanel_state;
mod ctl_panel_type;
mod demo_enum;
mod dir_type;
mod exit_type;
mod extra_constants;
mod global_state;
mod gr_type;
mod input_type;
mod obj_def_type;
mod obj_type;
mod objects;
mod pcrlib_a;
mod pcrlib_a_state;
mod pcrlib_c;
mod pcrlib_c_state;
mod pic_file_type;
mod pic_type;
mod rleasm;
mod scan_codes;
mod scores;
mod sdl_manager;
mod sound_type;
mod spkr_table;
mod spksndtype;
mod sprite_type;
mod state_type;
mod tag_type;
mod think_type;
mod vec2;

pub fn main() {
    catacomb::original_main();
}
