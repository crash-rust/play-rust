mod normal_struct;
mod tuple_struct;
mod unit_like_struct;

fn main() {
    normal_struct::use_struct();
    tuple_struct::use_tuple_struct();
    unit_like_struct::use_unit_like_struct();
}
