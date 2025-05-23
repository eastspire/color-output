use crate::text::r#struct::Text;
use crate::*;
use task::r#struct::Task;

/// Output list
///
/// [Official Documentation](https://docs.ltpp.vip/COLOR-OUTPUT/),
///
/// # Parameters
/// - `Vec<Output>`: The output struct
///
/// # Code Example
///
/// ## Using the Struct
///
/// ### Using the output Function
///
/// ```rust
/// use color_output::*;
/// ```
///
/// ## Using the Constructor
///
/// ### Using the output Function
///
/// ```rust
/// use color_output::*;
/// ```
pub fn output_list(output_list: &Vec<Output>) {
    let mut task_list: Task<'_> = Task::default();
    for output in output_list {
        // Text
        let text: &str = output.text;
        let color: ColorType = output.color.clone();
        let bg_color: ColorType = output.bg_color.clone();
        let blod: bool = output.blod.clone();
        // endl
        let endl: bool = output.endl;
        // Add text
        task_list.add(Text {
            text,
            color,
            bg_color,
            blod,
            endl,
        });
    }
    // run
    task_list.run_all();
}
