use crate::GuiData;
use crate::help_functions::get_notebook_enum_from_tree_view;
use crate::notebook_enums::to_notebook_main_enum;
use crate::notebook_info::NOTEBOOKS_INFO;

pub(crate) fn validate_notebook_data(gui_data: &GuiData) {
    // Test treeviews names, each treeview should have set name same as variable name

    for item in &gui_data.main_notebook.get_main_tree_views() {
        // println!("Checking {} element", i);

        get_notebook_enum_from_tree_view(item);
    }

    // This test main info about notebooks
    // Should have same order as notebook enum types
    for (i, item) in NOTEBOOKS_INFO.iter().enumerate() {
        let en = to_notebook_main_enum(i as u32);
        assert_eq!(item.notebook_type, en);
    }

    // Tests if data returned from array get_notebook_enum_from_tree_view are in right
    for (i, item) in gui_data.main_notebook.get_main_tree_views().iter().enumerate() {
        let nb_en = get_notebook_enum_from_tree_view(item);
        assert_eq!(to_notebook_main_enum(i as u32), nb_en);
    }
}
