mod arrays;

use crate::arrays::arrays_concept::arrays_declaration as declare_array;
use crate::arrays::arrays_concept::default_arrays as default_array;
use crate::arrays::arrays_concept::array_iterator as iterate_array;

fn main() {
    declare_array();
    default_array();
    iterate_array();
}
